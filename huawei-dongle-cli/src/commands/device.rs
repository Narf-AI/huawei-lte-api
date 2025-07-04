//! Device CLI commands

use crate::{cli::OutputFormat, output::format_output};
use anyhow::Result;
use clap::Subcommand;
use huawei_dongle_api::Client;

#[derive(Subcommand)]
pub enum DeviceCommands {
    /// Get device information
    Info,
    /// Reboot the device
    Reboot {
        /// Skip confirmation prompt
        #[arg(long)]
        confirm: bool,
    },
    /// Power off the device
    PowerOff {
        /// Skip confirmation prompt
        #[arg(long)]
        confirm: bool,
    },
}

impl DeviceCommands {
    pub async fn execute(&self, client: &Client, format: &OutputFormat) -> Result<()> {
        match self {
            DeviceCommands::Info => {
                let device_info = client.device().information().await?;
                format_output(&device_info, format)?;
            }
            DeviceCommands::Reboot { confirm } => {
                if !confirm {
                    println!("Are you sure you want to reboot the device? Use --confirm to skip this prompt.");
                    return Ok(());
                }

                client.device().reboot().await?;
                println!("Device reboot initiated successfully");
            }
            DeviceCommands::PowerOff { confirm } => {
                if !confirm {
                    println!("Are you sure you want to power off the device? Use --confirm to skip this prompt.");
                    return Ok(());
                }

                client.device().power_off().await?;
                println!("Device power off initiated successfully");
            }
        }
        Ok(())
    }
}
