use paho_mqtt::{
    Client, ConnectOptionsBuilder, CreateOptionsBuilder, MessageBuilder, MqttError, PersistenceType,
};
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

    pub fn publish(
        &mut self,
        topic: &str,
        payload: &str,
        qos: i32,
        retain: bool,
    ) -> Result<(), MqttError> {
        let before = self.cache.insert(topic.to_owned(), payload.to_owned());

        if before != Some(payload.to_owned()) {
            publish(&self.client, &topic, payload, qos, retain)
        } else {
            Ok(())
        }
    }
}

pub fn connect(
    mqtt_server: &str,
    base_topic_name: &str,
    qos: i32,
    retain: bool,
    file_persistence: bool,
) -> Result<Client, MqttError> {
    let connect_topic = format!("{}/connected", base_topic_name);

    let create_options = CreateOptionsBuilder::new()
        .server_uri(mqtt_server)
        .persistence(if file_persistence {
            PersistenceType::File
        } else {
            PersistenceType::None
        })
        .finalize();

    let client = Client::new(create_options)?;

    let last_will = MessageBuilder::new()
        .topic(&connect_topic)
        .qos(qos)
        .retained(retain)
        .payload("0")
        .finalize();

    let connection_options = ConnectOptionsBuilder::new()
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
        .will_message(last_will)
        .finalize();

    client.connect(connection_options)?;

    publish(&client, &connect_topic, "1", qos, retain)?;

    Ok(client)
}

fn publish(
    client: &Client,
    topic: &str,
    payload: &str,
    qos: i32,
    retain: bool,
) -> Result<(), MqttError> {
    let msg = MessageBuilder::new()
        .topic(topic)
        .qos(qos)
        .retained(retain)
        .payload(payload)
        .finalize();

    client.publish(msg)
}
