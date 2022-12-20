use std::{fs::File, path::Path, io::Read};

pub fn read_file_as_string(file_path: String) -> Result<String, String> {
    //Read file
    // Create a path to the desired file
    let path = Path::new(file_path.as_str());
    let display = path.display();
    let mut file = File::open(&path).unwrap();

    let mut s = String::new();
    let result = file.read_to_string(&mut s);
    match result {
        Ok(_) => return Ok(s),
        Err(why) => return Err(format!("couldn't read {}: {}", display, why)),
    }
}
