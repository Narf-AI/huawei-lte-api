# Huawei LTE Rust Client

Async Rust library and CLI for Huawei LTE dongles/routers.

> This library powers the mobile proxy infrastructure at **[Scraping Fish](https://scrapingfish.com)**, managing fleets of LTE dongles for IP rotation and mobile proxy functionality at scale.

## Features

- 🔄 Async API with automatic retry and connection pooling
- 🔐 Session management with CSRF token handling
- 📱 Device control (info, reboot, power off)
- 🌐 Network configuration (mode switching)
- 📊 Connection monitoring and status
- 💬 SMS management (list, delete, mark as read)
- 🖥️ DHCP configuration
- 🔧 CLI with multiple output formats (table, JSON)
- 🦾 Strong typing with enums for all API values

## Installation

### CLI Tool

```bash
cargo install huawei-dongle-cli
```

### Library

Add to your `Cargo.toml`:

```toml
[dependencies]
huawei-dongle-api = "0.2"
```

## Library Usage

```rust
use huawei_dongle_api::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Config::for_url("http://192.168.8.1")?)?;
    
    // Get device info
    let info = client.device().information().await?;
    println!("Device: {}", info.device_name);
    
    // Check connection status  
    let status = client.monitoring().status().await?;
    println!("Connected: {}", status.is_connected());
    
    Ok(())
}
```

## CLI Usage

```bash
# Device operations
huawei-dongle-cli device info
huawei-dongle-cli device reboot --confirm

# Network operations  
huawei-dongle-cli network mode
huawei-dongle-cli network set-mode 4g-only

# Monitoring
huawei-dongle-cli monitoring status

# SMS management
huawei-dongle-cli sms list
huawei-dongle-cli sms delete <id>

# DHCP configuration
huawei-dongle-cli dhcp show
huawei-dongle-cli dhcp set-ip 192.168.62.1

# Custom device URL
huawei-dongle-cli --url http://192.168.62.1 device info
```

## Strong Types

The library uses type-safe enums for all API values instead of magic strings:

- **ConnectionStatus**: `Connected`, `Disconnected`, `Connecting`, etc.
- **NetworkType**: `Lte`, `FiveGNsa`, `FiveGSa`, `Hspa`, etc.
- **NetworkModeType**: `Auto`, `FourGOnly`, `ThreeGOnly`, etc.
- **SmsStatus**: `Unread`, `Read`, `Sent`, etc.
- **SimStatus**: `Ready`, `NotReady`
- **ServiceStatus**: `FullService`, `LimitedService`, `NoService`
- **DhcpStatus/DnsStatus**: `Enabled`, `Disabled`
- **ApiErrorCode**: Strongly typed error codes

Example:
```rust
let status = client.monitoring().status().await?;
if status.connection_status == ConnectionStatus::Connected {
    match status.current_network_type {
        NetworkType::Lte => println!("Connected via 4G"),
        NetworkType::FiveGNsa => println!("Connected via 5G NSA"),
        _ => println!("Connected via other network"),
    }
}
```

## API Coverage

| API | Endpoint | Status |
|-----|----------|--------|
| Device Info | `GET /api/device/information` | ✅ |
| Device Control | `POST /api/device/control` | ✅ |
| Connection Status | `GET /api/monitoring/status` | ✅ |
| Network Mode | `GET/POST /api/net/net-mode` | ✅ |
| Current PLMN | `GET /api/net/current-plmn` | ✅ |
| SMS Count | `GET /api/sms/sms-count` | ✅ |
| SMS List | `POST /api/sms/sms-list` | ✅ |
| SMS Delete | `POST /api/sms/delete-sms` | ✅ |
| SMS Mark Read | `POST /api/sms/set-read` | ✅ |
| DHCP Settings | `GET/POST /api/dhcp/settings` | ✅ |

## Development

```bash
# Run tests
cargo test

# Check with clippy
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Build for release
cargo build --release
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
