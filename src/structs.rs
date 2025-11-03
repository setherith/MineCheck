use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "versions")]
pub struct Version {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Infomation {
    pub downloads: Downloads,
}