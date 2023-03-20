use super::LogRecord;

/// Logger implementaion
pub struct Logger {

}


impl opentelemetry_api::logs::Logger for Logger  {
    type LogRecord = LogRecord;
}