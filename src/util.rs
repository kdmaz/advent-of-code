#![allow(dead_code)]
use std::fs;

pub fn get_file_content(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("Could not read from path, ({})", path))
}
