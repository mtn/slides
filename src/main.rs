#![feature(fs_read_write)]

extern crate argparse;
extern crate flate2;
extern crate glob;
extern crate sha1;

mod object;
mod init;


use argparse::{ArgumentParser, Store, StoreTrue};

const BASE_DIR: &'static str = ".mgit";


fn main() {
    let mut init = false;
    let mut commit_message = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("a minimal version control system");
        ap.refer(&mut init)
            .add_option(&["--init"], StoreTrue, "create a new repository");
        ap.refer(&mut commit_message)
            .add_option(&["--commit"], Store, "create a new commit");
        ap.parse_args_or_exit();
    }

    // Because argparse doesn't support mutually exclusive groups
    // out of the box, options are just tried in order
    if init {
        init::init();
    } else if !commit_message.is_empty() {
        unimplemented!();
    }

    println!("Hello, world!");
}
