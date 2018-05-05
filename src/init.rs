use std::error::Error;
use std::fs;

use util::die;


// Initialize an empty repository
pub fn init(name: &str) {
    let create_res = fs::create_dir(name);
    if let Err(e) = create_res {
        die(e.description());
    }

    let create_res = fs::create_dir(format!("{}/.git", name));
    if let Err(e) = create_res {
        die(e.description());
    }

    for dir_name in ["objects", "refs", "refs/head"].iter() {
        let create_res = fs::create_dir(format!("{}/.git/{}",
                                                name, dir_name));
        if let Err(e) = create_res {
            die(e.description());
        }
    }

    fs::write(format!("{}/.git/HEAD", name), b"ref: refs/heads/master")
        .expect("Unable to write file");

    println!("New repository {} initialized", name);
}

