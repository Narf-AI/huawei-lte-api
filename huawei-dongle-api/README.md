# huawei-dongle-api

[![Documentation](https://docs.rs/huawei-dongle-api/badge.svg)](https://docs.rs/huawei-dongle-api)

A robust async Rust client library for interacting with Huawei LTE dongles and routers.

This library provides a type-safe, async interface to the XML-based API used by Huawei HiLink devices such as the E3372, E5577, B525 and many others. It handles authentication, session management, and CSRF token rotation automatically.

## Features

- **Async/await** - Built on tokio for efficient async I/O
- **Automatic retry** - Configurable retry logic with exponential backoff
- **Session management** - Automatic CSRF token handling and refresh
- **Type safety** - Strongly typed requests and responses with enums
- **Error handling** - Comprehensive error types with automatic recovery
- **Device compatibility** - Handles quirks across different firmware versions
- **Strong types** - Enums for all API values (connection status, network types, etc.)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
huawei-dongle-api = "0.2"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use huawei_dongle_api::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with default config
    let client = Client::new(Config::default())?;
    
    // Get device information (no auth required)
    let device_info = client.device().information().await?;
    println!("Device: {}", device_info.device_name);
    
    // Check connection status (auth handled automatically)
    let status = client.monitoring().status().await?;
    println!("Connected: {}", status.is_connected());
    
    Ok(())
}
```

## Configuration

The client can be configured with custom settings:

```rust
use huawei_dongle_api::{Client, Config};
use std::time::Duration;

let config = Config::builder()
    .base_url("http://192.168.8.1")
    .timeout(Duration::from_secs(30))
    .max_retries(5)
    .retry_delay(Duration::from_millis(500))
    .build()?;
    
let client = Client::new(config)?;
```

## Authentication

Most endpoints require authentication. The library handles this automatically, retrying with fresh tokens when needed:

```rust
// Login (only needed for password-protected operations)
client.auth().login("admin", "password").await?;

// Access protected endpoints
use huawei_dongle_api::models::{SmsListRequest, SmsBoxType, SmsSortType};
let request = SmsListRequest::new(1, 20, SmsBoxType::LocalInbox, SmsSortType::ByTime, false, true);
let sms_list = client.sms().list(&request).await?;

// Logout when done
client.auth().logout().await?;
```

## Examples

### Monitoring Connection Status

```rust
let status = client.monitoring().status().await?;

println!("Connection: {}", status.connection_status_text());
println!("Network: {}", status.network_type_text());
println!("Signal: {}/5", status.signal_level().unwrap_or(0));

if status.is_connected() {
    println!("Connected to {}", status.operator_name().unwrap_or("Unknown"));
}
```

### SMS Management

```rust
use huawei_dongle_api::models::{SmsListRequest, SmsBoxType, SmsSortType};

// Get SMS count
let count = client.sms().count().await?;
println!("Unread messages: {}", count.total_unread().unwrap_or(0));

// List messages
let request = SmsListRequest::new(
    1,                        // page
    20,                       // count per page
    SmsBoxType::LocalInbox,   // inbox
    SmsSortType::ByTime,      // sort by date
    false,                    // descending
    true,                     // unread first
);

let messages = client.sms().list(&request).await?;
for msg in &messages.messages.messages {
    println!("From: {} - {}", msg.phone, msg.content);
}

// Delete a message
client.sms().delete("40001").await?;
```

### Network Configuration

```rust
// Get current network mode
let net_mode = client.network().get_mode().await?;
println!("Network mode: {}", net_mode.mode_text());

// Switch network mode (for IP rotation)
use huawei_dongle_api::models::network::NetworkModeRequest;

let request = NetworkModeRequest::lte_only();
client.network().set_mode(&request).await?;
```

### DHCP Configuration

```rust
// Get current DHCP settings
let settings = client.dhcp().settings().await?;
println!("Gateway IP: {}", settings.dhcp_ip_address);

// Change gateway IP
use huawei_dongle_api::models::dhcp::DhcpSettingsRequest;
use huawei_dongle_api::models::{DhcpStatus, DnsStatus};

let new_settings = DhcpSettingsRequest::new(
    "192.168.62.1".to_string(),      // new gateway IP
    "255.255.255.0".to_string(),     // netmask
    DhcpStatus::Enabled,              // DHCP enabled
    "192.168.62.100".to_string(),    // start IP
    "192.168.62.200".to_string(),    // end IP
    "86400".to_string(),              // lease time (seconds)
    DnsStatus::Enabled,               // DNS enabled
    "192.168.62.1".to_string(),      // primary DNS
    "192.168.62.1".to_string(),      // secondary DNS
);

client.dhcp().set_settings(&new_settings).await?;
```

## Error Handling

The library provides comprehensive error handling with automatic recovery:

```rust
match client.monitoring().status().await {
    Ok(status) => println!("Connected: {}", status.is_connected()),
    Err(huawei_dongle_api::Error::CsrfTokenInvalid) => {
        // This is handled automatically with retry
        println!("Token error (handled automatically)");
    }
    Err(huawei_dongle_api::Error::LoginRequired) => {
        println!("Need to login first");
    }
    Err(e) => println!("Error: {}", e),
}
```

## Supported Devices

This library has been tested with:

- Huawei E3372h-320
- Huawei E5577
- Huawei B525

It should work with any Huawei HiLink device that uses the same XML API.

## Thread Safety

The client is thread-safe and can be shared across multiple tasks:

```rust
use std::sync::Arc;

let client = Arc::new(Client::new(Config::default())?);

let client2 = client.clone();
tokio::spawn(async move {
    let status = client2.monitoring().status().await;
});
```

## Used In Production

This library powers the mobile proxy infrastructure at **[Scraping Fish API](https://scrapingfish.com)**, a high-performance web scraping API service.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.