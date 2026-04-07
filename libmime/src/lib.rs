pub mod generated;
pub mod lookup;
pub mod mime;

pub use generated::*;
pub use lookup::lookup;
pub use mime::{toplevel::TopLevel, Mime};

#[cfg(test)]
mod tests;
