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
    // gets a JSON resp like
    // {"ticker":{"base":"BTC","target":"USD","price":"443.78078", .....},"timestamp":1399490941,"success":true,"error":""}
    let resp: Crypto = reqwest::get("https://api.cryptonator.com/api/ticker/btc-usd")
        .expect("Could not make request")
        .json().expect("Could not read json");
    
    let ticker = resp.ticker;

    // Converts string into float 
    let conv_price: f64 = ticker.price.parse().unwrap();

    let price = format!("{:.*}", 2, conv_price);
    let formatted_price = format!("{}", &price);
    let target = format!("{}", &ticker.target);
    println!(" {} {}", formatted_price, target);
}
