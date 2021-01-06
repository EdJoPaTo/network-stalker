use rumqttc::{Connection, LastWill, MqttOptions, QoS};
use std::collections::HashMap;
use std::string::String;
use std::thread::{self, sleep};
use std::time::Duration;

pub struct CachedPublisher {
    client: rumqttc::Client,
    publish_history: HashMap<String, String>,
    qos: QoS,
    retain: bool,
}

impl CachedPublisher {
    pub fn new(base_topic: &str, host: &str, port: u16, qos: QoS, retain: bool) -> Self {
        let last_will_topic = format!("{}/connected", base_topic);

        let mut mqttoptions = MqttOptions::new(base_topic, host, port);
        mqttoptions.set_last_will(LastWill::new(&last_will_topic, "0", qos, retain));

        let (mut client, connection) = rumqttc::Client::new(mqttoptions, 10);

        client
            .publish(last_will_topic, qos, retain, "1")
            .expect("failed to publish connected");

        thread::Builder::new()
            .name("mqtt connection".into())
            .spawn(move || thread_logic(connection))
            .expect("failed to spawn mqtt thread");

        Self {
            retain,
            qos,
            client,
            publish_history: HashMap::new(),
        }
    }

    pub fn publish(&mut self, topic: &str, payload: &str) {
        let last_value = self
            .publish_history
            .insert(topic.to_owned(), payload.to_owned());

        if last_value != Some(payload.to_owned()) {
            self.client
                .publish(topic, self.qos, self.retain, payload)
                .expect("failed to publish to mqtt");
        }
    }
}

fn thread_logic(mut connection: Connection) {
    for notification in connection.iter() {
        match notification {
            Ok(rumqttc::Event::Outgoing(rumqttc::Outgoing::Disconnect)) => {
                break;
            }
            Ok(_) => {}
            Err(err) => {
                println!("MQTT Connection Error: {}", err);
                sleep(Duration::from_secs(1));
            }
        };
    }
}
