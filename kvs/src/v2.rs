pub(crate) mod entry {
    use crate::Result;
    use byteorder::{ReadBytesExt, WriteBytesExt, BE};
    use std::fmt;

    struct Entry {
        checksum: u32,
        header: Header,
        key: String,
        value: Vec<u8>,
    }

    struct Header {
        state: State,
        key_len: u16,
        value_len: u32,
    }

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum State {
        Active = 1,
        Deleted = 2,
    }

    impl Entry {
        pub(crate) fn new<K: Into<String>>(key: K, value: Vec<u8>) -> Result<Self> {
            let key = key.into();
            let mut e = Entry {
                checksum: 0,
                header: Header {
                    state: State::Active,
                    key_len: key.len() as u16,     // TODO check
                    value_len: value.len() as u32, // TODO check
                },
                key,
                value,
            };
            e.checksum = e.checksum()?;

            Ok(e)
        }

        pub(crate) fn encode<W: WriteBytesExt>(&self, mut w: W) -> Result<usize> {
            w.write_u32::<BE>(self.checksum)?;
            w.write_u8(self.header.state as u8)?;
            w.write_u16::<BE>(self.header.key_len)?;
            w.write_u32::<BE>(self.header.value_len)?;

            let mut n: usize = 4 + Header::LEN;
            n += w.write(self.key.as_bytes())?;
            n += w.write(self.value.as_slice())?;

            Ok(n)
        }

        pub(crate) fn len(&self) -> usize {
            4 + Header::LEN + self.key.len() + self.value.len()
        }

        fn checksum(&self) -> Result<u32> {
            let mut h = crc32fast::Hasher::new();
            let mut buff = Vec::with_capacity(Header::LEN);
            buff.write_u8(self.header.state as u8)?;
            buff.write_u16::<BE>(self.header.key_len)?;
            buff.write_u32::<BE>(self.header.value_len)?;
            h.update(&buff);
            h.update(self.key.as_bytes());
            h.update(self.value.as_slice());
            Ok(h.finalize())
        }
    }

    impl fmt::Debug for Entry {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "|crc32: {}|state: {:?}|key_len: {}|value_len: {}|\
                key: {}|value: {}|",
                self.checksum,
                self.header.state,
                self.header.key_len,
                self.header.value_len,
                self.key,
                String::from_utf8_lossy(self.value.as_slice())
            )
        }
    }

    impl Header {
        const LEN: usize = 1 + 2 + 4; // state(1) + key_ley(2) + value_len(4)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use anyhow::Error;
        use std::io::{Cursor, Read, Seek, SeekFrom};
        use std::result::Result as StdResult;

        #[test]
        fn encode() -> StdResult<(), Error> {
            let mut cursor = Cursor::new(Vec::with_capacity(200));
            let entry = Entry::new("1", vec![b'1'])?;

            let encode_bytes = entry.encode(&mut cursor)?;
            assert_eq!(encode_bytes, entry.len());

            // ２つ目から確認する
            entry.encode(&mut cursor)?;
            cursor.seek(SeekFrom::Start(entry.len() as u64))?;

            // skip crc32
            cursor.seek(SeekFrom::Current(4))?;

            // state
            let mut buff = [0_u8; 1];
            cursor.read_exact(&mut buff)?;
            assert_eq!(State::Active as u8, buff[0], "state does not match");

            // key_len
            let mut buff = [0_u8; 2];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[0,1], "key_len does not match");

            // value_len
            let mut buff = [0_u8; 4];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[0,0,0,1], "value_len does not match");

            // key
            let mut buff = [0_u8; 1];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[b'1'], "key does not match");

            // value
            let mut buff = [0_u8;1];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[b'1'], "value does not match");

            Ok(())
        }
    }
}
