use crate::{
    entry::{self, Entry},
    error::KvsError,
    Result,
};
use std::io::BufReader;
use std::{
    collections,
    io::{BufWriter, Read, Seek, SeekFrom::*, Write},
};

pub struct Engine<F> {
    file: F,
    index: entry::KeyIndex,
    position: u64,
    // entryをdecodeする際に利用するBufReaderのbuffer sizeに利用する
    last_entry_len: usize,
}

impl<F> Engine<F>
where
    F: Read + Write + Seek,
{
    pub(crate) fn new(mut file: F) -> Result<Self> {
        let index = entry::KeyIndex::construct_from(&mut file)?;
        let position = file.seek(Current(0))?;
        Ok(Self {
            file,
            index,
            position,
            last_entry_len: 0,
        })
    }

    pub(crate) fn put<K>(&mut self, key: K, value: Vec<u8>) -> Result<()>
    where
        K: Into<String>,
    {
        self.put_entry(Entry::new(key, value)?, true)
    }

    fn put_entry(&mut self, entry: Entry, update_index: bool) -> Result<()> {
        let mut w = BufWriter::with_capacity(entry.len(), &mut self.file);
        let n = entry.encode(&mut w)?;
        w.flush()?;
        debug_assert_eq!(entry.len(), n, "decoded bytes does not match");

        if update_index {
            self.index.0.insert(entry.key, self.position as usize);
        }
        self.position += n as u64;

        Ok(())
    }

    pub(crate) fn get<K>(&mut self, key: K) -> Result<Vec<u8>>
    where
        K: AsRef<str>,
    {
        self.get_entry(key.as_ref()).map(|entry| entry.value)
    }

    fn get_entry(&mut self, key: &str) -> Result<Entry> {
        if let Some(&offset) = self.index.0.get(key) {
            self.file.seek(Start(offset as u64))?;
            let mut r = BufReader::with_capacity(self.last_entry_len, &mut self.file);
            let entry = Entry::decode_with_check(&mut r)?;
            r.seek(Start(self.position))?;
            self.last_entry_len = entry.len();
            Ok(entry)
        } else {
            Err(KvsError::NotFound)
        }
    }

    // If the key exists, it returns the deleted value.
    // Return None if it does not exist.
    pub(crate) fn delete<K>(&mut self, key: K) -> Result<Option<Vec<u8>>>
    where
        K: AsRef<str>,
    {
        self.delete_entry(key.as_ref())
            .map(|opt| opt.map(|entry| entry.value))
    }

    fn delete_entry(&mut self, key: &str) -> Result<Option<Entry>> {
        let entry = match self.get_entry(key.as_ref()) {
            Ok(entry) => entry,
            Err(KvsError::NotFound) => return Ok(None),
            Err(err) => return Err(err),
        };
        // persist
        self.put_entry(entry.mark_delete()?, false)?;

        // remove from index
        self.index.0.remove(key);
        Ok(Some(entry))
    }

    pub(crate) fn keys(&self) -> Keys<'_> {
        Keys {
            inner: self.index.0.iter(),
        }
    }

    #[cfg(test)]
    fn dump(&mut self, buf: &mut [u8]) -> Result<()> {
        self.file.seek(Start(0))?;
        self.file.read_exact(buf)?;
        Ok(())
    }
}

pub struct Keys<'a> {
    inner: collections::hash_map::Iter<'a, String, usize>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

#[cfg(test)]
mod tests {
    use super::entry::*;
    use super::*;
    use anyhow::Error;
    use std::io::{Cursor, Seek};
    use std::result::Result as StdResult;

    type InMemoryKvs = Engine<Cursor<Vec<u8>>>;

    #[test]
    fn put_and_get() -> StdResult<(), Error> {
        let entries = vec![
            Entry::new("1", vec![b'1'])?,
            Entry::new("2", vec![b'2', b'2'])?,
            Entry::new("3", vec![b'3', b'3', b'3'])?,
        ];

        let mut kvs = in_memory_kvs();

        entries.iter().for_each(|entry| {
            kvs.put(&entry.key, entry.value.clone()).unwrap();
        });

        assert_eq!(kvs.get("1")?, vec![b'1']);
        assert_eq!(kvs.get("2")?, vec![b'2', b'2']);
        assert_eq!(kvs.get("3")?, vec![b'3', b'3', b'3']);

        let mut kvs = dump_and_restore(kvs);
        assert_eq!(kvs.get("1")?, vec![b'1']);
        assert_eq!(kvs.get("2")?, vec![b'2', b'2']);
        assert_eq!(kvs.get("3")?, vec![b'3', b'3', b'3']);

        Ok(())
    }

    #[test]
    fn delete() -> StdResult<(), Error> {
        let mut kvs = in_memory_kvs();
        kvs.put("1", vec![b'1'])?;

        assert_eq!(kvs.delete("1").unwrap(), Some(vec![b'1']));
        assert!(kvs.get("1").unwrap_err().is_not_found());
        assert_eq!(kvs.delete("1").unwrap(), None);

        // 削除された状態が維持されるか
        let mut kvs = dump_and_restore(kvs);
        assert!(kvs.get("1").unwrap_err().is_not_found());
        assert_eq!(kvs.delete("1").unwrap(), None);

        Ok(())
    }

    #[test]
    fn keys() -> StdResult<(), Error> {
        let mut kvs = in_memory_kvs();

        kvs.put("1", vec![b'1'])?;
        kvs.put("2", vec![b'2'])?;

        let mut v = Vec::<String>::new();
        kvs.keys().for_each(|key| {
            v.push(key.clone());
        });
        v.sort();
        assert_eq!(v, vec!["1", "2"]);

        Ok(())
    }

    fn in_memory_kvs() -> InMemoryKvs {
        Engine::new(Cursor::new(Vec::new())).unwrap()
    }

    // kvsのfile(buffer)をdumpして再度、kvsを作成しなおす
    // 既存のfileがある状態でのkvsの利用と同じことをやろうとしている
    fn dump_and_restore(mut kvs: InMemoryKvs) -> InMemoryKvs {
        let mut buff = std::iter::repeat(0)
            .take(kvs.position as usize)
            .collect::<Vec<u8>>();
        kvs.dump(&mut buff).unwrap();

        let mut cursor = Cursor::new(buff);
        cursor.seek(Start(0)).unwrap();
        Engine::new(cursor).unwrap()
    }
}
