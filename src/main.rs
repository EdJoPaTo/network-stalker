use chrono::SecondsFormat;
use chrono::Utc;
use std::collections::HashMap;
use std::thread;
use std::time;

mod cli;
mod mqtt;
mod nmap;

const LAST_ONLINE_MINUTES: [i64; 4] = [1, 3, 5, 15];

fn main() {
    let args = cli::arguments();

    let mut mqtt_cached_publisher = mqtt::MqttCachedPublisher::new(
        mqtt::connect(
            &args.mqtt_server,
            &args.mqtt_base_topic,
            args.mqtt_qos,
            args.mqtt_retain,
            args.mqtt_file_persistence,
        )
        .expect("failed to connect to MQTT server"),
    );

    let mut last_seen_online: HashMap<String, i64> = HashMap::new();

    let starttime = Utc::now().timestamp();
    loop {
        for hostname in args.hostnames.iter() {
            check_host(
                &mut mqtt_cached_publisher,
                &args.mqtt_base_topic,
                args.mqtt_qos,
                args.mqtt_retain,
                &mut last_seen_online,
                starttime,
                &hostname,
            );
        }

        // Loop worked out fine -> everything is fine -> 2
        mqtt_cached_publisher
            .publish(
                &format!("{}/connected", &args.mqtt_base_topic),
                "2",
                args.mqtt_qos,
                args.mqtt_retain,
            )
            .expect("failed to update connected status");

        thread::sleep(time::Duration::from_secs(30));
    }
}

fn check_host(
    mqtt_client: &mut mqtt::MqttCachedPublisher,
    mqtt_topic_base: &str,
    mqtt_qos: i32,
    mqtt_retain: bool,
    last_seen_online: &mut HashMap<String, i64>,
    starttime: i64,
    hostname: &str,
) {
    let host_topic = format!("{}/hosts/{}", mqtt_topic_base, hostname);

    let reachable = nmap::is_reachable(hostname);
    publish_reachable(
        mqtt_client,
        &host_topic,
        "now",
        mqtt_qos,
        mqtt_retain,
        Some(reachable),
    );

    let now = Utc::now();
    let unix = now.timestamp();
    let iso = now.to_rfc3339_opts(SecondsFormat::Secs, true);

    println!("{} reachable: {:>5} {}", &iso, reachable, hostname);

    if reachable {
        last_seen_online.insert(hostname.to_owned(), unix);
    }

    let last_online = last_seen_online.get(hostname);

    for within_minutes in LAST_ONLINE_MINUTES.iter() {
        let topic_suffix = format!("{}min", within_minutes);
        let min_timestamp = unix - (within_minutes * 60);

        let online_within_timespan = if last_online.is_some() {
            last_online.map(|last| *last > min_timestamp)
        } else if starttime < min_timestamp {
            Some(false)
        } else {
            None
        };

        publish_reachable(
            mqtt_client,
            &host_topic,
            &topic_suffix,
            mqtt_qos,
            mqtt_retain,
            online_within_timespan,
        )
    }
}

fn publish_reachable(
    mqtt_client: &mut mqtt::MqttCachedPublisher,
    topic_base: &str,
    topic_suffix: &str,
    qos: i32,
    retain: bool,
    reachable: Option<bool>,
) {
    let topic = format!("{}/{}", topic_base, topic_suffix);
    let payload = match reachable {
        Some(true) => "online",
        Some(false) => "offline",
        None => "unknown",
    };

    mqtt_client
        .publish(&topic, payload, qos, retain)
        .expect("publish host check to mqtt failed");
}
