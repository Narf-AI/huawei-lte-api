//! CLI argument parsing and command structure

use crate::commands::Commands;
use anyhow::Result;
use clap::Parser;
use huawei_dongle_api::{Client, Config};
use std::time::Duration;

#[derive(Parser)]
#[command(name = "huawei-dongle-cli")]
#[command(about = "A CLI for interacting with Huawei LTE dongles")]
#[command(version)]
pub struct Cli {
    /// Device URL
    #[arg(long, default_value = "http://192.168.8.1")]
    pub url: String,

    /// Request timeout in seconds
    #[arg(long, default_value = "30")]
    pub timeout: u64,

    /// Maximum retry attempts
    #[arg(long, default_value = "3")]
    pub retries: usize,

    /// Output format
    #[arg(long, value_enum, default_value = "table")]
    pub format: OutputFormat,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

impl Cli {
    /// Execute the CLI command
    pub async fn execute(self) -> Result<()> {
        if self.verbose {
            let _ = tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .try_init();
        }

        let config = Config::builder()
            .base_url(self.url)
            .timeout(Duration::from_secs(self.timeout))
            .max_retries(self.retries)
            .build()?;

        let client = Client::new(config)?;

        self.command.execute(&client, &self.format).await
    }
}
