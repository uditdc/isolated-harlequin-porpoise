mod blockless;
use blockless::{BlocklessHttp, FetchOptions};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct CoinPriceData {
    id: String,
    price: u64,
    currency: String,
}

#[derive(Debug, Deserialize)]
struct SuccessResponse {
    ethereum: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    status: ErrorStatus,
}

#[derive(Debug, Deserialize)]
struct ErrorStatus {
    error_code: u32,
    error_message: String,
}

fn main() {
    let coin_id = "ethereum";
    let fetch_opts = FetchOptions::new("GET");

    let http = BlocklessHttp::open(
        &format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            coin_id
        ),
        &fetch_opts,
    )
    .unwrap();

    let body = String::from_utf8(http.get_all_body().unwrap()).unwrap();
    http.close();

    // Try to parse as success response first
    match serde_json::from_str::<SuccessResponse>(&body) {
        Ok(success_response) => {
            if let Some(usd_price) = success_response.ethereum.get("usd") {
                let coin_price = CoinPriceData {
                    id: coin_id.to_string(),
                    price: (*usd_price * 1_000_000.0) as u64, // price 6dp
                    currency: "usd".to_string(),
                };
                println!("ethereum price: ${:.2}", usd_price);
                println!("Full data: {}", json!(coin_price));
            } else {
                println!("USD price not found in the response");
            }
        }
        Err(_) => {
            // If it's not a success response, try to parse as error response
            match serde_json::from_str::<ErrorResponse>(&body) {
                Ok(error_response) => {
                    println!(
                        "Error: {} (Code: {})",
                        error_response.status.error_message, error_response.status.error_code
                    );
                }
                Err(_) => {
                    println!("Failed to parse the response: {}", body);
                }
            }
        }
    }
}
