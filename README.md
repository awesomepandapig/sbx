# SBX - SkyBlock Exchange

SBX is a high-performance trading platform engineered specifically for Hypixel SkyBlock. It is built on modern exchange design principles, featuring horizontally scalable microservices that communicate over [Aeron](https://github.com/aeron-io/aeron), a high-throughput, low-latency messaging system.

<div align="center">
    
<img width="1507" src="https://github.com/user-attachments/assets/2cab6199-5f6e-4a54-af00-5906d71e0af8" />
</p>

</div>
 
Check out our [Github Project](https://github.com/users/awesomepandapig/projects/6) to see current progress.

## Architecture

| Component           | Tech Stack                 | Description                                                               |
| ------------------- | -------------------------- | ------------------------------------------------------------------- |
| **Matching Engine** | Rust                       | Zero-GC, in-memory matching engine designed for predictable performance with minimal latency jitter, capable of processing millions of orders per second. |
| **Data Transport**  | Aeron                      | High-performance, low-latency messaging for service decoupling and reliable event streaming.        |
| **Persistence**     | QuestDB + Postgres         | Time-series storage for candles & tickers; relational storage for account and order data.   |
| **REST Gateway**    | Rust + Axum                | Type-safe API design with comprehensive request validation. Enables order lifecycle management and account operations. |
| **WebSocket**       | Rust + Tungstenite         | Market data feed pushes real-time updates (orders, trades, tickers) to clients via selective, channel-based routing. |
| **Frontend**        | TypeScript + Remix + TailwindCSS + Vite | Intuitive trading interfaces, market visualization, and account management features |


## Engineering Highlights

* **In-Memory Order Matching**
  
  Built in Rust for maximum safety, the matching engine is capable of processing `4.15 million` messages per second (benchmarked on a 2024 M4 Mac Mini). It implements a price-time priority model to ensure fair and efficient order execution.

* **Event-Sourced Architecture**
  
  Trade and order lifecycle events are streamed via Aeron, creating a durable and auditable log. This enables complete system recovery and allows for decoupled state reconstruction across all microservices.

* **Scalable Market Data Distribution**
  
  Market state is derived from the multicast Aeron stream and efficiently broadcast via WebSockets. Channel-based subscription management ensures that clients receive only the updates that are relevant to them, minimizing network overhead.

* **Fault Isolation**

  Each service is independently deployable and restartable, allowing for horizontal scaling and minimizing the impact of any single component failure. The system is designed to maintain partial functionality even if individual services go down.

## License

This project is dual-licensed under either:

* [MIT License](./LICENSE-MIT)
* [Apache License 2.0](./LICENSE-APACHE)

You may choose either license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in SBX by you shall be dual-licensed as above, without any additional
terms or conditions.
