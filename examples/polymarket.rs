//! Complete Polymarket API rate limit configuration.
//!
//! This example shows how to configure rate limits for the full Polymarket API
//! including CLOB, Data API, GAMMA, and other endpoints.
//!
//! Run with: cargo run --example polymarket

use http::Method;
use reqwest_middleware::ClientBuilder;
use route_ratelimit::{RateLimitMiddleware, ThrottleBehavior};
use std::time::Duration;

/// Helper to create Duration from seconds
const fn secs(s: u64) -> Duration {
    Duration::from_secs(s)
}

/// Helper to create Duration from minutes
const fn mins(m: u64) -> Duration {
    Duration::from_secs(m * 60)
}

/// Build the complete Polymarket rate limiting middleware.
pub fn polymarket_middleware() -> RateLimitMiddleware {
    RateLimitMiddleware::builder()
        // =========================================================================
        // CLOB API (clob.polymarket.com)
        // =========================================================================
        .host("clob.polymarket.com", |host| {
            host
                // General CLOB limit
                .route(|r| r.limit(9000, secs(10)))
                // -----------------------------------------------------------------
                // Trading Endpoints (with burst + sustained limits)
                // -----------------------------------------------------------------
                .route(|r| {
                    r.method(Method::POST)
                        .path("/order")
                        .limit(3500, secs(10)) // Burst: 3500/10s
                        .limit(36000, mins(10)) // Sustained: 36000/10min
                })
                .route(|r| {
                    r.method(Method::DELETE)
                        .path("/order")
                        .limit(3000, secs(10)) // Burst: 3000/10s
                        .limit(30000, mins(10)) // Sustained: 30000/10min
                })
                .route(|r| {
                    r.method(Method::POST)
                        .path("/orders")
                        .limit(1000, secs(10)) // Burst: 1000/10s
                        .limit(15000, mins(10)) // Sustained: 15000/10min
                })
                .route(|r| {
                    r.method(Method::DELETE)
                        .path("/orders")
                        .limit(1000, secs(10)) // Burst: 1000/10s
                        .limit(15000, mins(10)) // Sustained: 15000/10min
                })
                .route(|r| {
                    r.method(Method::DELETE)
                        .path("/cancel-all")
                        .limit(250, secs(10)) // Burst: 250/10s
                        .limit(6000, mins(10)) // Sustained: 6000/10min
                })
                .route(|r| {
                    r.method(Method::DELETE)
                        .path("/cancel-market-orders")
                        .limit(1000, secs(10)) // Burst: 1000/10s
                        .limit(1500, mins(10)) // Sustained: 1500/10min
                })
                // -----------------------------------------------------------------
                // Market Data Endpoints
                // -----------------------------------------------------------------
                .route(|r| r.path("/book").limit(1500, secs(10)))
                .route(|r| r.path("/books").limit(500, secs(10)))
                .route(|r| r.path("/price").limit(1500, secs(10)))
                .route(|r| r.path("/prices").limit(500, secs(10)))
                .route(|r| r.path("/midpoint").limit(1500, secs(10)))
                .route(|r| r.path("/midpoints").limit(500, secs(10)))
                // -----------------------------------------------------------------
                // Ledger Endpoints
                // -----------------------------------------------------------------
                .route(|r| r.path("/trades").limit(900, secs(10)))
                .route(|r| r.path("/orders").limit(900, secs(10)))
                .route(|r| r.path("/notifications").limit(125, secs(10)))
                .route(|r| r.path("/data/orders").limit(500, secs(10)))
                .route(|r| r.path("/data/trades").limit(500, secs(10)))
                // -----------------------------------------------------------------
                // Other CLOB Endpoints
                // -----------------------------------------------------------------
                .route(|r| r.path("/tick-size").limit(200, secs(10)))
                .route(|r| r.path("/price-history").limit(1000, secs(10)))
                .route(|r| r.path("/api-keys").limit(100, secs(10)))
                .route(|r| {
                    r.method(Method::GET)
                        .path("/balance-allowance")
                        .limit(200, secs(10))
                })
                .route(|r| {
                    r.method(Method::POST)
                        .path("/balance-allowance")
                        .limit(50, secs(10))
                })
        })
        // =========================================================================
        // Data API (data-api.polymarket.com)
        // =========================================================================
        .host("data-api.polymarket.com", |host| {
            host
                // General Data API limit
                .route(|r| r.limit(1000, secs(10)))
                // Specific endpoints
                .route(|r| r.path("/trades").limit(200, secs(10)))
                .route(|r| r.path("/positions").limit(150, secs(10)))
                .route(|r| r.path("/closed-positions").limit(150, secs(10)))
        })
        // =========================================================================
        // GAMMA API (gamma-api.polymarket.com)
        // =========================================================================
        .host("gamma-api.polymarket.com", |host| {
            host
                // General GAMMA limit
                .route(|r| r.limit(4000, secs(10)))
                // Specific endpoints
                .route(|r| r.path("/events").limit(300, secs(10)))
                .route(|r| r.path("/markets").limit(300, secs(10)))
                .route(|r| r.path("/comments").limit(200, secs(10)))
                .route(|r| r.path("/tags").limit(200, secs(10)))
                .route(|r| r.path("/search").limit(300, secs(10)))
        })
        // =========================================================================
        // Relayer API
        // =========================================================================
        .host("relayer.polymarket.com", |host| {
            host.route(|r| r.path("/submit").limit(25, mins(1)))
        })
        .build()
}

