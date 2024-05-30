use crate::error::CapyError;
use std::fmt::{self, Display, Formatter};

pub struct Font {}

pub fn parse_from_file(filepath: &str) -> Result<Font, CapyError> {
    let _message = std::fs::read_to_string(filepath)?;
    // FIXME: implement actual font parsing
    Ok(Font {})
}

impl Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "font parsing is unimplemented")
    }
}
