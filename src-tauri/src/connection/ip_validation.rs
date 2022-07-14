// TODO: add custom error structures/enums for address and port validation, as well as for overall IP validation.

const NETWORK_TABLES_PORT: &str = "1735";

fn validate_ip_port(port: &str) -> bool {
    port == NETWORK_TABLES_PORT
}

fn validate_ip_address(address: &str) -> bool {
    let numbers: Vec<&str> = address.split(".").collect();
    let are_all_u8: bool = numbers.iter().all(|n: &&str| n.parse::<u8>().is_ok());
    numbers.len() == 4 && are_all_u8 && numbers[0] == "10" && numbers[3] == "2"
}

pub fn validate_ip(ip: &str) -> Result<(), String> {
    if let [address, port] = ip.split(":").collect::<Vec<&str>>().as_slice() {
        let is_port_valid: bool = validate_ip_port(port);
        let is_address_valid: bool = validate_ip_address(address);

        if !is_address_valid {
            return Err(String::from(""));
        }

        if !is_port_valid {
            return Err(String::from(""));
        }
    }

    // The pattern match fails when the IP given doesn't contain an address and a port
    Err(String::from("Robot IP must contain an address and a port"))
}
