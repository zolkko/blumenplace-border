extern crate getopts;


use getopts::{optopt, optflag, getopts, OptGroup};
use std::uint;
use std::os;
use url::Url;


static DEF_PORT:uint = 1234u;

static DEF_FRONT_URL:&'static str = "http://api.blumenplace.io";

static DEF_EXE_PATH:&'static str = "blumenplace-border";

static BREIF_USAGE:&'static str = "[Options]";


pub struct Settings<'a> {
    pub front_url: Url,
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


pub fn parse_args<'a>() -> Option<Settings<'a>> {
    let args: Vec<String> = os::args();

    let opts = [
        optopt("f", "front", "a URL to blumenplace`s front server", "[URL]"),
        optopt("p", "port", "a port number", "[PORT]"),
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

    let front_url = match matches.opt_str("f") {
        Some(value) => {
            match Url::parse(value.as_slice()) {
                Ok(url_value) => url_value,
                Err(_) => Url::parse(DEF_FRONT_URL).unwrap()
            }
        },
        None => Url::parse(DEF_FRONT_URL).unwrap()
    };

    let port = match matches.opt_str("p") {
        Some(port_value) => {
            match uint::parse_bytes(port_value.as_bytes(), 10) {
                Some(val) => val,
                None => DEF_PORT
            }
        }
        None => { DEF_PORT }
    };

    Some(Settings {
        front_url: front_url,
        port: port
    })
}

