use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Cli {
    /// Host on which the MQTT Broker is running
    #[arg(
        long,
        short,
        env = "MQTT_BROKER",
        value_hint = ValueHint::Hostname,
        value_name = "HOST",
        default_value = "localhost",
    )]
    pub broker: String,

    /// Port on which the MQTT Broker is running
    #[arg(
        long,
        short,
        env = "MQTT_PORT",
        value_hint = ValueHint::Other,
        value_name = "INT",
        default_value_t = 1883,
    )]
    pub port: u16,

    /// Username to access the MQTT broker.
    ///
    /// Anonymous access when not supplied.
    #[arg(
        long,
        short,
        env = "MQTT_USERNAME",
        value_hint = ValueHint::Username,
        value_name = "STRING",
        requires = "password",
    )]
    pub username: Option<String>,

    /// Password to access the MQTT broker.
    ///
    /// Passing the password via command line is insecure as the password can be read from the history!
    #[arg(
        long,
        env = "MQTT_PASSWORD",
        value_hint = ValueHint::Other,
        value_name = "STRING",
        hide_env_values = true,
        requires = "username",
    )]
    pub password: Option<String>,

    /// MQTT Root Topic to publish to
    #[arg(
        short = 't',
        long,
        env = "MQTT_BASE_TOPIC",
        value_hint = ValueHint::Other,
        value_name = "STRING",
        default_value = "network-stalker",
    )]
    pub base_topic: String,

    /// Define the Quality of Service for the MQTT Messages
    #[arg(
        short,
        long,
        env = "MQTT_QOS",
        value_hint = ValueHint::Other,
        value_name = "INT",
        value_parser = ["0", "1", "2"],
        default_value_t = 2,
    )]
    pub qos: u8,

    /// Publish MQTT Messages with the retain flag
    #[arg(short, long, env = "MQTT_RETAIN")]
    pub retain: bool,

    /// Show network check results on stdout
    #[arg(short, long)]
    pub verbose: bool,

    /// Hostnames to be checked for being reachable like '192.168.178.1' or 'fritz.box'
    #[arg(
        value_hint = ValueHint::Hostname,
        value_name = "HOST",
    )]
    pub hostnames: Vec<String>,
}

#[test]
fn verify() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
