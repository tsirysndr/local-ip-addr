use local_ip_addr::get_local_ip_address;

fn main() {
    let ipv4 = get_local_ip_address().unwrap();
    println!("Local IP address: {}", ipv4);
}
