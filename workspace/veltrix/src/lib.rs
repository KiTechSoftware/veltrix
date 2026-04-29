pub mod error;

pub mod paths;
#[cfg(feature = "unistd")]
pub mod unistd;
#[cfg(feature = "emojis")]
pub mod emojis;

pub use error::{Result, VeltrixError as Error};