#[tokio::main]
async fn main() {
    println!("Polymarket Rate Limiting Example\n");

    // Create the middleware
    let middleware = polymarket_middleware();

    // Create a rate-limited client
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(middleware)
        .build();

    // Example: Fetch order book (limited to 1500 requests / 10s)
    println!("Fetching CLOB order book...");
    match client
        .get("https://clob.polymarket.com/book")
        .query(&[("token_id", "some-token-id")])
        .send()
        .await
    {
        Ok(resp) => println!("Response status: {}", resp.status()),
        Err(e) => println!("Request failed: {e}"),
    }

    // Example: Fetch positions from Data API (limited to 150 requests / 10s)
    println!("\nFetching positions from Data API...");
    match client
        .get("https://data-api.polymarket.com/positions")
        .send()
        .await
    {
        Ok(resp) => println!("Response status: {}", resp.status()),
        Err(e) => println!("Request failed: {e}"),
    }

    println!("\nRate limiting is active for all Polymarket endpoints!");
    println!("The middleware will automatically delay requests that exceed limits.");

    // Example: Using ThrottleBehavior::Error instead of the default Delay
    println!("\n--- Error Behavior Example ---");
    demonstrate_error_behavior().await;
}

/// Demonstrates using ThrottleBehavior::Error to fail fast instead of delaying.
async fn demonstrate_error_behavior() {
    // Create middleware that returns errors instead of delaying
    let middleware = RateLimitMiddleware::builder()
        .route(|r| {
            r.limit(2, secs(10)).on_limit(ThrottleBehavior::Error) // Fail fast instead of waiting
        })
        .build();

    let client = ClientBuilder::new(reqwest::Client::new())
        .with(middleware)
        .build();

    // First two requests succeed
    for i in 1..=3 {
        print!("Request {i}: ");
        match client.get("https://httpbin.org/get").send().await {
            Ok(resp) => println!("Success ({})", resp.status()),
            Err(e) => {
                // The error message includes retry timing
                println!("Rate limited - {e}");
            }
        }
    }

    println!("\nWith ThrottleBehavior::Error, requests fail immediately when rate limited.");
    println!("Use this when you want to handle rate limits in your application logic.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middleware_builds_successfully() {
        // Just verify it builds without panicking
        let _middleware = polymarket_middleware();
    }
}
