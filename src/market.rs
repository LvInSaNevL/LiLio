extern crate requests;
use std::fs::File;

pub fn Downloader(target: &str, output: &str) {
    let mut dest = File::create(output);
    let mut source = request::get(target);
    copy(&mut source, &mut dest);
}