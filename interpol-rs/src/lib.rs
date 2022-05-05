#![feature(try_reserve_kind)]

pub mod interpol;
pub mod mpi_events;
pub mod types;

#[non_exhaustive]
#[derive(Debug)]
pub enum InterpolErrorKind {
    Io,
    TryReserve,
    DeriveBuilder,
}

#[derive(Debug)]
pub struct InterpolError {
    kind: InterpolErrorKind,
    reason: String,
}

impl std::fmt::Display for InterpolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self.kind {
            InterpolErrorKind::Io => format!("I/O error: {}", self.reason),
            InterpolErrorKind::TryReserve => format!("Memory allocation error: {}", self.reason),
            InterpolErrorKind::DeriveBuilder => format!("Builder error: {}", self.reason),
            // _ => String::from("Unknown error kind"),
        };

        write!(f, "{err_msg}")
    }
}

impl From<std::io::Error> for InterpolError {
    fn from(error: std::io::Error) -> Self {
        InterpolError {
            kind: InterpolErrorKind::Io,
            reason: error.to_string(),
        }
    }
}

impl From<std::collections::TryReserveError> for InterpolError {
    fn from(error: std::collections::TryReserveError) -> Self {
        InterpolError {
            kind: InterpolErrorKind::TryReserve,
            reason: error.to_string(),
        }
    }
}

#[macro_export]
macro_rules! impl_builder_error {
    ($t:ty) => {
        use crate::{InterpolError, InterpolErrorKind};

        impl From<$t> for InterpolError {
            fn from(error: $t) -> Self {
                InterpolError {
                    kind: InterpolErrorKind::DeriveBuilder,
                    reason: error.to_string(),
                }
            }
        }
    };
}
