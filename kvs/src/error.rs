use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("kind: {:?} {}", .source.kind(), .source)]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("bincode: {}", .source)]
    Serialize {
        #[from]
        source: bincode::Error,
    },
    #[error("max key bytes({}) exceeded", crate::MAX_KEY_BYTES)]
    MaxKeyBytes,
    #[error("max value bytes({}) exceeded", crate::MAX_VALUE_BYTES)]
    MaxValueBytes,
    #[error("invalid entry state {}", .0)]
    InvalidState(u8),
    #[error("not found")]
    NotFound,
    #[error("unknown err")]
    Unknown,
}

impl KvsError {
    pub fn is_eof(&self) -> bool {
        if let KvsError::Io { source } = self {
            source.kind() == io::ErrorKind::UnexpectedEof
        } else {
            false
        }
    }
}
