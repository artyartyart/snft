
// #![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod processor;
pub mod instruction;
pub mod error;
pub mod state;


#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
