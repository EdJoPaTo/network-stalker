use clap::{App, Arg};

pub struct RuntimeArguments {
    pub mqtt_server: String,
    pub mqtt_base_topic: String,
    pub mqtt_qos: i32,
    pub mqtt_retain: bool,
    pub mqtt_file_persistence: bool,
    pub verbose: bool,
    pub hostnames: Vec<String>,
}

pub fn build_cli() -> App<'static, 'static> {
    App::new("Network Stalker")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Tries to reach hosts on the network and reports their online status to MQTT")
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
        .arg(Arg::with_name("MQTT File persistence")
            .short("p")
            .long("file-persistence")
            .help("When enabled the MQTT persistence is done via files within the working directory. Enabling this is more reliable.")
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
    let matches = build_cli().get_matches();

    let mqtt_server = matches
        .value_of("MQTT Server")
        .expect("MQTT Server could not be read from command line")
        .to_owned();

    let mqtt_base_topic = matches
        .value_of("MQTT Base Topic")
        .expect("MQTT Base Topic could not be read from command line")
        .to_owned();

    let mqtt_qos: i32 = matches
        .value_of("MQTT QoS")
        .and_then(|s| s.parse::<i32>().ok())
        .expect("MQTT QoS could not be read from command line. Make sure its 0, 1 or 2");

    let mqtt_retain = matches.is_present("MQTT Retain");

    let mqtt_file_persistence = matches.is_present("MQTT File persistence");

    let verbose = matches.is_present("verbose");

    let hostnames: Vec<String> = matches
        .values_of("hostnames")
        .expect("hostnames could not be read from command line")
        .map(|str| str.to_owned())
        .collect();

    RuntimeArguments {
        mqtt_server,
        mqtt_base_topic,
        mqtt_qos,
        mqtt_retain,
        mqtt_file_persistence,
        verbose,
        hostnames,
    }
}
