use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(crate) enum AppError {
    #[allow(dead_code)]
    #[error("{entity} \"{name}\" not found")]
    NotFound { entity: String, name: String },

    #[error("internal error")]
    Internal,
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        // Full causal chain stays in logs; the frontend only sees the kind.
        tracing::error!(error = format!("{err:#}"), "internal command error");
        Self::Internal
    }
}

pub(crate) type CommandResult<T> = Result<T, AppError>;
