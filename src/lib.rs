#![forbid(unsafe_code)]

pub mod drawingml;
pub mod error;
pub mod relationship;
pub mod sharedtypes;
pub mod util;
pub mod xml;
pub mod xsdtypes;

extern crate strum;
#[macro_use]
extern crate strum_macros;
