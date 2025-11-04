use std::fs;
use std::io::{copy, Read};
use std::fs::File;

use sha1::{Digest, Sha1};
use reqwest::blocking;

pub fn download_file(src: String, dest: &str) {
    let mut file_data = blocking::get(src).expect("Failed");
    let mut file = File::create(&dest).expect("Failed");

    copy(&mut file_data, &mut file).expect("Failed");
}

fn sha1_check(path: &str, checksum: &str) -> bool {
    let mut file = File::open(&path).expect("Failed");
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer).expect("msg");

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();

    if format!("{:x}", result) == checksum {
        println!("SHA1 matches \u{2705}");
        true
    } else {
        println!("SHA1 does NOT match \u{274E}");
        false
    }
}

fn size_check(path: &str, expected: u64) -> bool {
    let actual = fs::metadata(&path).expect("Failed").len();
    if actual == expected {
        println!("File size matches \u{2705}");
        true
    } else {
        println!("File size does NOT match \u{274E}");
        false
    }
}

pub fn check_integrity(path: &str, checksum: &str, size: u64) -> bool {
    sha1_check(path, checksum) && size_check(path, size)
}