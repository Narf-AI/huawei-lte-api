# huawei-dongle-cli

Command-line interface for interacting with Huawei LTE dongles and routers. Built on top of the [huawei-dongle-api](https://crates.io/crates/huawei-dongle-api) library.

## Installation

```bash
cargo install huawei-dongle-cli
```

## Usage

```bash
huawei-dongle-cli [OPTIONS] <COMMAND>

Commands:
  device      Device information and control
  network     Network configuration and status
  sms         SMS management
  monitoring  Status monitoring
  dhcp        DHCP configuration
  
Options:
  --url <URL>     Device URL [default: http://192.168.8.1]
  --timeout <N>   Request timeout in seconds [default: 30]
  --retries <N>   Max retry attempts [default: 3]
  --output <FMT>  Output format (table, json) [default: table]
```

## Examples

### Device Operations

```bash
# Get device information
huawei-dongle-cli device info

# Reboot the device
huawei-dongle-cli device reboot --confirm

# Power off the device
huawei-dongle-cli device power-off --confirm
```

### Network Management

```bash
# Show current network mode
huawei-dongle-cli network mode

# Set network to 4G only
huawei-dongle-cli network set-mode 4g-only

# Show current operator
huawei-dongle-cli network plmn
```

### Connection Monitoring

```bash
# Show connection status
huawei-dongle-cli monitoring status

# Output in JSON format
huawei-dongle-cli --output json monitoring status
```

### SMS Management

```bash
# List all SMS messages
huawei-dongle-cli sms list

# List unread messages only
huawei-dongle-cli sms list --unread

# Show SMS count
huawei-dongle-cli sms count

# Delete a message
huawei-dongle-cli sms delete 40001

# Mark message as read
huawei-dongle-cli sms mark-read 40001
```

### DHCP Configuration

```bash
# Show current DHCP settings
huawei-dongle-cli dhcp show

# Change gateway IP address
huawei-dongle-cli dhcp set-ip 192.168.62.1

# Update complete DHCP configuration
huawei-dongle-cli dhcp set \
    --ip 192.168.62.1 \
    --netmask 255.255.255.0 \
    --start-ip 192.168.62.100 \
    --end-ip 192.168.62.200 \
    --lease-time 86400 \
    --primary-dns 8.8.8.8 \
    --secondary-dns 8.8.4.4
```

### Custom Device URL

```bash
# Use a different device IP
huawei-dongle-cli --url http://192.168.62.1 device info

# With custom timeout
huawei-dongle-cli --url http://192.168.1.1 --timeout 60 monitoring status
```

## Output Formats

The CLI supports two output formats:

### Table Format (default)
```
$ huawei-dongle-cli device info
┌──────────────────┬─────────────────────────┐
│ Field            │ Value                   │
├──────────────────┼─────────────────────────┤
│ Device Name      │ E3372h-320              │
│ Serial Number    │ 1234567890ABCDEF        │
│ IMEI             │ 123456789012345         │
│ Hardware Version │ CL1E3372HM              │
│ Software Version │ 10.0.2.1                │
│ MAC Address      │ AA:BB:CC:DD:EE:FF       │
└──────────────────┴─────────────────────────┘
```

### JSON Format
```bash
$ huawei-dongle-cli --output json device info
{
  "device_name": "E3372h-320",
  "serial_number": "1234567890ABCDEF",
  "imei": "123456789012345",
  "hardware_version": "CL1E3372HM",
  "software_version": "10.0.2.1",
  "mac_address": "AA:BB:CC:DD:EE:FF"
}
```

## Network Modes

Supported network mode values for `set-mode` command:

- `auto` - Automatic mode selection
- `2g-only` - 2G only
- `3g-only` - 3G only  
- `4g-only` - 4G only
- `4g-3g-auto` - 4G preferred, 3G fallback
- `5g-4g-3g-auto` - 5G NSA preferred

## Used In Production

This CLI tool is used to manage fleets of Huawei LTE dongles powering the mobile proxy infrastructure at **[Scraping Fish API](https://scrapingfish.com)**.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.