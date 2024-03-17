#![allow(unused)]

pub use de::from_str;
pub use formatter::{detect_version, X12Formatter, detect_format};
pub(crate) use formatter::Level;
pub use generic::{Element, GenericDocument, Segment};
pub use ser::to_string;

mod formatter;
mod generic;
pub mod de;
pub mod ser;

