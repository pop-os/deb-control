#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate derive_new;

pub mod codec;
pub mod parser;

pub mod prelude {
    pub use crate::{codec::*, parser::*};
}
