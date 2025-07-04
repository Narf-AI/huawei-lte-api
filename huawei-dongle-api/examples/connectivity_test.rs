use huawei_dongle_api::{Client, Config};
use std::error::Error;
use std::time::Duration;

const DEFAULT_DEVICE_IP: &str = "192.168.8.1";

/// Simple connectivity test for Huawei LTE device
/// 
/// This test performs the following checks:
/// 1. HTTP GET to root endpoint
/// 2. Check for common API endpoints
/// 3. Try to get basic device info without authentication
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let device_ip = std::env::var("HUAWEI_DEVICE_IP").unwrap_or_else(|_| DEFAULT_DEVICE_IP.to_string());
    
    println!("Huawei LTE Device Connectivity Test");
    println!("===================================");
    println!("Target device: {}", device_ip);
    println!();

    println!("1. Testing basic HTTP connectivity...");
    match test_http_connectivity(&device_ip).await {
        Ok(status) => println!("   ✓ HTTP connection successful (Status: {})", status),
        Err(e) => {
            println!("   ✗ HTTP connection failed: {}", e);
            println!("   Note: Cannot reach device. Please check:");
            println!("   - Device is powered on and connected");
            println!("   - Network interface is up");
            println!("   - IP address is correct ({})", device_ip);
            return Err(e);
        }
    }

    println!("\n2. Checking common API endpoints...");
    let endpoints = vec![
        "/api/webserver/SesTokInfo",
        "/api/device/information",
        "/api/monitoring/status",
        "/api/net/current-plmn",
        "/api/sms/sms-count",
    ];

    for endpoint in endpoints {
        match test_api_endpoint(&device_ip, endpoint).await {
            Ok(status) => println!("   ✓ {} - Status: {}", endpoint, status),
            Err(e) => println!("   ✗ {} - Error: {}", endpoint, e),
        }
    }

    println!("\n3. Testing session token endpoint...");
    match get_session_info(&device_ip).await {
        Ok(info) => {
            println!("   ✓ Session endpoint accessible");
            println!("   Response preview: {}", &info[..info.len().min(200)]);
        }
        Err(e) => {
            println!("   ✗ Session endpoint error: {}", e);
        }
    }

    println!("\n4. Testing with huawei-dongle-api client...");
    let config = Config::for_url(&format!("http://{}", device_ip))?;
    let client = Client::new(config)?;
    
    match client.device().information().await {
        Ok(device_info) => {
            println!("   ✓ Device information retrieved:");
            println!("   - Device Name: {}", device_info.device_name);
            println!("   - Model: {}", device_info.device_name);
            println!("   - Software Version: {}", device_info.software_version);
        }
        Err(e) => {
            println!("   ✗ Failed to get device info: {}", e);
        }
    }

    println!("\n5. Alternative testing approaches:");
    println!("   a) SSH to test node:");
    println!("      ssh user@testnode");
    println!("      curl -v http://{}/", device_ip);
    println!();
    println!("   b) Use Python huawei-lte-api for comparison:");
    println!("      python3 context/huawei-lte-api/examples/device_info.py http://{}/", device_ip);
    println!();
    println!("   c) Manual browser test:");
    println!("      Open http://{}/ in a web browser", device_ip);
    
    Ok(())
}

async fn test_http_connectivity(device_ip: &str) -> Result<u16, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    
    let url = format!("http://{}/", device_ip);
    let response = client.get(&url).send().await?;
    Ok(response.status().as_u16())
}

async fn test_api_endpoint(device_ip: &str, endpoint: &str) -> Result<u16, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    
    let url = format!("http://{}{}", device_ip, endpoint);
    let response = client.get(&url).send().await?;
    Ok(response.status().as_u16())
}

async fn get_session_info(device_ip: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    
    let url = format!("http://{}/api/webserver/SesTokInfo", device_ip);
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_device_connectivity() {
        // This test requires actual hardware connection
        assert!(test_http_connectivity(DEFAULT_DEVICE_IP).await.is_ok(), "Device should be reachable via HTTP");
    }
}