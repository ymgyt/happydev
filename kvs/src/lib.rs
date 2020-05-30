mod engine;
mod entry;
mod error;
mod store;

pub use error::KvsError;
pub use store::Kvs;

const MAX_KEY_BYTES: u16 = std::u16::MAX;
const MAX_VALUE_BYTES: u32 = std::u32::MAX;

type Result<T> = std::result::Result<T, error::KvsError>;
