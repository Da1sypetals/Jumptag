use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum TagbaseError {
    #[error("jump-tag: internal error: {}", .0)]
    Internal(String),

    #[error("jump-tag: tag not found: {}", .0)]
    TagNotFound(String),
}

pub(crate) type TagbaseResult<T> = Result<T, TagbaseError>;

pub(crate) trait ReportError<T> {
    fn report(self) -> T;
}

