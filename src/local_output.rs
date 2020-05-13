use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

pub fn write_to_file(last_seen: &HashMap<String, String>, path: &str) -> std::io::Result<()> {
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

pub fn display_last_seen(last_seen: &HashMap<String, String>) {
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
