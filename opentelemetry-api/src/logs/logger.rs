use super::log_record::LogRecord;


/// The interface for emitting [`LogRecord`]s.
pub trait Logger {
    /// The [`LogRecord`] type used by this logger.
    type LogRecord: LogRecord;
}
