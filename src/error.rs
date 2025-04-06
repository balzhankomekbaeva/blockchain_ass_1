use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ReqwestError(reqwest::Error),
    ApiLimitReached,
    ApiError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ReqwestError(e) => write!(f, "API request failed: {}", e),
            AppError::ApiLimitReached => write!(f, "CoinMarketCap API limit reached"),
            AppError::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ReqwestError(_) => HttpResponse::BadGateway().json("API service error"),
            AppError::ApiLimitReached => HttpResponse::TooManyRequests().json("API limit reached"),
            AppError::ApiError(msg) => HttpResponse::BadRequest().json(format!("API error: {}", msg)),
        }
    }
}