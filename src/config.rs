use std::time::Duration;

use clap::Parser;
use humantime::parse_duration;
use serde::{Deserialize, Serialize};
use xelis_common::prompt::{default_logs_datetime_format, LogLevel, ModuleConfig};

fn default_prometheus_route() -> String {
    "/metrics".to_owned()
}

#[derive(Debug, Clone, clap::Args, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enable Prometheus metrics server
    #[clap(long = "prometheus-enable")]
    #[serde(default)]
    pub enable: bool,
    /// Route for the Prometheus metrics export
    #[clap(name = "prometheus-route", long, default_value_t = default_prometheus_route())]
    #[serde(default = "default_prometheus_route")]
    pub route: String,
}

// Functions helpers for serde default values
fn default_filename_log() -> String {
    "xswd-relayer.log".to_owned()
}

fn default_logs_path() -> String {
    "logs/".to_owned()
}

#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
pub struct LogConfig {
    /// Set log level
    #[clap(long, value_enum, default_value_t = LogLevel::Info)]
    #[serde(default)]
    pub log_level: LogLevel,
    /// Set file log level
    /// By default, it will be the same as log level
    #[clap(long, value_enum)]
    pub file_log_level: Option<LogLevel>,
    /// Disable the log file
    #[clap(long)]
    #[serde(default)]
    pub disable_file_logging: bool,
    /// Disable the log filename date based
    /// If disabled, the log file will be named xswd-relayer.log instead of YYYY-MM-DD.xswd-relayer.log
    #[clap(long)]
    #[serde(default)]
    pub disable_file_log_date_based: bool,
    /// Disable the usage of colors in log
    #[clap(long)]
    #[serde(default)]
    pub disable_log_color: bool,
    /// Disable terminal interactive mode
    /// You will not be able to write CLI commands in it or to have an updated prompt
    #[clap(long)]
    #[serde(default)]
    pub disable_interactive_mode: bool,
    /// Enable the log file auto compression
    /// If enabled, the log file will be compressed every day
    /// This will only work if the log file is enabled
    #[clap(long)]
    #[serde(default)]
    pub auto_compress_logs: bool,
    /// Log filename
    /// 
    /// By default filename is xswd-relayer.log.
    /// File will be stored in logs directory, this is only the filename, not the full path.
    /// Log file is rotated every day and has the format YYYY-MM-DD.xswd-relayer.log.
    #[clap(long, default_value_t = default_filename_log())]
    #[serde(default = "default_filename_log")]
    pub filename_log: String,
    /// Logs directory
    /// 
    /// By default it will be logs/ of the current directory.
    /// It must end with a / to be a valid folder.
    #[clap(long, default_value_t = default_logs_path())]
    #[serde(default = "default_logs_path")]
    pub logs_path: String,
    /// Module configuration for logs
    #[clap(long)]
    #[serde(default)]
    pub logs_modules: Vec<ModuleConfig>,
    /// Disable the ascii art at startup
    #[clap(long)]
    #[serde(default)]
    pub disable_ascii_art: bool,
    /// Change the datetime format used by the logger
    #[clap(long, default_value_t = default_logs_datetime_format())]
    #[serde(default = "default_logs_datetime_format")]
    pub datetime_format: String, 
}

const fn default_max_frame_size() -> usize {
    64 * 1024
}

const fn default_keep_alive_interval() -> Duration {
    Duration::from_secs(60)
}

const fn default_session_message_timeout() -> Duration {
    Duration::from_secs(1)
}

const fn default_channel_creation_timeout() -> Duration {
    Duration::from_secs(120)
}

fn default_bind_address() -> String {
    "0.0.0.0:8080".to_owned()
}

#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
pub struct RelayerConfig {
    /// Sets the maximum permitted size for received WebSocket frames, in bytes.
    #[arg(long, default_value_t = default_max_frame_size())]
    #[serde(default = "default_max_frame_size")]
    pub max_frame_size: usize,
    /// Interval at which keep-alive ping messages are sent over the WebSocket connection.
    /// If set, the server will periodically send pings to ensure the connection is still alive.
    #[arg(long, value_parser = parse_duration, default_value = format!("{:?}", default_keep_alive_interval()))]
    #[serde(with = "humantime_serde", default = "default_keep_alive_interval")]
    pub keep_alive_interval: Duration,
    /// Maximum duration allowed to send a message to a session before timing out.
    /// If a WebSocket message cannot be delivered within this time, the connection will be terminated.
    #[arg(long, value_parser = parse_duration, default_value = format!("{:?}", default_session_message_timeout()))]
    #[serde(with = "humantime_serde", default = "default_session_message_timeout")]
    pub session_message_timeout: Duration,
    /// Maximum duration before timeout for waiting on the peer to join once a channel has been created.
    /// If no peer is connecting to the channel, the channel is expired and the host connection is closed.
    #[arg(long, value_parser = parse_duration, default_value = format!("{:?}", default_channel_creation_timeout()))]
    #[serde(with = "humantime_serde", default = "default_channel_creation_timeout")]
    pub channel_creation_timeout: Duration,
    /// IP:Port address to use for listening connections
    #[arg(long, default_value_t = default_bind_address())]
    #[serde(default = "default_bind_address")]
    pub bind_address: String,
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Prometheus configuration
    #[structopt(flatten)]
    pub prometheus: PrometheusConfig,
    /// Log configuration
    #[structopt(flatten)]
    pub log: LogConfig,
    /// Relayer configuration
    #[structopt(flatten)]
    pub relayer: RelayerConfig,
}