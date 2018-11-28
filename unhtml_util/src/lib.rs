#![feature(custom_attribute)]
extern crate failure;
#[macro_use]
extern crate failure_derive;

#[cfg(test)]
mod test;
mod err;
mod traits;
mod utils;
pub use self::err::*;
pub use self::utils::*;
pub use self::traits::*;
pub const HTML_IDENT: &str = "html";
pub const SELECTOR_IDENT: &str = "selector";
pub const ATTR_IDENT: &str = "attr";
pub const DEFAULT_IDENT: &str = "default";
pub const EQUAL_PUNCT: char = '=';
pub const ROOT_SELECTOR: &str = "root";
pub const ATTR_INNER_TEXT: &str = "innerText";