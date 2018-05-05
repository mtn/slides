use std::process::exit;


// Print out a reason and exit
pub fn die (e_desc: &str) {
    println!("{}", e_desc);
    exit(1);
}

