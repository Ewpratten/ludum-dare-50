//! This module contains some bootstrapping code for setting up logging.

use std::env::temp_dir;

use chrono::Utc;
use fern::colors::ColoredLevelConfig;
use log::LevelFilter;
use thiserror::Error;

/// Error definitions for setting up the logging system
#[derive(Error, Debug)]
pub enum LoggingSystemInitError {
    /// Error caused by a failure to set the global logger
    #[error(transparent)]
    SetLoggerError(#[from] log::SetLoggerError),

    /// Error caused by a failure to open the filesystem log location
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Sets up global logging for an application
#[profiling::function]
pub fn init_logging_system(
    tool_name: &str,
    log_level: Option<LevelFilter>,
) -> Result<(), LoggingSystemInitError> {
    // Set up coloring system for STDIO logs
    let stdio_colors = ColoredLevelConfig::default();

    // Set up a dispatcher for STDIO
    // This will skip over un-needed verbosity
    log::trace!("Setting up STDIO logging");
    let stdio_dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}: {}",
                stdio_colors.color(record.level()),
                message
            ))
        })
        .level(log_level.unwrap_or(LevelFilter::Info))
        .chain(std::io::stdout());

    // Determine where to write the logfile
    let log_file_path = temp_dir().join(format!("{}.{}.log", tool_name, Utc::now().timestamp()));
    log::info!(
        "A verbose copy of the application log will be written to: {}",
        log_file_path.display()
    );

    // Set up a dispatcher for the logfile
    log::trace!("Setting up file logging");
    let fs_dispatch = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(fern::log_file(log_file_path)?);

    // Combine and apply the dispatchers
    log::trace!("Setting up global logger");
    fern::Dispatch::new()
        .chain(stdio_dispatch)
        .chain(fs_dispatch)
        .apply()?;

    Ok(())
}