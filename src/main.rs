use std::collections::HashMap;
use std::string::String;
use std::thread;
use std::time;

mod nmap;
mod local_output;

fn main() {
    let mut last_seen: HashMap<String, String> = HashMap::new();
    let address_range = "192.168.178.0/24";
    let filename = "/tmp/network-stalker";

    loop {
        nmap::run_check(&mut last_seen, address_range);
        local_output::write_to_file(&last_seen, filename).expect("write file failed");
        local_output::display_last_seen(&last_seen);

        thread::sleep(time::Duration::from_secs(30));
    }
}
