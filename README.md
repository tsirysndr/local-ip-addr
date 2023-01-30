<h1>local-ip-addr</h1>
<p>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
  <a href="https://crates.io/crates/local-ip-addr" target="_blank">
    <img src="https://img.shields.io/crates/v/local-ip-addr.svg" />
  </a>
  
  <a href="https://crates.io/crates/local-ip-addr" target="_blank">
    <img src="https://img.shields.io/crates/dr/local-ip-addr" />
  </a>
  
  <a href="https://docs.rs/local-ip-addr" target="_blank">
    <img src="https://docs.rs/local-ip-addr/badge.svg" />
  </a>
</p>

A simple library for getting the local IP address of the current host.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
local-ip-addr = "0.1"
```

## Example

```rust
use local_ip_addr::get_local_ip_address;

fn main() {
    let ip_addr = get_local_ip_address().unwrap();
    println!("Local IP address: {}", ip_addr);
}
```

