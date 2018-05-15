use std::path::Path;
use std::fs;

use sha1;

// Compute the digest of an object
// Optionally write to the object store as well
pub fn digest(data: &str, obj_type: &str, write: bool) {
    let header = format!("{} {}", obj_type, data.len());
    let to_hash = format!("{}{}{}", header, "\x00", data);

    let hashed_digest = sha1::Sha1::from(to_hash.as_bytes()).hexdigest();

    if write {
        let path = format!("{}/objects/{}/{}",
                           ::BASE_DIR,
                           hashed_digest.chars().take(2).collect::<String>(),
                           hashed_digest.chars().skip(2).collect::<String>());
        let path = Path::new(&path);

        if !path.exists() {
            fs::create_dir(path).expect("directory creation failed;");

        }
    }
}
