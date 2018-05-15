use std::process::exit;
use std::path::Path;
use std::env;
use std::fs;


// Initialize an empty repository
pub fn init() {
    // If the git subdirectory already exists, abort initialization
    if Path::new(::BASE_DIR).exists() {
        println!("there is already a repository here, aborting");
        exit(0);
    }

    // Create the main vc subdirectory
    fs::create_dir(::BASE_DIR).expect("git subdirectory creation failed");

    for dir_name in ["objects", "refs", "refs/head"].iter() {
        fs::create_dir(format!("{}/{}", ::BASE_DIR, dir_name))
            .expect("directory creation failed");
    }

    fs::write(format!("{}/HEAD", ::BASE_DIR), b"ref: refs/heads/master")
        .expect("unable to write file HEAD");

    println!("initialized empty git repository in {}",
             env::current_dir().unwrap().display());
}

