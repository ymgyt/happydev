use crate::{error::KvsError, Result};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt,
    io::{BufReader, Read},
};

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
            _ => Err(KvsError::InvalidState(n)),
        }
    }
}

#[derive(PartialEq, Clone)]
pub(crate) struct Entry {
    header: Header,
    pub(crate) key: String,
    pub(crate) value: Vec<u8>,
}

#[derive(PartialEq, Clone)]
struct Header {
    checksum: u32,
    state: State,
    key_len: u16,
    value_len: u32,
}

impl Header {
    const LEN: usize = 4 + 1 + 2 + 4; // checksum(4) + state(1) + key_ley(2) + value_len(4)
    const LEN_WITHOUT_CHECKSUM: usize = Header::LEN - 4;
}

impl Entry {
    pub(crate) fn new<K: Into<String>>(key: K, value: Vec<u8>) -> Result<Self> {
        Entry::new_with_state(key.into(), value, State::Active)
    }

    fn new_with_state(key: String, value: Vec<u8>, state: State) -> Result<Self> {
        let mut e = Entry {
            header: Header {
                checksum: 0,
                state,
                key_len: key.len() as u16,     // TODO check
                value_len: value.len() as u32, // TODO check
            },
            key,
            value,
        };
        e.header.checksum = e.calc_checksum()?;

        Ok(e)
    }

    pub(crate) fn mark_delete(&self) -> Result<Self> {
        Entry::new_with_state(self.key.clone(), Vec::new(), State::Deleted)
    }

    pub(crate) fn is_deleted(&self) -> bool {
        self.header.state == State::Deleted
    }

    pub(crate) fn encode<W: WriteBytesExt>(&self, mut w: W) -> Result<usize> {
        w.write_u32::<BE>(self.header.checksum)?;
        w.write_u8(self.header.state as u8)?;
        w.write_u16::<BE>(self.header.key_len)?;
        w.write_u32::<BE>(self.header.value_len)?;

        let mut n: usize = Header::LEN;
        n += w.write(self.key.as_bytes())?;
        n += w.write(self.value.as_slice())?;

        Ok(n)
    }
    pub(crate) fn decode_with_check<R: ReadBytesExt>(r: R) -> Result<Self> {
        Entry::decode(r).and_then(|entry| {
            if entry.header.checksum != entry.calc_checksum()? {
                Err(KvsError::CorruptData)
            } else {
                Ok(entry)
            }
        })
    }

    pub(crate) fn decode<R: ReadBytesExt>(mut r: R) -> Result<Self> {
        let checksum = r.read_u32::<BE>()?;
        let state = State::try_from(r.read_u8()?)?;
        let key_len = r.read_u16::<BE>()?;
        let value_len = r.read_u32::<BE>()?;

        let data_len = key_len as usize + value_len as usize;
        let mut key_value = Vec::with_capacity(data_len);
        r.take(data_len as u64).read_to_end(&mut key_value)?;

        let value = key_value.split_off(key_len as usize);
        let key = String::from_utf8(key_value).map_err(|err| KvsError::from(err.utf8_error()))?;

        Ok(Entry {
            header: Header {
                checksum,
                state,
                key_len,
                value_len,
            },
            key,
            value,
        })
    }

    pub(crate) fn len(&self) -> usize {
        Header::LEN + self.key.len() + self.value.len()
    }

    fn calc_checksum(&self) -> Result<u32> {
        let mut h = crc32fast::Hasher::new();
        let mut buff = Vec::with_capacity(Header::LEN_WITHOUT_CHECKSUM);
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
            self.header.checksum,
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
    pub(crate) fn construct_from<R: Read>(r: R) -> Result<Self> {
        let mut r = BufReader::new(r);
        let mut h = HashMap::new();
        let mut position = 0;
        let err = loop {
            if let Err(err) = Entry::decode(r.by_ref()).map(|entry| {
                let entry_len = entry.len();
                if entry.is_deleted() {
                    // 削除されているentryは明示的にindexから削除しておかないと
                    // 削除前のentryがindexに残ってしまう
                    h.remove(entry.key.as_str());
                } else {
                    h.insert(entry.key, position);
                }
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
    use std::io::{Cursor, Seek, SeekFrom};
    use std::result::Result as StdResult;

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
    fn encode_decode_deleted() -> StdResult<(), KvsError> {
        let mut cursor = Cursor::new(Vec::new());
        let deleted = Entry::new("1", vec![b'1'])?.mark_delete()?;
        deleted.encode(&mut cursor)?;

        // skip crc32
        cursor.seek(SeekFrom::Start(4))?;

        // state
        let mut buff = [0_u8; 1];
        cursor.read_exact(&mut buff)?;
        assert_eq!(State::Deleted as u8, buff[0], "state does not match");

        // key_len
        let mut buff = [0_u8; 2];
        cursor.read_exact(&mut buff)?;
        assert_eq!(&buff, &[0, 1], "key_len does not match");

        // value_len
        let mut buff = [0_u8; 4];
        cursor.read_exact(&mut buff)?;
        assert_eq!(&buff, &[0, 0, 0, 0], "value_len does not match");

        // key
        let mut buff = [0_u8; 1];
        cursor.read_exact(&mut buff)?;
        assert_eq!(&buff, &[b'1'], "key does not match");

        // value is empty

        cursor.seek(SeekFrom::Start(0))?;
        let decoded = Entry::decode(&mut cursor)?;
        assert_eq!(decoded, deleted, "decoded entry does not match");
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
