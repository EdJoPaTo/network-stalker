use chrono::Utc;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::string::String;
use std::thread;
use std::time;

fn main() {
    let mut last_seen: HashMap<String, String> = HashMap::new();

    loop {
        run_check(&mut last_seen, "192.168.178.0/24");
        write_to_file(&last_seen, "/tmp/network-stalker").expect("write file failed");
        display_last_seen(&last_seen);

        thread::sleep(time::Duration::from_secs(30));
    }
}

fn run_check(last_seen: &mut HashMap<String, String>, address_range: &str) {
    let timestamp = Utc::now().to_rfc3339();
    println!("run check at {}", timestamp);

    let seen = nmap_scan(address_range);
    println!("have seen {:3} hosts within {}", seen.len(), address_range);

    for hostname in seen {
        last_seen.insert(hostname, timestamp.to_string());
    }
}

fn write_to_file(last_seen: &HashMap<String, String>, path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;

    let lines: Vec<_> = last_seen
        .keys()
        .map(|hostname| {
            format!(
                "{} {}",
                last_seen.get(hostname).expect("someone stole a hostname"),
                hostname
            )
        })
        .collect();

    let content = lines.join("\n");
    f.write_all(content.as_bytes())
}

fn display_last_seen(last_seen: &HashMap<String, String>) {
    let max_hostname_length = last_seen
        .keys()
        .map(|hostname| hostname.len())
        .max()
        .expect("found no hostnames");

    let mut hostnames: Vec<_> = last_seen.keys().collect();
    hostnames.sort_by_key(|hostname| hostname.to_lowercase());

    for hostname in hostnames {
        let last_seen_timestamp = last_seen
            .get(hostname)
            .expect("someone stole the host list");

        println!(
            "{:>width$} {}",
            hostname,
            last_seen_timestamp,
            width = max_hostname_length + 1
        );
    }
}

fn nmap_scan(address_range: &str) -> HashSet<String> {
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
