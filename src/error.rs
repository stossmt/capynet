use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CapyError {
    error_impl: Box<ErrorImpl>,
}

#[derive(Debug)]
pub enum ErrorCode {
    BadArgument,
}

impl std::error::Error for CapyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error_impl.source.as_deref()
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::BadArgument => write!(f, "[BAD ARGUMENT]"),
        }
    }
}

impl Display for CapyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // FIXME: Print the entire error chain (the source field)
        write!(f, "{} {}\n", self.error_impl.code, self.error_impl.message)?;
        if let Some(source) = &self.error_impl.source {
            write!(f, "   Caused by: {}\n", source)?;
        }
        Ok(())
    }
}

impl CapyError {
    pub fn with_source(
        code: ErrorCode,
        message: &'static str,
        source: Box<dyn std::error::Error + 'static>,
    ) -> Self {
        Self {
            error_impl: Box::new(ErrorImpl {
                code,
                message,
                source: Some(source),
            }),
        }
    }
}

#[derive(Debug)]
struct ErrorImpl {
    code: ErrorCode,
    message: &'static str,
    source: Option<Box<dyn std::error::Error + 'static>>,
}

impl From<std::io::Error> for CapyError {
    fn from(err: std::io::Error) -> CapyError {
        CapyError::with_source(ErrorCode::BadArgument, "I/O Error occurred", Box::new(err))
    }
}
