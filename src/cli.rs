use clap::{App, AppSettings, Arg};
use rumqttc::{qos, QoS};

pub struct RuntimeArguments {
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub mqtt_base_topic: String,
    pub mqtt_qos: QoS,
    pub mqtt_retain: bool,
    pub verbose: bool,
    pub hostnames: Vec<String>,
}

pub fn build() -> App<'static, 'static> {
    App::new("Network Stalker")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Tries to reach hosts on the network and reports their online status to MQTT")
        .global_setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("MQTT Server")
            .short("h")
            .long("host")
            .value_name("HOST")
            .takes_value(true)
            .help("Host on which the MQTT Broker is running")
            .default_value("localhost")
        )
        .arg(Arg::with_name("MQTT Port")
            .short("p")
            .long("port")
            .value_name("INT")
            .takes_value(true)
            .help("Port on which the MQTT Broker is running")
            .default_value("1883")
        )
        .arg(Arg::with_name("MQTT Base Topic")
            .short("t")
            .long("base-topic")
            .value_name("STRING")
            .takes_value(true)
            .help("MQTT Root Topic to publish to")
            .default_value("network-stalker")
        )
        .arg(Arg::with_name("MQTT QoS")
            .short("q")
            .long("qos")
            .value_name("INT")
            .takes_value(true)
            .help("Define the Quality of Service for the MQTT Messages (0, 1 or 2)")
            .default_value("2")
        )
        .arg(Arg::with_name("MQTT Retain")
            .short("r")
            .long("retain")
            .help("Publish MQTT Messages with the retain flag")
        )
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Show network check results on stdout")
        )
        .arg(Arg::with_name("hostnames")
            .multiple(true)
            .min_values(1)
            .required(true)
            .help("Hostnames to be checked for being reachable like '192.168.178.1' or 'fritz.box'")
        )
}

pub fn arguments() -> RuntimeArguments {
    let matches = build().get_matches();

    let mqtt_host = matches
        .value_of("MQTT Server")
        .expect("MQTT Host could not be read from command line")
        .to_owned();

    let mqtt_port = matches
        .value_of("MQTT Port")
        .and_then(|s| s.parse::<u16>().ok())
        .expect("MQTT Port could not be read from command line");

    let mqtt_base_topic = matches
        .value_of("MQTT Base Topic")
        .expect("MQTT Base Topic could not be read from command line")
        .to_owned();

    let mqtt_qos = matches
        .value_of("MQTT QoS")
        .and_then(|s| s.parse::<u8>().ok())
        .and_then(|num| qos(num).ok())
        .expect("MQTT QoS could not be read from command line. Make sure its 0, 1 or 2");

    let mqtt_retain = matches.is_present("MQTT Retain");

    let verbose = matches.is_present("verbose");

    let hostnames: Vec<String> = matches
        .values_of("hostnames")
        .expect("hostnames could not be read from command line")
        .map(std::string::ToString::to_string)
        .collect();

    RuntimeArguments {
        mqtt_host,
        mqtt_port,
        mqtt_base_topic,
        mqtt_qos,
        mqtt_retain,
        verbose,
        hostnames,
    }
}
