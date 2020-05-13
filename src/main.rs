use chrono::Utc;
use paho_mqtt::Client;
use std::collections::HashMap;
use std::thread;
use std::time;

mod mqtt;
mod nmap;

#[cfg(debug_assertions)]
const RETAIN: bool = false;
#[cfg(not(debug_assertions))]
const RETAIN: bool = true;

fn main() {
    // TODO: get from command line arguments
    let mqtt_server = "tcp://etoPiServer:1883";
    let mqtt_base_topic = "network-stalker";

    let to_be_checked = ["etoNUC", "etoWindoof", "etoPhone", "etoPad"];

    let mqtt_client = mqtt::connect(mqtt_server, mqtt_base_topic, RETAIN)
        .expect("failed to connect to MQTT server");

    let mut last_known_state: HashMap<String, bool> = HashMap::new();

    loop {
        for &hostname in to_be_checked.iter() {
            check_host(
                &mqtt_client,
                &mqtt_base_topic,
                &mut last_known_state,
                &hostname,
            );
        }

        thread::sleep(time::Duration::from_secs(30));
    }
}

fn check_host(
    mqtt_client: &Client,
    mqtt_base_topic: &str,
    last_known_state: &mut HashMap<String, bool>,
    hostname: &str,
) {
    let topic = format!("{}/hosts/{}/now", mqtt_base_topic, hostname);
    // TODO: implement now / 1 min / 5 min / 15 min

    let reachable = nmap::is_reachable(&hostname);

    let before = last_known_state.insert(hostname.to_owned(), reachable);
    let changed = before.is_none() || before.unwrap() != reachable;

    let timestamp = Utc::now().to_rfc3339();
    println!(
        "{} reachable: {:>5} (changed: {:>5}) {}",
        &timestamp, reachable, changed, &hostname
    );

    if changed {
        let payload = if reachable { b"1" } else { b"0" };
        mqtt::publish(mqtt_client, topic, payload, RETAIN)
            .expect("failed to publish host check to mqtt");
    }
}
