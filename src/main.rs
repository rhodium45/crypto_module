#[macro_use]
extern crate serde_derive;
extern crate reqwest;

extern crate serde;
extern crate serde_json;

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
    let resp: Crypto = reqwest::get("https://api.cryptonator.com/api/ticker/btc-usd")
        .expect("Could not make request")
        .json().expect("Could not read json");
    
    let ticker = resp.ticker;
    // let price = ticker.price.parse::<f32>().map(|n| n + 0.0);
    let price = format!("{}", &ticker.price);
    let target = format!("{}", &ticker.target);
    println!("{} {}", price, target);
}
