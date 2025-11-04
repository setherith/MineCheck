
use std::fs;
use std::io::{copy, Read};
use std::fs::File;

use reqwest::blocking;
use sha1::{Digest, Sha1};

use crate::structs::Infomation;
use crate::structs::Manifest;

mod structs;

fn main() {
    let response = blocking::get("https://launchermeta.mojang.com/mc/game/version_manifest.json").ok();
    let manifest: Manifest = response.expect("Failed").json().unwrap();

    let mut current_version_url: String = String::new();

    for version in manifest.versions {
        if version.id == manifest.latest.release {
            current_version_url = version.url;
            break;
        }
    }

    println!("Current version url: {current_version_url}");

    let response = blocking::get(current_version_url).ok();

    let info: Infomation = response.expect("Failed").json().unwrap();

    println!("SHA1:\t{}\nSize:\t{}\nURL:\t{}", 
            info.downloads.server.sha1, 
            info.downloads.server.size, 
            info.downloads.server.url);

    // download the file
    let path = String::from("./server.jar");
    let mut file_data = blocking::get(info.downloads.server.url).expect("Failed");
    let mut file = File::create(&path).expect("Failed");

    copy(&mut file_data, &mut file).expect("Failed");

    // confirm download
    let size = fs::metadata(&path).expect("Failed").len();
    println!("File length: {:#?}", size);

    if info.downloads.server.size == size {
        println!("File size matches \u{2705}");
    } else {
        println!("File size does NOT match \u{274E}")
    }

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

    println!("SHA1: {:x}", result);


    if format!("{:x}", result) == info.downloads.server.sha1 {
        println!("SHA1 matches \u{2705}");
    } else {
        println!("SHA1 does NOT match \u{274E}");
    }

    // check the local version - either contents of the zip or record in json file
    // if local is less than latest then replace
        // schedule replacement
            // find running instances
            // exit process
            // move old version to archive folder
            // check archive for limit and delete versions outside of retention policy
            // chmod new version
            // restart service
            // record newest version in json file 

}