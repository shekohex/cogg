#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

use crypto::digest::Digest;
use crypto::md5::Md5;
use failure::Error;
use log::{debug, info, log};
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Error>;

pub fn get_hash_from(path: impl AsRef<str>) -> Result<String> {
    let buf = read_as_bytes(path)?;
    let hash_str = get_md5_hash(&buf);
    Ok(hash_str.as_ref().to_string())
}

#[inline]
fn get_md5_hash(buf: &[u8]) -> impl AsRef<str> {
    let mut hash = Md5::new();
    hash.input(buf);
    let result: String = hash.result_str();
    debug!("Got the MD5 Hash: {}", result);
    result
}

fn read_as_bytes(path: impl AsRef<str>) -> Result<Vec<u8>> {
    info!("Reading file at {}", path.as_ref());
    let mut f = File::open(path.as_ref())?;
    let file_len = f.metadata()?.len();
    debug!("Size of that file is {}", file_len);
    let mut v: Vec<u8> = Vec::with_capacity(file_len as usize + 1);
    f.read_to_end(&mut v)?;
    info!("Got the file bytes OK!");
    Ok(v)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_generate_hash() {
        // the file only contains "1234"
        let hash = crate::get_hash_from("../tmp/f1.txt").unwrap();
        assert_eq!(hash.as_ref(), "81dc9bdb52d04dc20036dbd8313ed055");
    }
}
