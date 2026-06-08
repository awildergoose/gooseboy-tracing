use std::io::Write;

use gooseboy::bindings::log;
use tracing::{level_filters::LevelFilter, subscriber::SetGlobalDefaultError};
use tracing_subscriber::{
    Layer,
    fmt::{MakeWriter, time::FormatTime},
    layer::SubscriberExt,
};

struct GooseboyLog;

impl<'a> MakeWriter<'a> for GooseboyLog {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        Self
    }
}

impl Write for GooseboyLog {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let str = String::from_utf8_lossy(buf);
        let str = str.trim_end();
        let len = str.len().try_into().unwrap();

        unsafe {
            log(str.as_ptr(), len);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl FormatTime for GooseboyLog {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let nanos = gooseboy::system::get_time_nanos();

        let secs = nanos / 1_000_000_000;
        let subsec = nanos % 1_000_000_000;

        write!(w, "{secs}.{subsec:09}")
    }
}

fn build_subscriber(level: LevelFilter) -> impl tracing::Subscriber {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(GooseboyLog)
        .with_timer(GooseboyLog)
        .with_filter(level);

    tracing_subscriber::registry().with(fmt_layer)
}

/// Initialize tracing with `level` filter.
///
/// # Errors
///
/// This function will return an error if the global default logger has already been set.
pub fn init_with_level(level: LevelFilter) -> Result<(), SetGlobalDefaultError> {
    let subscriber = build_subscriber(level);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

/// Initialize tracing with `LevelFilter::INFO` filter
///
/// # Errors
///
/// This function will return an error if the global default logger has already been set.
pub fn init() -> Result<(), SetGlobalDefaultError> {
    init_with_level(LevelFilter::INFO)
}
