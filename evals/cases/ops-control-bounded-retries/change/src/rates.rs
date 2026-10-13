//! Daily exchange rates from the central bank's public feed.

use std::time::Duration;

use tracing::{info, warn};

const FEED_URL_VAR: &str = "RATES_FEED_URL";
const ATTEMPTS: u32 = 3;

#[derive(Debug, serde::Deserialize)]
pub struct Rates {
    pub base: String,
    pub rates: std::collections::BTreeMap<String, f64>,
}

/// Fetches today's rates: a GET, safe to retry, three attempts at most with
/// a doubling pause, each bounded by a ten-second timeout.
pub fn fetch_rates() -> Result<Rates, String> {
    let url = std::env::var(FEED_URL_VAR).map_err(|_| format!("{FEED_URL_VAR} is not set"))?;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("cannot build the HTTP client: {e}"))?;
    let mut pause = Duration::from_millis(500);
    for attempt in 1..=ATTEMPTS {
        match client.get(&url).send().and_then(|r| r.error_for_status()) {
            Ok(response) => {
                let rates: Rates = response.json().map_err(|e| format!("rates feed {url}: invalid body: {e}"))?;
                info!(attempt, currencies = rates.rates.len(), "exchange rates fetched");
                return Ok(rates);
            }
            Err(error) if attempt < ATTEMPTS => {
                warn!(attempt, %error, "exchange rates fetch failed, retrying");
                std::thread::sleep(pause);
                pause *= 2;
            }
            Err(error) => return Err(format!("rates feed {url}: {error} after {ATTEMPTS} attempts")),
        }
    }
    unreachable!("the last attempt returns")
}
