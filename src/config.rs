#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub crypto_iso: String,
    pub fiat_iso: String,
    pub crypto_logo: String
}

pub fn get_config() -> Config {
    let crypto_iso = String::from("btc");
    let fiat_iso = String::from("usd");
    let crypto_logo = String::from(" ");

    let conf = Config {crypto_iso: crypto_iso, fiat_iso: fiat_iso, crypto_logo: crypto_logo};
    conf
}

