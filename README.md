# Bitcoin Price Ticker

A polybar module which tracks and displays the current price of BTC in USD

## How to set up

 * git clone https://github.com/rhodium45/crypto_module
 * cd into crypto_module
 * build binary with: cargo build --release
 * now the binary will be located in /target/release/crypto_module

 Now add this into your modules.conf file or wherever you keep your modules

```
[module/crypto_module]
type = custom/script
exec = ~/path/to/crypto_module/target/release/crypto_module
interval = 600
label-font = 3
```

## How to configure
 
 * Go to src/config.rs
 * and exchange btc for ltc, usd for eur and  for LTC example

```
let crypto_iso = String::from("btc");
let fiat_iso = String::from("usd");
let crypto_logo = String::from(" ");
``` 

```
let crypto_iso = String::from("ltc");
let fiat_iso = String::from("eur");
let crypto_logo = String::from("LTC");
```
