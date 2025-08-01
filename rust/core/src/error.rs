use std::{error, fmt};

struct ErrorInner {
    when: &'static str,
    cause: Option<Box<dyn error::Error + Sync + Send>>,
}

/// An error when managing the liveo workspace.
pub struct Error(Box<ErrorInner>);

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Error")
            .field("when", &self.0.when)
            .field("cause", &self.0.cause)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0.when)?;
        if let Some(ref cause) = self.0.cause {
            write!(fmt, ": {}", cause)?;
        }
        Ok(())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.0.cause.as_ref().map(|e| &**e as _)
    }
}

impl Error {
    pub(crate) fn new(
        when: &'static str,
        cause: Option<Box<dyn error::Error + Sync + Send>>,
    ) -> Error {
        Error(Box::new(ErrorInner { when, cause }))
    }
}
