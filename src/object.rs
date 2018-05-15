use std::path;
use std::fs;

use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

use glob;
use sha1;


// Compute the digest of an object
// Optionally write to the object store as well
pub fn digest(data: &str, obj_type: &str, write: bool) -> String {
    let header = format!("{} {}", obj_type, data.len());
    let data = format!("{}{}{}", header, "\x00", data);

    let hashed_digest = sha1::Sha1::from(data.as_bytes()).hexdigest();

    if write {
        let path = format!("{}/objects/{}/{}",
                           ::BASE_DIR,
                           hashed_digest.chars().take(2).collect::<String>(),
                           hashed_digest.chars().skip(2).collect::<String>());
        let path = path::Path::new(&path);

        if !path.exists() {
            fs::create_dir(path).expect("blob directory creation failed");

            // Compress the digest using zlib
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(data.as_bytes()).expect("data compression failed");
            let compressed_bytes = e.finish().expect("data compression failed");

            fs::write(path, compressed_bytes)
                .expect("failed to write compressed data to file");
        }
    }

    hashed_digest
}

// Given a hash prefix, find the matching object
// Returns the path to the matching object in the objects directory
// Returns the first match
pub fn find(prefix: String) -> path::PathBuf {
    if prefix.len() < 2 {
        panic!("Invalid prefix (too short)");
    }

    let dir_path = format!("{}/objects/{}/{}*",
                           ::BASE_DIR,
                           prefix.chars().take(2).collect::<String>(),
                           prefix.chars().skip(2).collect::<String>());

    let first_match = glob::glob(&dir_path).unwrap().nth(0).unwrap();

    first_match.unwrap()
}
