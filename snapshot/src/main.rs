mod order;
mod order_book;
mod utils;

use order_book::OrderBook;
use utils::{acknowledge, read_from_stream};

use std::env;
use std::error::Error;

use chrono::{Utc, SecondsFormat};
use redis::{Client};
use redis::aio::MultiplexedConnection;
use serde::Serialize;
use serde_json;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response, header};

use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use tower::ServiceBuilder;

// pub fn startup() {
//     // Read the entire stream until the last acknowledged message to rebuild the book
//     // for order in orders
//         if(order.type == 'new') {
//             orderbook.add(&order)
//         }
//         if(order.type == 'match') {
//             // reduce order size by the match size
//             // if the size on book is 0
//                 // remove order

//     // Use XPENDING to read any unacknowledge messages
//     let missed_orders = ;
// }

#[derive(Serialize, Debug)]
struct L2Data {
    side: String,
    event_time: String,
    price_level: i64,
    new_quantity: i64,
}

async fn snapshot(req: Request<Incoming>, book: Arc<RwLock<OrderBook>>) -> Result<Response<Full<Bytes>>, Infallible> {
    if req.method() != hyper::Method::GET {
        return Ok(Response::builder()
            .status(405)
            .body(Full::new(Bytes::from("Method Not Allowed")))
            .unwrap());
    }

    let event_time: String = Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true);
    let mut updates: Vec<L2Data> = Vec::new();
    let mut sequence_num: String;

    {
        let book_guard = book.read().await;
        for (price_level, new_quantity) in book_guard.bids.iter().rev() {
            let l2_data = L2Data {
                side: "buy".to_string(),
                event_time: event_time.clone(),
                price_level: *price_level,
                new_quantity: *new_quantity,
            };
            updates.push(l2_data);
        }

        for (price_level, new_quantity) in book_guard.asks.iter() {
            let l2_data = L2Data {
                side: "sell".to_string(),
                event_time: event_time.clone(),
                price_level: *price_level,
                new_quantity: *new_quantity,
            };
            updates.push(l2_data);
        }

        sequence_num = book_guard.sequence_num.clone();
    }

    let json = serde_json::json!({
        "sequence_num": sequence_num,
        "updates": updates
    });
    let json_str = serde_json::to_string(&json).unwrap();
    let response = Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(json_str)))
        .unwrap();

    Ok(response)
}

async fn build_book_loop(
    book: Arc<RwLock<OrderBook>>,
    product_id: String,
    mut conn: MultiplexedConnection,
) {
    loop {
        let orders = read_from_stream(&mut conn, product_id.clone()).await;

        if orders.is_empty() {
            continue;
        }

        let mut book_guard = book.write().await;
        for (message_id, order) in &orders {
            match order.action.as_str() {
                "create" if order.r#type == "limit" => {
                    book_guard.add_order(&order);
                }
                "match" | "cancel" => {
                    book_guard.remove_order(&order);
                }
                _ => {
                    eprintln!("Unknown order action: {}", order.action);
                }
            }

            let stream_name = format!("instrument:events:{}", &product_id);
            acknowledge(&mut conn, &stream_name, &message_id).await;
        }

        if let Some((message_id, _)) = orders.last() {
            book_guard.sequence_num = message_id.clone();
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    // TODO: Handle REDIS_TLS configuration
    let redis_url: String = env::var("REDIS_URL").expect("REDIS_URL environment variable not set");
    let product_id: String =
        env::var("PRODUCT_ID").expect("PRODUCT_ID environment variable not set");

    println!("Connecting to Redis at: {}", redis_url);
    let client: Client = Client::open(redis_url)?;
    let mut conn: MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    // TODO: startup (rebuild the book)

    let book = Arc::new(RwLock::new(OrderBook::new()));

    // Spawn book-building loop
    {
        let book_clone = Arc::clone(&book);
        let product_id_clone = product_id.clone();

        tokio::spawn(async move {
            build_book_loop(book_clone, product_id_clone, conn).await;
        });
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let book_clone = Arc::clone(&book);

        tokio::spawn(async move {
            let svc = hyper::service::service_fn(move |req| {
                let book = Arc::clone(&book_clone);
                snapshot(req, book)
            });
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                eprintln!("server error: {}", err);
            }
        });
    }
}