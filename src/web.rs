use reqwest::blocking;

use crate::structs::{Infomation, Manifest, FileMeta};

pub fn get_manifest_url() -> String {
    let response = blocking::get("https://launchermeta.mojang.com/mc/game/version_manifest.json").ok();
    let manifest: Manifest = response.expect("Failed").json().unwrap();

    let mut current_version_url: String = String::new();

    for version in manifest.versions {
        if version.id == manifest.latest.release {
            current_version_url = version.url;
            break;
        }
    }

    current_version_url
}

pub fn get_download_details(version_url: String) -> FileMeta {
    let response = blocking::get(version_url).ok();
    let info: Infomation = response.expect("Failed").json().unwrap();

    info.downloads.meta
}