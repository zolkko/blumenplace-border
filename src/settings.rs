extern crate getopts;


use getopts::{optopt, optflag, getopts, OptGroup};
use std::uint;
use std::os;


static DEF_PORT:uint = 1234u;

static DEF_FRONT_URL:&'static str = "http://front.blumenplace.io";

static DEF_EXE_PATH:&'static str = "blumenplace-border";

static BREIF_USAGE:&'static str = "[Options]";


pub struct Settings {
    pub front_url: String,
    pub port: uint
}


fn print_help(opts: &[OptGroup]) -> () {
    match os::self_exe_name() {
        Some(path) => {
            println!("{} {}", path.display(), getopts::usage(BREIF_USAGE, opts));
        }
        None => {
            println!("{} {}", DEF_EXE_PATH, getopts::usage(BREIF_USAGE, opts));
        }
    };
}


pub fn parse_args() -> Option<Settings> {
    let args: Vec<String> = os::args();

    let opts = [
        optopt("f", "front", "front server url", "FRONT"),
        optopt("p", "port", "port to listen to", "PORT"),
        optflag("h", "help", "print this help menu")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(_) => {
            print_help(opts);
            return None
        }
    };

    if matches.opt_present("h") {
        print_help(opts);
        return None
    }

    let front_url: String = match matches.opt_str("f") {
        Some(front_url_value) => { front_url_value }
        None => { String::from_str(DEF_FRONT_URL) }
    };

    let port = match matches.opt_str("p") {
        Some(port_value) => {
            match uint::parse_bytes(port_value.as_bytes(), 10) {
                Some(port_uint_value) => { port_uint_value }
                None => { DEF_PORT }
            }
        }
        None => { DEF_PORT }
    };

    return Some(Settings {
        front_url: front_url,
        port: port
    });
}

