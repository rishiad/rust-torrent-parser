use serde_bencode::ser;
use serde_bencoded::from_bytes;
use std::io::Read;
use torrent_struct::Torrent;
mod torrent_struct;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = if atty::is(atty::Stream::Stdin) {
        let mut file = std::fs::File::open("./sample.torrent")?;
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
    let info_bytes: Vec<u8> = ser::to_bytes(&torrent.info)?;
    let mut hasher = Sha1::new();
    hasher.input(&info_bytes);
    let info_hash = hasher.result_str();
    println!("infohash {:?}", &info_hash);
    Ok(())
}
