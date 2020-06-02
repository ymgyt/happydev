use crate::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;

pub(crate) struct Message {
    header: Header,
    encoded_payload: Vec<u8>,
}

impl Message {
    pub(crate) fn from_payload(payload: Payload) -> Result<Self> {
        // TODO: rewrite
        match &payload {
            Payload::EchoRequest { message, .. } => {
                let mut buff = Vec::with_capacity(message.len());
                std::io::Write::write_all(&mut buff, message.as_bytes())?;
                Ok(Self {
                    header: Header {
                        magic_word: 0xFF,
                        payload_kind: payload.kind(),
                        payload_bytes: message.len() as u64,
                    },
                    encoded_payload: buff,
                })
            }
            Payload::EchoResponse { message, .. } => {
                let mut buff = Vec::with_capacity(message.len());
                std::io::Write::write_all(&mut buff, message.as_bytes())?;
                Ok(Self {
                    header: Header {
                        magic_word: 0xFF,
                        payload_kind: payload.kind(),
                        payload_bytes: message.len() as u64,
                    },
                    encoded_payload: buff,
                })
            }
        }
    }
}

pub(crate) struct Header {
    magic_word: u8,
    payload_kind: PayloadKind,
    payload_bytes: u64,
}

#[repr(u8)]
pub(crate) enum PayloadKind {
    EchoRequest = 100,
    EchoResponse = 101,
}

#[derive(Debug)]
pub(crate) enum Payload {
    EchoRequest {
        message: String,
    },
    #[allow(dead_code)]
    EchoResponse {
        message: String,
    },
}

impl Payload {
    pub fn kind(&self) -> PayloadKind {
        match self {
            Payload::EchoRequest { .. } => PayloadKind::EchoRequest,
            Payload::EchoResponse { .. } => PayloadKind::EchoResponse,
        }
    }
}

pub(crate) struct Operator {
    conn: TcpStream,
}

impl Operator {
    pub(crate) fn with_stream(stream: TcpStream) -> Result<Self> {
        Ok(Self { conn: stream })
    }

    pub(crate) async fn send(&mut self, message: Message) -> Result<()> {
        self.conn
            .write_u8(message.header.magic_word.to_be())
            .await?;
        self.conn
            .write_u8((message.header.payload_kind as u8).to_be())
            .await?;
        self.conn
            .write_u64(message.header.payload_bytes.to_be())
            .await?;
        self.conn
            .write_all(message.encoded_payload.as_slice())
            .await?;
        Ok(())
    }

    pub(crate) async fn receive(&mut self) -> Result<Payload> {
        let magic_word = u8::from_be(self.conn.read_u8().await?);
        info!("read magic_word {}", magic_word);

        let payload_kind = u8::from_be(self.conn.read_u8().await?);
        // TODO: try_from定義する
        info!("read payload_kind {}", payload_kind);

        let payload_bytes = u64::from_be(self.conn.read_u64().await?);
        info!("read payload_bytes {}", payload_bytes);

        // Vecでbuffを確保して、conn.take().read_to_end()を行おうとしたが
        // as_ref()が使えずmoveを回避できない
        let mut buff = bytes::BytesMut::with_capacity(payload_bytes as usize);
        self.conn.read_buf(&mut buff).await?;

        let message = String::from_utf8(buff.to_vec()).map_err(anyhow::Error::from)?;

        match payload_kind {
            100 => Ok(Payload::EchoRequest { message }),
            101 => Ok(Payload::EchoResponse { message }),
            _ => panic!("unexpected..."),
        }
    }
}
