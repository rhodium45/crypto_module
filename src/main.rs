#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;

mod config;

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
    let config = config::get_config();

    let resp: Crypto = make_request(create_request_url(config.crypto_iso, config.fiat_iso));
    print_crypto(config.crypto_logo, format_price(resp.ticker.price), format_target_currency(resp.ticker.target));
}

fn create_request_url(crypto_iso: String, fiat_iso: String) -> String {
    return format!("https://api.cryptonator.com/api/ticker/{}-{}", &crypto_iso, &fiat_iso);
}

fn make_request(req_url: String) -> Crypto {
    return reqwest::get(&req_url)
        .expect("Could not make request")
        .json().expect("Could not read json");
}

fn convert_price(crypto_price: String) -> f64 {
    return crypto_price.parse().unwrap();
}

fn format_price(price: String) -> String {
    return format!("{:.*}", 2, convert_price(price));
}

fn format_target_currency(target: String) -> String {
    return format!("{}", target);
}

fn print_crypto(logo: String, price: String, target: String) {
    println!("{}{} {}", logo, price, target);
}
