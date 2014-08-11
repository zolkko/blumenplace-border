
#![crate_name = "blumenplace-border"]
#![crate_type = "bin"]

#![desc = "blumenplace-border application"]
#![license = "GPLv3"]


#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate getopts;
extern crate url;
extern crate http;


use std::io::signal;
use std::io::println;
use std::sync::Arc;

use http::method::Get;
use http::client::RequestWriter;


mod settings;


static FRONT_SERVER_NAME: &'static str = "blumenplace-front server";


fn udp_service(shared_settings: Arc<settings::Settings>) -> () {
    info!("UDP service starting. UDP port {} will be used for incoming requests.", shared_settings.port);

    info!("UDP service finished.");
}


fn http_service(shared_settings: Arc<settings::Settings>) -> () {
    let url = shared_settings.front_url.clone();
    info!("Connecting to {} \"{}\".", FRONT_SERVER_NAME, shared_settings.front_url);

    let request: RequestWriter = match RequestWriter::new(Get, url) {
        Ok(r) => r,
        Err(error) => fail!("Failed to create request to {} \"{}\". Reason: {}", FRONT_SERVER_NAME, shared_settings.front_url, error)
    };
    info!("Request to {} \"{}\" has been successfully created.", FRONT_SERVER_NAME, shared_settings.front_url);

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err((_request, error)) => fail!("Failed to read response from {} \"{}\". Reason: {}.", FRONT_SERVER_NAME, shared_settings.front_url, error)
    };
    info!("The response from {} \"{}\" has been created.", FRONT_SERVER_NAME, shared_settings.front_url);

    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(err) => fail!("Reading response failed: {}", err),
    };
    println(std::str::from_utf8(body.as_slice()).expect("Failed to decode response. utf-8 encoding expected."));
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
        _ => (),
    };

    loop {
        match listener.rx.recv() {
            signal::Interrupt => break,
            _ => (),
        }
    }

    info!("Application finished.");
}


fn main () -> () {
    info!("Application initialized.");
    match settings::parse_args() {
        Some(settings) => run_app(settings),
        None => (),
    }
    info!("Application finished");
}

