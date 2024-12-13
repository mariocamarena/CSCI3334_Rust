mod monitor;
mod tests;

use crate::monitor::{run_monitor, AppConfig};
use std::fs::File;
use std::io::{self, BufRead};

fn load_urls(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Couldn't open url txt file");
    io::BufReader::new(file).lines().filter_map(|line| line.ok()).collect()

}

fn main() {
    let urls = load_urls("urls.txt" );

    let config = AppConfig::new(4, 5, 3); //4 threads, 5-sec, 3 max retries

    println!("\n====== Website Monitoring ======");
    println!("======== Mario Camarena ========");
    println!("starting...");
    let results = run_monitor(urls, config.timeout, config.worker_threads, config.retries);

    for site in results {
        match &site.status {
            Ok(code) => println!(
                "[passed] {}  [status: {} (Time: {:?})]", site.url, code, site.response_time
            ),
            Err(err) => println!(
                "[failed] {}  [error: {} (Time: {:?})]",site.url, err, site.response_time
            ),
        }
    }

    println!("\n====== Monitoring Completed ======\n");
}
