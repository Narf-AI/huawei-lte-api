//! CLI commands

use crate::cli::OutputFormat;
use anyhow::Result;
use clap::Subcommand;
use huawei_dongle_api::Client;

pub mod device;
pub mod dhcp;
pub mod monitoring;
pub mod network;
pub mod sms;

#[derive(Subcommand)]
pub enum Commands {
    /// Device information and control
    Device {
        #[command(subcommand)]
        command: device::DeviceCommands,
    },
    /// Network configuration and status
    Network {
        #[command(subcommand)]
        command: network::NetworkCommands,
    },
    /// SMS management
    Sms {
        #[command(subcommand)]
        command: sms::SmsCommands,
    },
    /// Status monitoring
    Monitoring {
        #[command(subcommand)]
        command: monitoring::MonitoringCommands,
    },
    /// DHCP configuration
    Dhcp {
        #[command(subcommand)]
        command: dhcp::DhcpCommands,
    },
}

impl Commands {
    pub async fn execute(&self, client: &Client, format: &OutputFormat) -> Result<()> {
        match self {
            Commands::Device { command } => command.execute(client, format).await,
            Commands::Network { command } => command.execute(client, format).await,
            Commands::Sms { command } => command.execute(client, format).await,
            Commands::Monitoring { command } => command.execute(client, format).await,
            Commands::Dhcp { command } => command.execute(client, format).await,
        }
    }
}
