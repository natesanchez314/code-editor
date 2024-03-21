use std::{env, fs::{self, File}};

pub fn open_file(path: String) -> String {
    println!("{:?}", std::env::current_dir());
    println!("{}", path);
    let bytes = fs::read(path).expect("Unable to open file!");
    //let mut file = File::open(path);
    let s = String::from_utf8(bytes).unwrap();
    s
}

pub fn open_dir(path: &str) -> fs::ReadDir {
    env::set_current_dir(path);
    fs::read_dir(path).expect("Unable to open directory!")
}

pub fn save_file(path: String, data: String) {
    fs::write(path, data);
}