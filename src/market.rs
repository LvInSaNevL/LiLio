extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use reqwest::Error;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketApp {
	pub name: String,
	pub class: String,
	pub target: String,
	pub desc: String,
	pub developer: String,
	pub added: String,
	pub changed: String,
	pub file: String,
}

pub fn Download(target: &str) -> Result<(), Error> {
	println!("Downloading up to date market data");
	let request_url = format!("{}", target);
	let mut response = reqwest::get(&request_url)?;
	let marketplace: Vec<MarketApp> = response.json()?;
	CacheMarket(marketplace);
	Ok(())
}

fn CacheMarket(marketData: Vec<MarketApp>) {
	fs::create_dir_all("./market").expect("Unable to create the market directory, the location may not be correct or it may be unaccesable. Please restart your console and try again.");
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.truncate(true)
		.create(true)
		.open(getPath(true))
		.unwrap();

	serde_json::to_writer(&file, &marketData).expect("Unable to write data to the market file.");
}

pub fn ReadMarket(global: bool) -> Vec<MarketApp> {
	let rawData = fs::read_to_string(getPath(global)).unwrap();
	let parsedData: Vec<MarketApp> = serde_json::from_str(&rawData).unwrap();
	return parsedData;
}

fn getPath(_global: bool) -> String {
	if _global {
		return "./data/full-list.json".to_string();
	} else {
		return "./data/device-list.json".to_string();
	}
}
