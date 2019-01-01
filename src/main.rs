#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    crypto_iso: String,
    fiat_iso: String,
    crypto_logo: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Crypto {
    ticker: Ticker,
    success: bool,
    error: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Ticker {
    base: String,
    target: String,
    price: String,
    volume: String,
    change: String 
}

fn main() {
    let mut file = File::open("conf.toml").expect("Can't read file");
    let mut config = String::new();
    file.read_to_string(&mut config)
        .expect("Could not read the file");
    let config: Config = toml::from_str(&config).unwrap();
    println!("File content: {:?}", config);

    let req_link = format!("https://api.cryptonator.com/api/ticker/{}-{}", config.crypto_iso, config.fiat_iso);

    // gets a JSON resp like
    // {"ticker":{"base":"BTC","target":"USD","price":"443.78078", .....},"timestamp":1399490941,"success":true,"error":""}
    // let resp: Crypto = reqwest::get("https://api.cryptonator.com/api/ticker/btc-usd")
    let resp: Crypto = reqwest::get(&req_link)
        .expect("Could not make request")
        .json().expect("Could not read json");
    
    let ticker = resp.ticker;

    // Converts string into float 
    let conv_price: f64 = ticker.price.parse().unwrap();

    let price = format!("{:.*}", 2, conv_price);
    let formatted_price = format!("{}", &price);
    let target = format!("{}", &ticker.target);
    println!("  {} {} ", formatted_price, target);
}










