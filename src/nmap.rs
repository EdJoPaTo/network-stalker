use std::process::Command;
use std::string::String;

pub fn is_reachable(address: &str) -> bool {
    let nmap_result = Command::new("nmap")
        .arg("-oX")
        .arg("-")
        .arg("-sn")
        .arg(address)
        .output()
        .expect("nmap scan failed");

    let stdout_string = String::from_utf8_lossy(&nmap_result.stdout);
    !stdout_string.contains("0 hosts up")
}
