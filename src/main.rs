#![feature(fs_read_write)]

extern crate argparse;
extern crate sha1;

mod object;
mod init;
mod util;

use argparse::{ArgumentParser, Store, StoreTrue};


fn main() {
    let mut init = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("a minimal version control system");
        ap.refer(&mut init)
            .add_option(&["--init"], StoreTrue, "Create a new repository");
        ap.parse_args_or_exit();
    }

    // Because argparse doesn't support mutually exclusive groups
    // out of the box, options are just tried in order
    if init {
        init::init();
    }

    println!("Hello, world!");
}
