extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
use reqwest::Error;
use std::fs;
use std::fs::OpenOptions;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    name: String,
    class: String,
    target: String,
    desc: String,
    developer: String,
    added: String,
    changed: String,
    file: String,
}

pub fn Download(target: &str) -> Result<(), Error> {
    println!("Downloading up to date market data");
    let request_url = format!("{}",target);
    let mut response = reqwest::get(&request_url)?;
    let marketplace: Vec<App> = response.json()?;
    CacheMarket(marketplace, true);
    Ok(())
}

pub fn CacheMarket(marketData: Vec<App>, global: bool) {
    fs::create_dir_all("./market");
    let file = OpenOptions::new()
                   .read(true)
                   .write(true)
                   .truncate(true)
                   .create(true)
                   .open(getPath(global))
                   .unwrap();
    
    serde_json::to_writer(&file, &marketData);
}   

pub fn ReadMarket(global: bool) -> Vec<App> {
    let rawData = fs::read_to_string(getPath(global)).unwrap();
    let parsedData: Vec<App> = serde_json::from_str(&rawData).unwrap();
    return parsedData;
}

fn getPath(global: bool) -> String {
    if global { return "./market/full-list.json".to_string(); }
    else { return "./market/device-list.json".to_string(); }
}