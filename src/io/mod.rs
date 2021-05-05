/// Basic IO trait for all types of IO to use.
mod io_trait;
/// Memory mapped IO.
mod mmio;

pub use self::io_trait::*;
pub use self::mmio::*;
