//! Huawei Dongle CLI

mod cli;
mod commands;
mod config;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let cli = Cli::parse();

    cli.execute().await
}
