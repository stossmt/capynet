use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CapyError {
    error_impl: Box<ErrorImpl>,
}

#[derive(Debug)]
pub enum ErrorCode {
    Cancelled,
    Unknown,
    InvalidArgument,
    DeadlineExceeded,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    ResourceExhausted,
    FailedPrecondition,
    Aborted,
    OutOfRange,
    Unimplemented,
    Internal,
    Unavailable,
    DataLoss,
    Unauthenticated,
}

impl std::error::Error for CapyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error_impl.source.as_deref()
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::Cancelled => write!(f, "CANCELLED"),
            ErrorCode::Unknown => write!(f, "UNKNOWN"),
            ErrorCode::InvalidArgument => write!(f, "INVALID_ARGUMENT"),
            ErrorCode::DeadlineExceeded => write!(f, "DEADLINE_EXCEEDED"),
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::AlreadyExists => write!(f, "ALREADY_EXISTS"),
            ErrorCode::PermissionDenied => write!(f, "PERMISSION_DENIED"),
            ErrorCode::ResourceExhausted => write!(f, "RESOURCE_EXHAUSTED"),
            ErrorCode::FailedPrecondition => write!(f, "FAILED_PRECONDITION"),
            ErrorCode::Aborted => write!(f, "ABORTED"),
            ErrorCode::OutOfRange => write!(f, "OUT_OF_RANGE"),
            ErrorCode::Unimplemented => write!(f, "UNIMPLEMENTED"),
            ErrorCode::Internal => write!(f, "INTERNAL"),
            ErrorCode::Unavailable => write!(f, "UNAVAILABLE"),
            ErrorCode::DataLoss => write!(f, "DATA_LOSS"),
            ErrorCode::Unauthenticated => write!(f, "UNAUTHENTICATED"),
        }
    }
}

impl Display for CapyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // FIXME: Print the entire error chain (the source field)
        write!(
            f,
            "[{}] {}\n",
            self.error_impl.code, self.error_impl.message
        )?;
        if let Some(source) = &self.error_impl.source {
            write!(f, "   Caused by: {}\n", source)?;
        }
        Ok(())
    }
}

impl CapyError {
    pub fn new(code: ErrorCode, message: &'static str) -> Self {
        Self {
            error_impl: Box::new(ErrorImpl {
                code,
                message: message.to_string(),
                source: None,
            }),
        }
    }

    pub fn with_source(
        code: ErrorCode,
        message: &'static str,
        source: Box<dyn std::error::Error + 'static>,
    ) -> Self {
        Self {
            error_impl: Box::new(ErrorImpl {
                code,
                message: message.to_string(),
                source: Some(source),
            }),
        }
    }

    pub fn with_context(mut self, extra_context: &str) -> Self {
        self.error_impl.message = format!("{}: {}", extra_context, self.error_impl.message);
        self
    }
}

#[derive(Debug)]
struct ErrorImpl {
    code: ErrorCode,
    message: String,
    source: Option<Box<dyn std::error::Error + 'static>>,
}

impl From<std::io::Error> for CapyError {
    fn from(err: std::io::Error) -> CapyError {
        CapyError::with_source(
            ErrorCode::Unknown,
            "FIXME: define a real IOError => CapyError mapping",
            Box::new(err),
        )
    }
}

pub trait ResultExt<T, E> {
    fn with_error_context(self, context: &'static str) -> Result<T, CapyError>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Into<CapyError>,
{
    fn with_error_context(self, context: &'static str) -> Result<T, CapyError> {
        self.map_err(|e| e.into().with_context(context))
    }
}
