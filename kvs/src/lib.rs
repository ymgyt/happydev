mod engine;
mod entry;
mod error;

pub use engine::Kvs;
pub use error::KvsError;

const MAX_KEY_BYTES: u16 = std::u16::MAX;
const MAX_VALUE_BYTES: u32 = std::u32::MAX;

type Result<T> = std::result::Result<T, error::KvsError>;
