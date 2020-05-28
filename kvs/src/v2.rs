use std::{io::{Read, Write, Seek,SeekFrom::*}, path::Path};
use crate::Result;
use serde::{Serialize};
use crate::error::KvsError;


pub struct Kvs<F> {
    file: F,
    index: entry::KeyIndex,
    position: u64,
}

impl<F> Kvs<F>
    where F: Read + Write + Seek {
    pub fn with_path<P: AsRef<Path>>(_path: P) -> Result<Self> {
        todo!();
    }

    pub fn new(mut file: F) -> Result<Self> {
        let index = entry::KeyIndex::construct_from(&mut file)?;
        let position = file.seek(Current(0))?;
        Ok(Self {
            file,
            index,
            position,
        })
    }

    pub fn pos(&self) -> u64 {
        self.position
    }

    pub fn dump(&mut self, buf: &mut [u8]) -> Result<()> {
        self.file.seek(Start(0))?;
        self.file.read_exact(buf)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.file.flush().map_err(KvsError::from)
    }

    pub fn put_encoded<K>(&mut self, key: K, value: Vec<u8>) -> Result<()>
        where K: Into<String>{
        let entry = entry::Entry::new(key, value)?;
        let n = entry.encode(&mut self.file)?;
        debug_assert_eq!(entry.len(), n, "decoded bytes does not match");

        self.index.0.insert(entry.key, self.position as usize);
        self.position += n as u64;

        Ok(())
    }

    pub fn get_encoded(&mut self, key: &str) -> Result<Vec<u8>> {
        if let Some(&offset) = self.index.0.get(key) {
            self.file.seek(Start(offset as u64))?;
            let entry = entry::Entry::decode(&mut self.file)?;

            self.file.seek(Start(self.position))?;
            Ok(entry.value)
        } else {
            Err(KvsError::NotFound)
        }
    }
}

pub(crate) mod entry {
    use crate::Result;
    use byteorder::{ReadBytesExt, WriteBytesExt, BE};
    use std::{fmt, convert::TryFrom, io::Read, collections::HashMap};
    use crate::error::KvsError;

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum State {
        Active = 1,
        Deleted = 2,
    }

    impl TryFrom<u8> for State {
        type Error = KvsError;

        fn try_from(n: u8) -> std::result::Result<Self, Self::Error> {
            match n {
                1 => Ok(State::Active),
                2 => Ok(State::Deleted),
                _ => Err(KvsError::InvalidState(n))
            }
        }
    }

    #[derive(PartialEq, Clone)]
    pub(crate) struct Entry {
        checksum: u32,
        header: Header,
        pub(crate) key: String,
        pub(crate)value: Vec<u8>,
    }

    #[derive(PartialEq, Clone)]
    struct Header {
        state: State,
        key_len: u16,
        value_len: u32,
    }

    impl Header {
        const LEN: usize = 1 + 2 + 4; // state(1) + key_ley(2) + value_len(4)
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

