## local-ip-addr

A simple library for getting the local IP address of the current host.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
local-ip-addr = "0.1"
```

## Example

```rust
use local_ip_addr::get_local_ip_addr;

fn main() {
    let ip_addr = get_local_ip_addr().unwrap();
    println!("Local IP address: {}", ip_addr);
}
```

