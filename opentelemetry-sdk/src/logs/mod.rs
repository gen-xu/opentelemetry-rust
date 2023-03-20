//! # OpenTelemetry Log SDK

mod config;
mod log_emitter;
mod log_processor;
mod log_record;
mod runtime;
mod provider;
mod logger;
pub use logger::Logger;
pub use config::Config;
pub use log_emitter::{Builder, LogEmitter, LogEmitterProvider};
pub use log_processor::{
    BatchConfig, BatchLogProcessor, BatchLogProcessorBuilder, BatchMessage, LogProcessor,
    SimpleLogProcessor,
};
pub use log_record::{Any, LogRecord, LogRecordBuilder, Severity, TraceContext};
pub use runtime::{LogRuntime, TrySend};
