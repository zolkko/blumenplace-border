
#![crate_id = "blumenplace-border#0.0.1"]
#![crate_type = "bin"]

#![desc = "blumenplace-border application"]
#![license = "GPLv3"]


extern crate getopts;


mod settings;


fn main () -> () {
    match settings::parse_args() {
        Some(settings) => { println!("Start application. Front-url: {}, Port: {}.", settings.front_url, settings.port) }
        None => { return; }
    }
}