        pub(crate) fn decode<R: ReadBytesExt>(mut r: R) -> Result<Self> {
            let checksum = r.read_u32::<BE>()?;
            let state = State::try_from(r.read_u8()?)?;
            let key_len = r.read_u16::<BE>()?;
            let value_len = r.read_u32::<BE>()?;

            let mut key = String::with_capacity(key_len as usize);
            r.by_ref().take(key_len as u64).read_to_string(&mut key)?;

            let mut value = Vec::with_capacity(value_len as usize);
            r.by_ref().take(value_len as u64).read_to_end(&mut value)?;

            Ok(Entry {
                checksum,
                header: Header {
                    state,
                    key_len,
                    value_len,
                },
                key,
                value,
            })
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

    #[derive(Debug)]
    pub(crate) struct KeyIndex(pub HashMap<String, usize>);

    impl KeyIndex {
        pub(crate) fn construct_from<R: Read>(mut r: R) -> Result<Self> {
            let mut h = HashMap::new();
            let mut position = 0;
            let err = loop {
                if let Err(err) = Entry::decode(r.by_ref()).map(|entry| {
                    let entry_len = entry.len();
                    h.insert(entry.key, position);
                    position += entry_len;
                }) {
                    break err;
                }
            };
            if err.is_eof() {
                Ok(Self(h))
            } else {
                Err(err)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use anyhow::Error;
        use std::io::{Cursor, Read, Seek, SeekFrom};
        use std::result::Result as StdResult;
        use std::io::SeekFrom::Current;

        #[test]
        fn encode_decode() -> StdResult<(), Error> {
            let mut cursor = Cursor::new(Vec::with_capacity(200));
            let entry = Entry::new("1", vec![b'1'])?;

            let encode_bytes = entry.encode(&mut cursor)?;
            assert_eq!(encode_bytes, entry.len());

            // skip crc32
            cursor.seek(SeekFrom::Start(4))?;

            // state
            let mut buff = [0_u8; 1];
            cursor.read_exact(&mut buff)?;
            assert_eq!(State::Active as u8, buff[0], "state does not match");

            // key_len
            let mut buff = [0_u8; 2];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[0, 1], "key_len does not match");

            // value_len
            let mut buff = [0_u8; 4];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[0, 0, 0, 1], "value_len does not match");

            // key
            let mut buff = [0_u8; 1];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[b'1'], "key does not match");

            // value
            let mut buff = [0_u8; 1];
            cursor.read_exact(&mut buff)?;
            assert_eq!(&buff, &[b'1'], "value does not match");


            cursor.seek(SeekFrom::Start(0))?;
            let decoded = Entry::decode(&mut cursor)?;
            assert_eq!(decoded, entry, "decoded entry does not match");
            assert_eq!(cursor.seek(SeekFrom::Current(0))?, decoded.len() as u64);

            Ok(())
        }

        #[test]
        fn key_index_from() -> StdResult<(), Error> {
            let entries = vec![
                Entry::new("1", vec![b'1'])?,
                Entry::new("2", vec![b'2'])?,
                Entry::new("3", vec![b'3'])?,
            ];

            let mut cursor = Cursor::new(Vec::new());
            entries.iter().for_each(|entry| {
                entry.encode(&mut cursor).unwrap();
            });
            cursor.seek(SeekFrom::Start(0))?;

            let index = KeyIndex::construct_from(&mut cursor)?;
            let mut position: usize = 0;
            for entry in entries {
                let offset = index.0.get(entry.key.as_str()).unwrap();
                assert_eq!(*offset, position);
                position += entry.len();
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::entry::*;
    use anyhow::Error;
    use std::io::{Cursor, Read, Seek, SeekFrom};
    use std::result::Result as StdResult;
    use std::io::SeekFrom::Current;
    use std::fs::File;

    #[test]
    fn put_and_get() -> StdResult<(), Error>{
        let entries = vec![
            Entry::new("1", vec![b'1'])?,
            Entry::new("2", vec![b'2'])?,
            Entry::new("3", vec![b'3'])?,
        ];

        let cursor = Cursor::new(Vec::new());
        let mut kvs = Kvs::new(cursor)?;

        entries.iter().for_each(|entry| {
            kvs.put_encoded(&entry.key, entry.value.clone()).unwrap();
        });

        assert_eq!(kvs.get_encoded("1")?, vec![b'1']);
        assert_eq!(kvs.get_encoded("2")?, vec![b'2']);
        assert_eq!(kvs.get_encoded("3")?, vec![b'3']);

        let mut buff = std::iter::repeat(0).take(kvs.pos() as usize).collect::<Vec<u8>>();
        kvs.dump(&mut buff)?;

        let mut cursor = Cursor::new(buff);
        cursor.seek(Start(0))?;
        let mut kvs = Kvs::new(cursor)?;
        assert_eq!(kvs.get_encoded("1")?, vec![b'1']);
        assert_eq!(kvs.get_encoded("2")?, vec![b'2']);
        assert_eq!(kvs.get_encoded("3")?, vec![b'3']);

        Ok(())
    }

}

