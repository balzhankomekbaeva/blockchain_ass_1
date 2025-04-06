use crate::error::AppError;
use crate::models::NewsArticle;
use reqwest::header;
use std::env;

pub async fn fetch_news(crypto_id: &str) -> Result<Vec<NewsArticle>, AppError> {
    let api_key = env::var("COINMARKETCAP_API_KEY")
        .expect("Missing COINMARKETCAP_API_KEY in .env");
    
    // For news, you need to use the news API endpoint
    let client = reqwest::Client::new();
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
    
    let response = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .header("X-CMC_PRO_API_KEY", api_key)
        .query(&[("symbol", crypto_id)])
        .send()
        .await
        .map_err(AppError::ReqwestError)?;
    
    // Handle rate limits
    if response.status() == 429 {
        return Err(AppError::ApiLimitReached);
    }
    
    // Check for other errors
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        println!("API Error: {}", error_text);
        return Err(AppError::ApiError(error_text));
    }
    
    let data: serde_json::Value = response
        .json()
        .await
        .map_err(AppError::ReqwestError)?;
    
    Ok(parse_news(data))
}

fn parse_news(data: serde_json::Value) -> Vec<NewsArticle> {
    // Debug output to understand response structure
    println!("API Response structure: {:?}", data);
    
    data["data"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| NewsArticle {
            title: item["title"].as_str().unwrap_or("").to_string(),
            url: item["url"].as_str().unwrap_or("").to_string(),
            source: item["source"].as_str().unwrap_or("Unknown").to_string(),
            published_at: parse_date(item["published_at"].as_str().unwrap_or("")),
            description: item["description"].as_str().map(|s| s.to_string()),
        })
        .collect()
}

fn parse_date(date_str: &str) -> chrono::NaiveDateTime {
    if date_str.is_empty() {
        return chrono::Local::now().naive_local();
    }
    
    // Try different date formats
    for format in &["%Y-%m-%dT%H:%M:%S%z", "%Y-%m-%dT%H:%M:%SZ", "%Y-%m-%d %H:%M:%S"] {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, format) {
            return dt;
        }
    }
    
    // If all parsing attempts fail, return current time
    chrono::Local::now().naive_local()
}