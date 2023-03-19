//! Metrics Export

#[cfg(feature = "logs")]
#[cfg_attr(docsrs, doc(cfg(feature = "logs")))]
pub mod logs;
#[cfg(feature = "metrics")]
#[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
pub mod metrics;
#[cfg(feature = "trace")]
#[cfg_attr(docsrs, doc(cfg(feature = "trace")))]
pub mod trace;

pub use opentelemetry_api::ExportError;
