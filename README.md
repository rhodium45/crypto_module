# Bitcoin Price Ticker

A polybar module which tracks and displays the current price of BTC in USD

## How to set up

 * git clone https://github.com/rhodium45/crypto_module
 * cd into crypto_module
 * build binary with: cargo build --release
 * now the binary will be located in /target/release/crypto_module

 Now add this into your modules.conf file or wherever you keep your modules

 [module/crypto_module]
 type = custom/script
 exec = ~/path/to/crypto_module/target/release/crypto_module
 interval = 600
 label-font = 3


