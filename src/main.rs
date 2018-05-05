#![feature(fs_read_write)]

extern crate argparse;

mod init;
mod util;

use argparse::{ArgumentParser, StoreTrue, Store};


fn main() {
    let mut init_name = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("a minimal version control system");
        ap.refer(&mut init_name)
            .add_option(&["--init"], Store, "Project name");
        ap.parse_args_or_exit();
    }

    // Because argparse doesn't support mutually exclusive groups
    // out of the box, options are just tried in order
    if !init_name.is_empty() {
        init::init(&init_name);
    }

    println!("Hello, world!");
}
