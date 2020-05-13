use chrono::Utc;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::process::Command;
use std::string::String;

pub fn run_check(last_seen: &mut HashMap<String, String>, address_range: &str) {
    let timestamp = Utc::now().to_rfc3339();
    println!("run check at {}", timestamp);

    let seen = nmap_scan(address_range);
    println!("have seen {:3} hosts within {}", seen.len(), address_range);

    for hostname in seen {
        last_seen.insert(hostname, timestamp.to_string());
    }
}

pub fn nmap_scan(address_range: &str) -> HashSet<String> {
    let nmap_result = Command::new("nmap")
        .arg("-oX")
        .arg("-")
        .arg("-sn")
        .arg(address_range)
        .output()
        .expect("nmap scan failed");

    let stdout_string = String::from_utf8_lossy(&nmap_result.stdout);

    let re: Regex = Regex::new(r#"<hostname name="([^"]+)"#).unwrap();

    let mut seen = HashSet::new();

    for cap in re.captures_iter(&stdout_string) {
        let hostname_match = &cap[1];
        let hostname = hostname_match.to_owned();
        seen.insert(hostname);
    }

    seen
}
