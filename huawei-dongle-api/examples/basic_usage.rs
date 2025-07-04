//! Basic usage example for the Huawei Dongle API

use huawei_dongle_api::{Client, Config};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = Config::builder()
        .base_url("http://192.168.8.1")
        .timeout(Duration::from_secs(30))
        .max_retries(3)
        .build()?;

    let client = Client::new(config)?;

    println!("Connecting to Huawei dongle...");

    match client.device().information().await {
        Ok(device_info) => {
            println!("Device Information:");
            println!("  Device Name: {}", device_info.device_name);
            println!("  Serial Number: {}", device_info.serial_number);
            println!("  IMEI: {}", device_info.imei);
            if let Some(imsi) = device_info.imsi {
                println!("  IMSI: {}", imsi);
            }
            if let Some(iccid) = device_info.iccid {
                println!("  ICCID: {}", iccid);
            }
            println!("  Hardware Version: {}", device_info.hardware_version);
            println!("  Software Version: {}", device_info.software_version);
            if let Some(webui_version) = device_info.webui_version {
                println!("  WebUI Version: {}", webui_version);
            }
        }
        Err(e) => {
            eprintln!("Failed to get device information: {}", e);
            std::process::exit(1);
        }
    }

    println!("\nTest completed successfully!");
    Ok(())
}
