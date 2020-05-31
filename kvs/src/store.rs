use crate::{engine::Engine, KvsError, Result};
use std::{
    fs::{self, File},
    io,
    path::Path,
};

pub struct Kvs {
    engine: Engine<File>,
}

impl Kvs {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // make sure root directory exists
        // ".data.kvs".parent() return Some("")
        if let Some(parent) = path
            .parent()
            .filter(|&p| !p.to_str().unwrap_or("").is_empty())
        {
            match fs::create_dir(parent) {
                Ok(_) => (),
                Err(err) => match err.kind() {
                    io::ErrorKind::AlreadyExists => (),
                    _ => return Err(err.into()),
                },
            }
        }

        let file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;

        Engine::new(file).map(|engine| Self { engine })
    }

    pub fn put<K, T: ?Sized>(&mut self, key: K, value: &T) -> Result<()>
    where
        K: Into<String>,
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        bincode::serialize(value)
            .map_err(KvsError::from)
            .and_then(|bytes| self.engine.put(key, bytes))
    }

    pub fn get<T>(&mut self, key: &str) -> Result<T>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        self.engine
            .get(key)
            .and_then(|bytes| bincode::deserialize::<T>(bytes.as_slice()).map_err(KvsError::from))
    }

    pub fn delete<T>(&mut self, key: &str) -> Result<Option<T>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        self.engine.delete(key).and_then(|opt| match opt {
            Some(bytes) => Ok(Some(
                bincode::deserialize::<T>(bytes.as_slice()).map_err(KvsError::from)?,
            )),
            None => Ok(None),
        })
    }

    pub fn keys(&self) -> crate::Keys<'_> {
        self.engine.keys()
    }
}
