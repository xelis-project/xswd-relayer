mod relayer;
mod session;
mod channel;
mod error;
mod config;
mod routes;

use clap::Parser;
use xelis_common::prompt::Prompt;

use crate::{
    config::Config,
    relayer::Relayer
};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let log_config = config.log;
    let _prompt = Prompt::new(
        log_config.log_level,
        &log_config.logs_path,
        &log_config.filename_log,
        log_config.disable_file_logging,
        log_config.disable_file_log_date_based,
        log_config.disable_log_color,
        log_config.auto_compress_logs,
        !log_config.disable_interactive_mode,
        log_config.logs_modules.clone(),
        log_config.file_log_level.unwrap_or(log_config.log_level),
        !log_config.disable_ascii_art,
        log_config.datetime_format.clone(),
    )?;

    let relayer = Relayer::new(config.relayer);
    
    relayer.run().await
}
