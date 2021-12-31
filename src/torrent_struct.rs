use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Deserialize, Serialize)]
pub struct Node(String, i64);

#[derive(Debug, Deserialize, Serialize)]
pub struct TFile {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: u64,
    #[serde(default)]
    pub files: Option<Vec<TFile>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub info: Info,
    #[serde(default)]
    pub announce: String,
    #[serde(default)]
    pub nodes: Option<Vec<Node>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    //#[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}

pub(crate) struct Announce {
    pub info_hash: String,
    pub peer_id: String,
    /// The port on which we are listening.
    pub port: u16,
    /// True IP address
    pub ip: Option<IpAddr>,
    /// Number up bytes downloaded so far.
    pub downloaded: u64,
    /// Number up bytes uploaded so far.
    pub uploaded: u64,
    /// Number up bytes left to download.
    pub left: u64,
    pub peer_count: Option<usize>,
    #[allow(dead_code)]
    pub tracker_id: Option<String>,
}

//https://github.com/mandreyel/cratetorrent/blob/master/cratetorrent/src/tracker.rs
