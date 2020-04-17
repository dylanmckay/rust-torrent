extern crate bencode;

mod hash;
mod decoder;
mod parser;
mod tracker;
mod peer_connection;
mod message;
mod download;
mod split_stream;

use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const TORRENT_PATH: &str = "src/archlinux-2020.02.01-x86_64.iso.torrent";

    // TEST TORRENTS
    // const TORRENT_PATH: &str = "test.torrent";
    // const TORRENT_PATH: &str = "ubuntu-18.04.4-desktop-amd64.iso.torrent";

    let bencoded_metadata: Vec<u8> = fs::read(TORRENT_PATH).unwrap();
    let metadata = parser::parse_bencoded_torrent(bencoded_metadata).unwrap();

    // pass in output path at a later stage (or hardcode)
    download::download_to_file(&metadata).await;

    Ok(())
}
