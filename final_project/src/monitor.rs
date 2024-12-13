use serde::Serialize;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

pub struct AppConfig {
    pub worker_threads: usize,
    pub timeout: Duration,
    pub retries: usize,
}

impl AppConfig {
    pub fn new(worker_threads: usize, timeout_secs: u64, retries: usize) -> Self {
        AppConfig {
            worker_threads,
            timeout: Duration::from_secs(timeout_secs),
            retries,
        }
    }
}

#[derive(Serialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: SystemTime,
}

pub fn run_monitor(
    urls: Vec<String>,
    timeout: Duration,
    worker_count: usize,
    max_retries: usize,
) -> Vec<WebsiteStatus> {
    let shared_urls = Arc::new(urls);
    let (tx, rx) = mpsc::channel();
    let mut workers = vec![];

    let http_agent = ureq::AgentBuilder::new().timeout_read(timeout).timeout_connect(timeout).build();

    for _ in 0..worker_count {
        let tx_clone = tx.clone();
        let urls_clone = Arc::clone(&shared_urls);
        let http_agent_clone = http_agent.clone();

        workers.push(thread::spawn(move || {
            for url in urls_clone.iter() {
                let mut attempts = 0;

                let result = loop {
                    let start_time = Instant::now();
                    let response = match http_agent_clone.get(url).call() {
                        Ok(resp) => Ok(resp.status()),
                        Err(err) => {
                            attempts += 1;
                            if attempts > max_retries {
                                Err(err.to_string())
                            } else {
                                continue;
                            }
                        }
                    };

                    let elapsed = start_time.elapsed();
                    tx_clone
                        .send(WebsiteStatus {
                            url: url.clone(),
                            status: response.clone(),
                            response_time: elapsed,
                            timestamp: SystemTime::now(),
                        }).unwrap();
                        
                    break response;
                };

                if result.is_err() && attempts == max_retries {
                    eprintln!("[MAX RETRIES] reached for: {}", url);
                }
            }
        }));
    }

    drop(tx); 
    let mut output = vec![];
    for site in rx.iter() {
        output.push(site);
    }
    for worker in workers {
        worker.join().unwrap();
    }
    output
}
