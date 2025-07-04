//! Network CLI commands

use crate::{cli::OutputFormat, output::format_output};
use anyhow::Result;
use clap::Subcommand;
use huawei_dongle_api::{models::{network::NetworkModeRequest, NetworkModeType}, Client};

#[derive(Subcommand)]
pub enum NetworkCommands {
    /// Get current network mode configuration
    Mode,

    /// Set network mode
    SetMode {
        /// Network mode (00=Auto, 01=2G, 02=3G, 03=4G, 0302=4G+3G fallback)
        mode: String,

        /// Network band (hex, default: 3fffffff for all bands)
        #[arg(long, default_value = "3fffffff")]
        network_band: String,

        /// LTE band (hex, default: 80800C5 for common bands)
        #[arg(long, default_value = "80800C5")]
        lte_band: String,

        /// Wait for reconnection after mode change
        #[arg(long)]
        wait: bool,
    },

    /// Get current network operator (PLMN) information
    Operator,
}

impl NetworkCommands {
    pub async fn execute(&self, client: &Client, format: &OutputFormat) -> Result<()> {
        match self {
            NetworkCommands::Mode => {
                let mode = client.network().get_mode().await?;

                match format {
                    OutputFormat::Table => {
                        println!("Network Mode: {} ({})", mode.network_mode, mode.mode_text());
                        println!("Network Band: {}", mode.network_band);
                        println!("LTE Band: {}", mode.lte_band);
                    }
                    _ => {
                        format_output(&mode, format)?;
                    }
                }
            }

            NetworkCommands::SetMode {
                mode,
                network_band,
                lte_band,
                wait,
            } => {
                let mode_enum = parse_network_mode(mode)?;
                let request =
                    NetworkModeRequest::new(mode_enum, network_band.clone(), lte_band.clone());

                println!(
                    "Changing network mode to: {} ({})",
                    mode,
                    request.network_mode.to_string()
                );
                println!("Warning: This will temporarily disconnect the device!");

                client.network().set_mode(&request).await?;
                println!("Network mode changed successfully");

                if *wait {
                    println!("Note: Mode change will cause temporary disconnection");
                    println!("You may need to wait manually for reconnection");
                }
            }

            NetworkCommands::Operator => {
                let plmn = client.network().current_plmn().await?;

                match format {
                    OutputFormat::Table => {
                        if let Some(name) = plmn.operator_name() {
                            println!("Operator: {}", name);
                        }
                        if let Some(numeric) = &plmn.numeric {
                            println!("Numeric ID: {}", numeric);
                        }
                        println!("State: {}", plmn.state);
                    }
                    _ => {
                        format_output(&plmn, format)?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// Parse network mode string to enum
fn parse_network_mode(mode: &str) -> Result<NetworkModeType> {
    match mode {
        "00" => Ok(NetworkModeType::Auto),
        "01" => Ok(NetworkModeType::TwoGOnly),
        "02" => Ok(NetworkModeType::ThreeGOnly),
        "03" => Ok(NetworkModeType::FourGOnly),
        "0201" => Ok(NetworkModeType::ThreeGPreferredTwoGFallback),
        "0301" => Ok(NetworkModeType::FourGPreferredTwoGFallback),
        "0302" => Ok(NetworkModeType::FourGPreferredThreeGFallback),
        _ => Err(anyhow::anyhow!("Invalid network mode: {}. Valid modes: 00, 01, 02, 03, 0201, 0301, 0302", mode)),
    }
}
