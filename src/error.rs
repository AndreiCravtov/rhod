use thiserror::Error;

use crate::ImmutableString;
use crate::issue::RhodIssueOptionalMessage;

#[derive(Debug, Error)]
pub enum RhodError {
    #[error("TODO: make this a proper error")]
    TodoError,
}

/// The type of error messages
pub type RhodErrorMessage = ImmutableString;

/// Closure for mapping from a [RhodIssueOptionalMessage] and [RhodErrorMapContext], to a [RhodErrorMessage]
#[cfg(not(feature = "sync"))]
pub type RhodErrorMap = dyn Fn(RhodIssueOptionalMessage, RhodErrorMapContext) -> RhodErrorMessage;

/// Closure for mapping from a [RhodIssueOptionalMessage] and [RhodErrorMapContext], to a [RhodErrorMessage]
#[cfg(feature = "sync")]
pub type RhodErrorMap =
    dyn Fn(RhodIssueOptionalMessage, RhodErrorMapContext) -> RhodErrorMessage + Send + Sync;

/// The struct holding the current error context
pub struct RhodErrorMapContext {}
