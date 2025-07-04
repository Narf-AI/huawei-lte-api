//! DHCP commands

use crate::{cli::OutputFormat, output::format_output};
use anyhow::Result;
use clap::Subcommand;
use huawei_dongle_api::{models::dhcp::DhcpSettingsRequest, Client};

#[derive(Subcommand)]
pub enum DhcpCommands {
    /// Show DHCP settings
    Show,
    /// Set gateway IP address
    SetIp {
        /// New gateway IP address (must be in format 192.168.x.1)
        ip: String,
    },
}

impl DhcpCommands {
    pub async fn execute(&self, client: &Client, format: &OutputFormat) -> Result<()> {
        match self {
            DhcpCommands::Show => {
                let settings = client.dhcp().settings().await?;
                format_output(&settings, format)?;
            }
            DhcpCommands::SetIp { ip } => {
                if !ip.starts_with("192.168.") || !ip.ends_with(".1") {
                    anyhow::bail!("Gateway IP must be in format 192.168.x.1");
                }

                let parts: Vec<&str> = ip.split('.').collect();
                if parts.len() != 4 {
                    anyhow::bail!("Invalid IP address format");
                }

                let subnet = parts[2];
                let subnet_num: u8 = subnet.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid subnet number"))?;

                if subnet_num == 0 {
                    anyhow::bail!("Subnet number must be between 1 and 255");
                }

                let current = client.dhcp().settings().await?;

                let start_ip = format!("192.168.{}.100", subnet);
                let end_ip = format!("192.168.{}.200", subnet);

                let request = DhcpSettingsRequest::new(
                    ip.clone(),
                    current.dhcp_lan_netmask,
                    current.dhcp_status,
                    start_ip,
                    end_ip,
                    current.dhcp_lease_time,
                    current.dns_status,
                    ip.clone(), // Primary DNS = gateway IP
                    ip.clone(), // Secondary DNS = gateway IP
                );

                client.dhcp().set_settings(&request).await?;
                println!("Gateway IP changed to: {}", ip);
                println!("Note: You may need to reconnect to the new IP address");
            }
        }
        Ok(())
    }
}