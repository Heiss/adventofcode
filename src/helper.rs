use std::{fs::File, io::Read};

pub fn read_in(filepath: &str) -> String {
    let mut file = File::open(filepath).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("file cannot be read");
    contents
}
