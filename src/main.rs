use serde_bencode::ser;
use serde_bencoded::from_bytes;
use std::io::Read;
use torrent_struct::Torrent;
use url::Url;
mod torrent_struct;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = if atty::is(atty::Stream::Stdin) {
        let mut file = std::fs::File::open("./sintel.torrent")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        buf
    } else {
        let mut buf = Vec::new();
        std::io::stdin().lock().read_to_end(&mut buf)?;
        buf
    };
    let torrent: Torrent = from_bytes(&bytes)?;
    println!("name {:?}", torrent.info.name);
    println!("path:\t\t{:?}", torrent.info.path);
    if let &Some(ref files) = &torrent.info.files {
        for f in files {
            println!("file path:\t{:?}", f.path);
            println!("file length:\t{}", f.length);
        }
    }
    println!("announce_url {:?}", torrent.announce);
    let info_bytes: Vec<u8> = ser::to_bytes(&torrent.info)?;
    let mut hasher = Sha1::new();
    hasher.input(&info_bytes);
    let info_hash = hasher.result_str();
    println!("infohash {:?}", &info_hash);
    let val: &str = &*torrent.announce;
    let parsed_url = Url::parse(val)?;
    println!(
        "{:?}{:?}",
        parsed_url.domain().unwrap(),
        parsed_url.port().unwrap()
    );

    let uri = parsed_url.domain().unwrap();
    println!("{}", uri);

    let body = reqwest::get(format!("http://{}", uri))
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
    Ok(())
}
