pub mod generated;
pub mod lookup;
pub mod mime;

pub use generated::*;
pub use lookup::lookup;
pub use mime::{Mime, TopLevel};

#[cfg(test)]
mod tests;