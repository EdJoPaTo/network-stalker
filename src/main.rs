#![forbid(unsafe_code)]

use chrono::SecondsFormat;
use chrono::Utc;
use clap::Parser;
use rumqttc::qos;
use std::collections::HashMap;
use std::thread;
use std::time;

mod cli;
mod mqtt;
mod nmap;

const LAST_ONLINE_MINUTES: [i64; 4] = [1, 3, 5, 15];

fn main() {
    let matches = cli::Cli::parse();

    let mut mqtt_cached_publisher = mqtt::CachedPublisher::new(
        matches.base_topic.clone(),
        matches.broker,
        matches.port,
        matches.username,
        matches.password,
        qos(matches.qos).expect("Should be valid QoS"),
        matches.retain,
    );

    let mut last_seen_online: HashMap<String, i64> = HashMap::new();

    let starttime = Utc::now().timestamp();
    loop {
        for hostname in &matches.hostnames {
            check_host(
                &matches.base_topic,
                matches.verbose,
                &mut mqtt_cached_publisher,
                &mut last_seen_online,
                starttime,
                hostname,
            );
        }

        // Loop worked out fine -> everything is fine -> 2
        mqtt_cached_publisher.publish(&format!("{}/connected", matches.base_topic), "2");

        thread::sleep(time::Duration::from_secs(30));
    }
}

enum Reachable {
    Online,
    Offline,
    Unknown,
}

impl From<bool> for Reachable {
    fn from(input: bool) -> Self {
        if input {
            Self::Online
        } else {
            Self::Offline
        }
    }
}

fn check_host(
    mqtt_base_topic: &str,
    verbose: bool,
    mqtt_client: &mut mqtt::CachedPublisher,
    last_seen_online: &mut HashMap<String, i64>,
    starttime: i64,
    hostname: &str,
) {
    let host_topic = format!("{mqtt_base_topic}/hosts/{hostname}");

    let reachable = nmap::is_reachable(hostname);
    publish_reachable(mqtt_client, &host_topic, "now", &Reachable::from(reachable));

    let now = Utc::now();
    let unix = now.timestamp();

    if verbose {
        let iso = now.to_rfc3339_opts(SecondsFormat::Secs, true);
        println!("{iso} reachable: {reachable:>5} {hostname}");
    }

    if reachable {
        last_seen_online.insert(hostname.to_owned(), unix);
    }

    let last_online = last_seen_online.get(hostname);

    for within_minutes in &LAST_ONLINE_MINUTES {
        let topic_suffix = format!("{within_minutes}min");
        let min_timestamp = unix - (within_minutes * 60);

        #[allow(clippy::option_if_let_else)]
        let online_within_timespan = if let Some(last_online) = last_online {
            // Was seen sometime -> within timespan?
            Reachable::from(*last_online > min_timestamp)
        } else if starttime < min_timestamp {
            // Wasnt seen and network-stalker is running longer long enough so it should have seen it otherwise
            Reachable::Offline
        } else {
            // Wasnt seen but network-stalker is not running as long as the required timespan
            Reachable::Unknown
        };

        publish_reachable(
            mqtt_client,
            &host_topic,
            &topic_suffix,
            &online_within_timespan,
        );
    }
}

fn publish_reachable(
    mqtt_client: &mut mqtt::CachedPublisher,
    topic_base: &str,
    topic_suffix: &str,
    reachable: &Reachable,
) {
    let topic = format!("{topic_base}/{topic_suffix}");
    let payload = match reachable {
        Reachable::Online => "online",
        Reachable::Offline => "offline",
        Reachable::Unknown => "unknown",
    };

    mqtt_client.publish(&topic, payload);
}
