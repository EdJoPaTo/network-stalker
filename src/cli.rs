use clap::{App, Arg};

pub struct RuntimeArguments {
    pub mqtt_server: String,
    pub mqtt_base_topic: String,
    pub mqtt_retain: bool,
    pub hostnames: Vec<String>,
}

pub fn build_cli() -> App<'static, 'static> {
    App::new("Network Stalker")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("tries to reach hosts on the network and reports their online status to MQTT")
        .arg(Arg::with_name("MQTT Server")
            .short("s")
            .long("mqtt-server")
            .value_name("URI")
            .takes_value(true)
            .help("Specify the MQTT Server")
            .default_value("tcp://localhost:1883")
        )
        .arg(Arg::with_name("MQTT Base Topic")
            .short("b")
            .long("base-topic")
            .value_name("STRING")
            .takes_value(true)
            .help("MQTT Root Topic to publish")
            .default_value("network-stalker")
        )
        .arg(Arg::with_name("MQTT Retain")
            .short("r")
            .long("retain")
            .help("publish MQTT Messages with the retain flag")
        )
        .arg(Arg::with_name("hostnames")
            .multiple(true)
            .min_values(1)
            .required(true)
            .help("Hostnames to be checked for being reachable like '192.168.178.1' or 'fritz.box'")
        )
}

pub fn arguments() -> RuntimeArguments {
    let matches = build_cli().get_matches();

    let mqtt_server = matches
        .value_of("MQTT Server")
        .expect("MQTT Server could not be read from command line")
        .to_owned();

    let mqtt_base_topic = matches
        .value_of("MQTT Base Topic")
        .expect("MQTT Base Topic could not be read from command line")
        .to_owned();

    let mqtt_retain = matches.is_present("MQTT Retain");

    let hostnames: Vec<String> = matches
        .values_of("hostnames")
        .expect("hostnames could not be read from command line")
        .map(|str| str.to_owned())
        .collect();

    RuntimeArguments {
        mqtt_server,
        mqtt_base_topic,
        mqtt_retain,
        hostnames,
    }
}
