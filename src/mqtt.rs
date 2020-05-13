use paho_mqtt::{Client, ConnectOptionsBuilder, MessageBuilder, MqttError};
use std::string::String;
use std::time::Duration;

pub fn connect(mqtt_server: &str, device_name: &str, retain: bool) -> Result<Client, MqttError> {
    let connect_topic_string = format!("{}/connected", device_name);
    let connect_topic: &str = connect_topic_string.as_ref();

    let client = Client::new(mqtt_server)?;

    let last_will = MessageBuilder::new()
        .topic(connect_topic)
        .qos(0)
        .retained(retain)
        .payload("0")
        .finalize();

    let connection_options = ConnectOptionsBuilder::new()
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
        .will_message(last_will)
        .finalize();

    client.connect(connection_options)?;

    publish(&client, connect_topic_string, b"2", retain)?;

    Ok(client)
}

pub fn publish(
    client: &Client,
    topic: String,
    payload: &[u8],
    retain: bool,
) -> Result<(), MqttError> {
    let msg = MessageBuilder::new()
        .topic(topic)
        .qos(0)
        .retained(retain)
        .payload(payload)
        .finalize();

    client.publish(msg)
}

// TODO: fn publishCached
// when the topic was never published or the value is different from last time publish it
// if it is the same as last time skip it -> less mqtt traffic, it is retained anyway in production
