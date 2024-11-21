use serde::Deserialize;
use std::{thread, time::Duration};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Bitcoin {
    price_usd: f64,
}

#[derive(Debug, Deserialize)]
struct Ethereum {
    price_usd: f64,
}

#[derive(Debug, Deserialize)]
struct SP500 {}

trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, filename: &str, price: f64);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let api_url = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
        let response = ureq::get(api_url).call().unwrap();
        let response_body = response.into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&response_body).unwrap();
        let rate = json["bpi"]["USD"]["rate_float"].as_f64().unwrap();
        Ok(rate)
    }

    fn save_to_file(&self, filename: &str, price: f64) {
        let mut file = OpenOptions::new().create(true).append(true).open(filename).unwrap();
        writeln!(file, "{:.2}", price).unwrap();
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        let api_url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(api_url).call().unwrap();
        let response_body = response.into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&response_body).unwrap();
        let price = json["ethereum"]["usd"].as_f64().unwrap();
        Ok(price)
    }

    fn save_to_file(&self, filename: &str, price: f64) {
        let mut file = OpenOptions::new().create(true).append(true).open(filename).unwrap();
        writeln!(file, "{:.2}", price).unwrap();
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        let api_url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";
        let response = ureq::get(api_url).call().unwrap();
        let response_body = response.into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&response_body).unwrap();
        let price = json["chart"]["result"][0]["indicators"]["quote"][0]["close"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .as_f64()
            .unwrap();
        Ok(price)
    }

    fn save_to_file(&self, filename: &str, price: f64) {
        let mut file = OpenOptions::new().create(true).append(true).open(filename).unwrap();
        writeln!(file, "{:.2}", price).unwrap();
    }
}

fn main() {
    let bitcoin_instance = Bitcoin { price_usd: 0.0 };
    let ethereum_instance = Ethereum { price_usd: 0.0 };
    let sp500_instance = SP500 {};

    let asset_data_sources: Vec<(&str, &dyn Pricing)> = vec![
        ("bitcoin_prices.txt", &bitcoin_instance),
        ("ethereum_prices.txt", &ethereum_instance),
        ("sp500_prices.txt", &sp500_instance),
    ];

    loop {
        for (filename, asset_instance) in &asset_data_sources {
            match asset_instance.fetch_price() {
                Ok(current_price) => {
                    asset_instance.save_to_file(filename, current_price);
                    println!("Saved price for {}: {:.2}", filename, current_price);
                }
                Err(_) => {
                    println!("Couldn't fetch data for {}", filename);
                }
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
