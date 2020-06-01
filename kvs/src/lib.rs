mod engine;
mod entry;
mod error;
mod protocol;
mod server;
mod store;

pub use engine::Keys;
pub use error::KvsError;
pub use server::Server;
pub use store::Kvs;

const MAX_KEY_BYTES: u16 = std::u16::MAX;
const MAX_VALUE_BYTES: u32 = std::u32::MAX;

type Result<T> = std::result::Result<T, error::KvsError>;
