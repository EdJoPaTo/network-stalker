use chrono::SecondsFormat;
use chrono::Utc;
use std::collections::HashMap;
use std::thread;
use std::time;

mod mqtt;
mod nmap;

#[cfg(debug_assertions)]
const RETAIN: bool = false;
#[cfg(not(debug_assertions))]
const RETAIN: bool = true;

const LAST_ONLINE_MINUTES: [i64; 4] = [1, 3, 5, 15];

fn main() {
    // TODO: get from command line arguments
    let mqtt_server = "tcp://etoPiServer:1883";
    let mqtt_base_topic = "network-stalker";

    let to_be_checked = ["etoNUC", "etoWindoof", "etoPhone", "etoPad"];

    let mut mqtt_cached_publisher = mqtt::MqttCachedPublisher::new(
        mqtt::connect(mqtt_server, mqtt_base_topic, RETAIN)
            .expect("failed to connect to MQTT server"),
    );

    let mut last_seen_online: HashMap<String, i64> = HashMap::new();

    let starttime = Utc::now().timestamp();
    loop {
        for &hostname in to_be_checked.iter() {
            check_host(
                &mut mqtt_cached_publisher,
                &mqtt_base_topic,
                &mut last_seen_online,
                starttime,
                &hostname,
            );
        }

        // Loop worked out fine -> everything is fine -> 2
        mqtt_cached_publisher
            .publish(&format!("{}/connected", mqtt_base_topic), "2", RETAIN)
            .expect("failed to update connected status");

        thread::sleep(time::Duration::from_secs(30));
    }
}

fn check_host(
    mqtt_client: &mut mqtt::MqttCachedPublisher,
    mqtt_topic_base: &str,
    last_seen_online: &mut HashMap<String, i64>,
    starttime: i64,
    hostname: &str,
) {
    let host_topic = format!("{}/hosts/{}", mqtt_topic_base, hostname);

    let reachable = nmap::is_reachable(hostname);
    publish_reachable(mqtt_client, &host_topic, "now", Some(reachable));

    let now = Utc::now();
    let unix = now.timestamp();
    let iso = now.to_rfc3339_opts(SecondsFormat::Secs, true);

    println!("{} reachable: {:>5} {}", &iso, reachable, hostname);

    if reachable {
        last_seen_online.insert(hostname.to_owned(), unix);
    }

    let last_online = last_seen_online.get(hostname);

    for minutes in LAST_ONLINE_MINUTES.iter() {
        publish_seen_within(
            mqtt_client,
            &host_topic,
            last_online,
            starttime,
            *minutes,
            unix,
        );
    }
}

fn publish_seen_within(
    mqtt_client: &mut mqtt::MqttCachedPublisher,
    topic_base: &str,
    last_online: Option<&i64>,
    starttime: i64,
    within_minutes: i64,
    unix_timestamp_now: i64,
) {
    let topic_suffix = format!("{}min", within_minutes);

    let min_timestamp = unix_timestamp_now - (within_minutes * 60);

    let online_within_timespan = if last_online.is_some() {
        last_online.map(|last| *last > min_timestamp)
    } else if starttime < min_timestamp {
        Some(false)
    } else {
        None
    };

    publish_reachable(
        mqtt_client,
        topic_base,
        &topic_suffix,
        online_within_timespan,
    )
}

fn publish_reachable(
    mqtt_client: &mut mqtt::MqttCachedPublisher,
    topic_base: &str,
    topic_suffix: &str,
    reachable: Option<bool>,
) {
    let topic = format!("{}/{}", topic_base, topic_suffix);
    let payload = match reachable {
        Some(true) => "online",
        Some(false) => "offline",
        None => "unknown",
    };

    mqtt_client
        .publish(&topic, payload, RETAIN)
        .expect("publish host check to mqtt failed");
}
