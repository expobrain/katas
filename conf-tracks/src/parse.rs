use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use talks::{Talk};


pub fn from_file(filename: &str) -> Vec<Talk> {
    // Read file
    let path = Path::new(filename);
    let mut buffer = String::new();

    let _ = match File::open(&path) {
        Ok(mut f) => f.read_to_string(&mut buffer),
        Err(e) => panic!("{}", e)
    };

    // Parse talks
    let talks = buffer.lines().map(|line| line.parse().unwrap()).collect();

    talks
}
