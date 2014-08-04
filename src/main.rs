
#![crate_id = "blumenplace-border#0.0.1"]
#![crate_type = "bin"]

#![desc = "blumenplace-border application"]
#![license = "GPLv3"]


#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate getopts;


use std::io::signal;
use std::sync::Arc;


mod settings;


fn udp_service(shared_settings: Arc<settings::Settings>) -> () {
    info!("UDP service starting. UDP port {} will be used for incoming requests.", shared_settings.port);

    info!("UDP service finished.");
}


fn http_service(shared_settings: Arc<settings::Settings>) -> () {
    info!("HTTP RESTFull client starting. Blumenplace url service is located at \"{}\" url.", shared_settings.front_url);

    info!("HTTP RESTFull client finished.");
}


fn run_app(settings: settings::Settings) -> () {
    info!("Start application.");

    let shared_settings = Arc::new(settings);

    let udp_shared_settings = shared_settings.clone();
    spawn(proc() { udp_service(udp_shared_settings) });

    let http_shared_settings = shared_settings.clone();
    spawn(proc() { http_service(http_shared_settings) });

    let mut listener = signal::Listener::new();

    match listener.register(signal::Interrupt) {
        Err(err) => error!("Failed to register {} listener. Reason: {}.", signal::Interrupt, err),
        _ => {}
    };

    loop {
        match listener.rx.recv() {
            signal::Interrupt => break,
            _ => ()
        }
    }

    info!("Application finished.");
}


fn main () -> () {
    info!("Application initialized.");
    match settings::parse_args() {
        Some(settings) => { run_app(settings); }
        None => { }
    }
    info!("Application finished");
}

