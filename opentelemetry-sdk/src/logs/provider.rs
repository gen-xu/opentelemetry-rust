use super::logger::Logger;

struct LoggerProvider {}

impl LoggerProvider {}

impl opentelemetry_api::logs::LoggerProvider for LoggerProvider {
    type Logger = Logger;

    fn get_logger(
        &self,
        name: impl Into<std::borrow::Cow<'static, str>>,
        version: Option<&'static str>,
        schema_url: Option<&'static str>,
        include_trace_context: Option<bool>,
        attributes: Option<opentelemetry_api::attributes::AttributeSet>,
    ) -> Self::Logger {
        todo!()
    }
}
