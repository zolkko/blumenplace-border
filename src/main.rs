
#![crate_name = "blumenplace-border"]
#![crate_type = "bin"]

#![desc = "blumenplace-border application"]
#![license = "GPLv3"]


#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate getopts;


mod settings;


fn run_app(settings: &settings::Settings) -> () {
    info!("Start application. Front-url: {}, Port: {}.", settings.front_url, settings.port)
}


fn main () -> () {
    info!("{} application initialized.", "test");
    match settings::parse_args() {
        Some(settings) => {
            run_app(&settings);
        }
        None => { }
    }
}

