use serde_json::Value;

fn main() {
    if let Err(err) = fetch_ton_price() {
        eprintln!("Error: {}", err);
    }
}

fn fetch_ton_price() -> Result<(), Box<dyn std::error::Error>> {
    // Replace 'YOUR_API_KEY' with your actual CoinAPI API key
    let api_key = "4F862EE0-2481-4EB0-9B62-23D998013223";

    // Specify the CoinAPI URL for TON price
    let url = format!(
        "https://rest.coinapi.io/v1/exchangerate/TON/USD?apikey={}",
        api_key
    );

    // Send a GET request to the CoinAPI
    let response = reqwest::blocking::get(&url)?;

    // Ensure the request was successful (HTTP status code 200)
    if response.status().is_success() {
        // Parse the JSON response body
        let json_data: Value = serde_json::from_str(&response.text()?)?;

        // Extract the TON price from the JSON data
        if let Some(price) = json_data.get("rate") {
            if let Some(price_float) = price.as_f64() {
                println!("Current TON price: ${:.2}", price_float);
                return Ok(());
            }
        }

        // If the TON price data is not found
        return Err("TON price data not found".into());
    } else {
        println!("Failed to fetch TON price: {}", response.status());
        return Err("Failed to fetch TON price".into());
    }
}
