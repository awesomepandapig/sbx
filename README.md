# SBX - SkyBlock Exchange

<p align="center">
  <img width="1507" src="https://github.com/user-attachments/assets/2cab6199-5f6e-4a54-af00-5906d71e0af8" />
</p>
<p align="center">
  A real-time, low-latency exchange built for Hypixel SkyBlock
</p>

## Overview

SBX is a high-performance trading platform designed specifically for Hypixel SkyBlock. The system architecture follows modern exchange design principles with horizontally scalable microservices communicating through Redis Streams (for event sourcing) and Pub/Sub (for low-latency broadcasting).

| Component        | Folder          | Language   | Description                                                                                                              |
| ---------------- | --------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------ |
| Matching Engine  | [`/me`](./me)   | Rust   | High-performance in-memory matching engine; processes thousands of orders per second with sub-millisecond latency. Emits trade events and writes to Redis Streams for durability          |
| Market Data Feed | [`/md`](./md)   | Rust   | Consumes trade events and maintains an up-to-date market state (tickers, order book snapshots) for broadcasting to clients |
| REST API         | [`/api`](./api) | TypeScript | Stateless HTTP layer for order lifecycle management and account operations                                                         |
| WebSocket Server | [`/ws`](./ws)   | TypeScript | Pushes real-time updates (orders, trades, tickers) to clients using efficient subscription-based routing                       |
| Client Interface | [`/web`](./web) | TypeScript | React + Remix frontend featuring intuitive trading interfaces, market visualization, and account management                                           |

## Technologies & Design Principles

| Layer               | Tech Stack                 | Notes                                                               |
| ------------------- | -------------------------- | ------------------------------------------------------------------- |
| **Matching Engine** | Rust                       | Zero-GC, predictable performance with minimal latency jitter |
| **Market Data**     | Rust                       | Push-based event handling, consistent state derivation from streams |
| **Data Transport**  | Redis Streams + Pub/Sub    | Durable event bus for service decoupling, reliable delivery, and fan-out operations        |
| **Persistence**     | TimescaleDB + Drizzle ORM  | Time-series storage for candles; relational storage for account and order data   |
| **API Layer**       | Node.js + TypeScript + Zod | Type-safe API design with comprehensive request validation |
| **Frontend**        | Remix + TailwindCSS + Vite | Server-side rendering for optimal performance |


## Engineering Highlights

* **In-Memory Order Matching**
  
  Built in Rust for speed and predictability, the matching engine processes and matches thousands of orders/sec with sub-millisecond latency, implementing a price-time priority model for fair execution.

* **Event-Sourced Architecture**
  
  Trade and order lifecycle events are stored in Redis Streams, enabling complete auditability, system recovery, and decoupled state reconstruction across services.

* **Scalable Market Data Distribution**
  
  Market state is derived from the event stream and efficiently broadcast via WebSockets with channel-based subscription management, ensuring clients receive only relevant updates.

* **Fault Isolation**
  
  Services are independently deployable and restartable, allowing horizontal scale-out and minimal blast radius in case of failure. The system maintains partial functionality even when individual components are down.

## License

This project is dual-licensed under either:

* [MIT License](./LICENSE-MIT)
* [Apache License 2.0](./LICENSE-APACHE)

You may choose either license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in SBX by you shall be dual-licensed as above, without any additional
terms or conditions.