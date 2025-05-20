use std::env;
use std::error::Error;

mod matching_engine;
mod models;

use matching_engine::MatchingEngine;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Handle REDIS_TLS configuration
    let redis_url: String =
        env::var("REDIS_URL").expect("REDIS_URL environment variable not set");
    let product_id: String = env::var("PRODUCT_ID")
        .expect("PRODUCT_ID environment variable not set");

    println!("Initializing matching engine for product: {}", product_id);
    println!("Connecting to Redis at: {}", redis_url);

    let mut engine: MatchingEngine =
        MatchingEngine::new(product_id, redis_url)?;
    engine.run();

    Ok(())
}
