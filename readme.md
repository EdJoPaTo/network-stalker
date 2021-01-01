# Network Stalker
![Rust](https://github.com/EdJoPaTo/network-stalker/workflows/Rust/badge.svg)

This tool is my first attempt of writing Rust.
It is probably more "trial and error" than well written code.

The basic idea is to ping devices on the network and report the status to MQTT.
This can be used for home automation.
When a certain device was online within the last 15 minutes the user is probably at home.

In order to check for a device `nmap` is used over `ping` as it had better results so far.
Running this tool as root will improve the accuracy on the cost of security (running tools as root is never a good idea).

## MQTT Topics

The general topic looks like this: `<base-topic>/hosts/<hostname>/<timespan>`.
Timespan is `now` `1min` `3min` `5min` `15min`.
On this topic the values `online` `offline` or `unknown` are published.

For example the host `fritz.box` was seen within 5 minutes so `online` is published to `network-stalker/hosts/fritz.box/5min`.

If a host is seen all timespans get `online` published as the host was seen within each timespan.
When a host wasnt seen for 3 minutes but was online before the `5min` and `15min` topic still are `online`.

On restart `unknown` is published when the device is not online.
Once the tool is running longer than 5min the `5min` topic can publish `offline` as the tool was there the whole time.
This prevents a wrong action when the tool was offline or is restarted.

Only changes are published.
This is helpful when doing something when someone arrives or someone is gone for some time.

## Usage

```plaintext
Network Stalker 0.3.0
EdJoPaTo <rust-package@edjopato.de>
Tries to reach hosts on the network and reports their online status to MQTT

USAGE:
    network-stalker [FLAGS] [OPTIONS] <hostnames>...

FLAGS:
    -r, --retain     Publish MQTT Messages with the retain flag
        --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show network check results on stdout

OPTIONS:
    -t, --base-topic <STRING>    MQTT Root Topic to publish to [default: network-stalker]
    -p, --port <INT>             Port on which the MQTT Broker is running [default: 1883]
    -q, --qos <INT>              Define the Quality of Service for the MQTT Messages (0, 1 or 2) [default: 2]
    -h, --host <HOST>            Host on which the MQTT Broker is running [default: localhost]

ARGS:
    <hostnames>...    Hostnames to be checked for being reachable like '192.168.178.1' or 'fritz.box'
```
