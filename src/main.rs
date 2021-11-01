use chrono::SecondsFormat;
use chrono::Utc;
use rumqttc::qos;
use std::collections::HashMap;
use std::thread;
use std::time;

mod cli;
mod mqtt;
mod nmap;

const LAST_ONLINE_MINUTES: [i64; 4] = [1, 3, 5, 15];

fn main() {
    let matches = cli::build().get_matches();

    let mqtt_host = matches
        .value_of("MQTT Server")
        .expect("MQTT Host could not be read from command line");

    let mqtt_port = matches
        .value_of("MQTT Port")
        .and_then(|s| s.parse::<u16>().ok())
        .expect("MQTT Port could not be read from command line");

    let mqtt_base_topic = matches
        .value_of("MQTT Base Topic")
        .expect("MQTT Base Topic could not be read from command line");

    let mqtt_qos = matches
        .value_of("MQTT QoS")
        .and_then(|s| s.parse::<u8>().ok())
        .and_then(|num| qos(num).ok())
        .expect("MQTT QoS could not be read from command line. Make sure its 0, 1 or 2");

    let mqtt_retain = matches.is_present("MQTT Retain");

    let verbose = matches.is_present("verbose");

    let hostnames = matches
        .values_of("hostnames")
        .expect("hostnames could not be read from command line")
        .collect::<Vec<_>>();

    let mut mqtt_cached_publisher =
        mqtt::CachedPublisher::new(mqtt_base_topic, mqtt_host, mqtt_port, mqtt_qos, mqtt_retain);

    let mut last_seen_online: HashMap<String, i64> = HashMap::new();

    let starttime = Utc::now().timestamp();
    loop {
        for hostname in &hostnames {
            check_host(
                mqtt_base_topic,
                verbose,
                &mut mqtt_cached_publisher,
                &mut last_seen_online,
                starttime,
                hostname,
            );
        }

        // Loop worked out fine -> everything is fine -> 2
        mqtt_cached_publisher.publish(&format!("{}/connected", &mqtt_base_topic), "2");

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
    let host_topic = format!("{}/hosts/{}", mqtt_base_topic, hostname);

    let reachable = nmap::is_reachable(hostname);
    publish_reachable(mqtt_client, &host_topic, "now", &Reachable::from(reachable));

    let now = Utc::now();
    let unix = now.timestamp();
    let iso = now.to_rfc3339_opts(SecondsFormat::Secs, true);

    if verbose {
        println!("{} reachable: {:>5} {}", &iso, reachable, hostname);
    }

    if reachable {
        last_seen_online.insert(hostname.to_owned(), unix);
    }

    let last_online = last_seen_online.get(hostname);

    for within_minutes in &LAST_ONLINE_MINUTES {
        let topic_suffix = format!("{}min", within_minutes);
        let min_timestamp = unix - (within_minutes * 60);

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
    let topic = format!("{}/{}", topic_base, topic_suffix);
    let payload = match reachable {
        Reachable::Online => "online",
        Reachable::Offline => "offline",
        Reachable::Unknown => "unknown",
    };

    mqtt_client.publish(&topic, payload);
}
