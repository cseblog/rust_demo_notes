use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{Duration, SystemTime};

fn main() {
    let start = SystemTime::now();

    // Create a path to the desired file
    let path = Path::new("./BTCUSDT-1m.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why),
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("{} contains:\n{}", display, s),
        Err(why) => panic!("couldn't read {}: {}", display, why),
    }

    let end = SystemTime::now();
    let diff = end.duration_since(start).expect("Testing");
    println!("Elapsed time: {} milliseconds", diff.as_millis());
}
