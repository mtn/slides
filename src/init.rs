use std::fs;


// Initialize an empty repository
pub fn init(name: &str) {
    let create_res = fs::create_dir(name);

    let create_res = fs::create_dir(format!("{}/.git", name));
    if let Err(e) = create_res {
        println!("An error occured while creating the directory: {}",
                 e.description());

        return
    }
}

