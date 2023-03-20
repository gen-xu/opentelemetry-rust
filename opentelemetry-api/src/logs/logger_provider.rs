use std::borrow::Cow;

use crate::attributes::AttributeSet;

use super::logger::Logger;

/// The interface for providing instance of [`Logger`].
pub trait LoggerProvider {
    /// The [`Logger`] type that this provider will return
    type Logger: Logger;

    /// See also: https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/logs/bridge-api.md#get-a-logger
    fn get_logger(
        &self,
        name: impl Into<Cow<'static, str>>,
        version: Option<&'static str>,
        schema_url: Option<&'static str>,
        include_trace_context: Option<bool>,
        attributes: Option<AttributeSet>,
    ) -> Self::Logger;
}
