//! Monitoring CLI commands

use crate::{cli::OutputFormat, output::format_output};
use anyhow::Result;
use clap::Subcommand;
use huawei_dongle_api::Client;

#[derive(Subcommand)]
pub enum MonitoringCommands {
    /// Get connection status and network information
    Status {
        /// Watch mode - continuously monitor status
        #[arg(long)]
        watch: bool,

        /// Watch interval in seconds
        #[arg(long, default_value = "5")]
        interval: u64,
    },
}

impl MonitoringCommands {
    pub async fn execute(&self, client: &Client, format: &OutputFormat) -> Result<()> {
        match self {
            MonitoringCommands::Status { watch, interval } => {
                if *watch {
                    self.watch_status(client, format, *interval).await?;
                } else {
                    let status = client.monitoring().status().await?;

                    match format {
                        OutputFormat::Table => {
                            println!("Connection Status: {}", status.connection_status_text());
                            println!("Network Type: {}", status.network_type_text());
                            if let Some(ex_type) = status.current_network_type_ex.as_ref() {
                                if ex_type != &status.current_network_type {
                                    println!("Extended Type: {}", status.network_type_ex_text());
                                }
                            }

                            if let Some(level) = status.signal_level() {
                                println!(
                                    "Signal Strength: {}/5 ({}%)",
                                    level,
                                    status.signal_percentage().unwrap_or(0)
                                );
                            }

                            println!(
                                "SIM Status: {}",
                                if status.is_sim_ready() {
                                    "Ready"
                                } else {
                                    "Not Ready"
                                }
                            );
                            println!(
                                "Service: {}",
                                if status.is_service_available() {
                                    "Available"
                                } else {
                                    "Unavailable"
                                }
                            );
                            println!(
                                "Roaming: {}",
                                if status.is_roaming() { "Yes" } else { "No" }
                            );

                            if let Some(primary_dns) = &status.primary_dns {
                                println!("Primary DNS: {}", primary_dns);
                            }
                            if let Some(secondary_dns) = &status.secondary_dns {
                                println!("Secondary DNS: {}", secondary_dns);
                            }
                        }
                        _ => {
                            format_output(&status, format)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn watch_status(
        &self,
        client: &Client,
        format: &OutputFormat,
        interval: u64,
    ) -> Result<()> {
        use tokio::time::{sleep, Duration};

        println!("Monitoring status (Press Ctrl+C to stop)...\n");

        loop {
            match client.monitoring().status().await {
                Ok(status) => {
                    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

                    match format {
                        OutputFormat::Table => {
                            println!(
                                "[{}] {} | {} | Signal: {}/5 | SIM: {} | Service: {}",
                                timestamp,
                                status.connection_status_text(),
                                status.network_type_text(),
                                status.signal_level().unwrap_or(0),
                                if status.is_sim_ready() {
                                    "Ready"
                                } else {
                                    "Not Ready"
                                },
                                if status.is_service_available() {
                                    "Available"
                                } else {
                                    "Unavailable"
                                }
                            );
                        }
                        _ => {
                            println!("--- {} ---", timestamp);
                            format_output(&status, format)?;
                            println!();
                        }
                    }
                }
                Err(e) => {
                    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
                    eprintln!("[{}] Error fetching status: {}", timestamp, e);
                }
            }

            sleep(Duration::from_secs(interval)).await;
        }
    }
}
