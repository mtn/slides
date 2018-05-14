use std::error::Error;
use std::env;
use std::fs;

use util::die;


const BASE_DIR: &'static str = ".mgit";

// Initialize an empty repository
pub fn init() {
    let create_res = fs::create_dir(BASE_DIR);
    if let Err(e) = create_res {
        die(e.description());
    }

    for dir_name in ["objects", "refs", "refs/head"].iter() {
        let create_res = fs::create_dir(format!("{}/{}",
                                                BASE_DIR,
                                                dir_name));
        if let Err(e) = create_res {
            die(e.description());
        }
    }

    fs::write(format!("{}/HEAD", BASE_DIR), b"ref: refs/heads/master")
        .expect("Unable to write file");

    println!("Initialized empty Git repository in {}",
             env::current_dir().unwrap().display());
}

