use paho_mqtt::{Client, ConnectOptionsBuilder, MessageBuilder, MqttError};
use std::collections::HashMap;
use std::string::String;
use std::time::Duration;

pub struct MqttCachedPublisher {
    client: Client,
    cache: Box<HashMap<String, String>>,
}

impl MqttCachedPublisher {
    pub fn new(client: Client) -> MqttCachedPublisher {
        MqttCachedPublisher {
            client,
            cache: Box::new(HashMap::new()),
        }
    }

    pub fn publish(&mut self, topic: &str, payload: &str, retain: bool) -> Result<(), MqttError> {
        let before = self.cache.insert(topic.to_owned(), payload.to_owned());

        if before != Some(payload.to_owned()) {
            publish(&self.client, &topic, payload, retain)
        } else {
            Ok(())
        }
    }
}

pub fn connect(
    mqtt_server: &str,
    base_topic_name: &str,
    retain: bool,
) -> Result<Client, MqttError> {
    let connect_topic = format!("{}/connected", base_topic_name);

    let client = Client::new(mqtt_server)?;

    let last_will = MessageBuilder::new()
        .topic(&connect_topic)
        .qos(0)
        .retained(retain)
        .payload("0")
        .finalize();

    let connection_options = ConnectOptionsBuilder::new()
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
        .will_message(last_will)
        .finalize();

    client.connect(connection_options)?;

    publish(&client, &connect_topic, "1", retain)?;

    Ok(client)
}

fn publish(client: &Client, topic: &str, payload: &str, retain: bool) -> Result<(), MqttError> {
    let msg = MessageBuilder::new()
        .topic(topic)
        .qos(0)
        .retained(retain)
        .payload(payload)
        .finalize();

    client.publish(msg)
}
