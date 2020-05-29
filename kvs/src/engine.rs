use crate::error::KvsError;
use crate::{entry, Result};
use std::{
    fs,
    io::{self, Read, Seek, SeekFrom::*, Write},
    path::Path,
};

pub struct Kvs<F> {
    file: F,
    index: entry::KeyIndex,
    position: u64,
}

impl<F> Kvs<F>
where
    F: Read + Write + Seek,
{
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
    where
        K: Into<String>,
    {
        let entry = entry::Entry::new(key, value)?;
        let n = entry.encode(&mut self.file)?;
        self.file.flush()?; // cleanupまわりに不安があるので毎回flushする
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

impl Kvs<fs::File> {
    pub fn with_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // make sure root directory exists
        if let Some(parent) = path.parent() {
            match fs::create_dir(parent) {
                Ok(_) => (),
                Err(err) => match err.kind() {
                    io::ErrorKind::AlreadyExists => (),
                    _ => return Err(KvsError::from(err)),
                },
            }
        }

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;

        let index = entry::KeyIndex::construct_from(&mut file)?;

        Ok(Kvs {
            file,
            index,
            position: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::entry::*;
    use super::*;
    use anyhow::Error;
    use std::io::{Cursor, Seek};
    use std::result::Result as StdResult;

    #[test]
    fn put_and_get() -> StdResult<(), Error> {
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

        let mut buff = std::iter::repeat(0)
            .take(kvs.pos() as usize)
            .collect::<Vec<u8>>();
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
