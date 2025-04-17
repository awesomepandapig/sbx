# SBX - SkyBlock Exchange

<p align="center">
  <img width="1507" src="https://github.com/user-attachments/assets/2cab6199-5f6e-4a54-af00-5906d71e0af8" />
</p>
<p align="center">
  SBX is a financial exchange for Hypixel SkyBlock Structures
</p>

## Overview

SBX provides a high-performance trading platform for Hypixel SkyBlock, featuring real-time order matching, market data streaming, and a responsive user interface. The platform processes thousands of orders per second, ensuring a seamless trading experience for users while maintaining a robust and scalable architecture.

- **Website**: [https://skyblock.exchange](https://skyblock.exchange)
- **Documentation**: [https://skyblock.exchange/docs/](https://skyblock.exchange/docs/)
- **Status**: [https://status.skyblock.exchange](https://status.skyblock.exchange)

## Core Components

SBX is built from modular services communicating over Redis Streams and Pub/Sub. Each component runs independently and is designed for horizontal scaling.

| Component        | Folder                          | Language       | Description |
|------------------|----------------------------------|----------------|-------------|
| Matching Engine  | [`/me`](./me)                    | Rust           | Handles order matching, emits trade events, and persists matches to Redis Streams |
| Market Data      | [`/md`](./md)                    | Rust           | Listens to trade streams and broadcasts real-time market data (ticker, order book, trades) |
| API              | [`/api`](./api)                  | TypeScript     | REST API for order creation, cancellation, and user management |
| WebSocket Server | [`/ws`](./ws)                    | TypeScript     | Pushes real-time order updates and market changes to clients |
| Website (Client) | [`/web`](./web)                  | TypeScript     | Frontend for placing orders, monitoring markets, and viewing account activity in real time |

## Tech Stack

| Layer           | Stack                                  |
|-----------------|----------------------------------------|
| Matching Engine | Rust                                   |
| Message bus     | Redis Pub/Sub / Streams                |
| Database        | TimescaleDB (PostgreSQL) + Drizzle ORM |
| API             | Node.js, TypeScript, BetterAuth        |
| Frontend        | Remix, TailwindCSS, Vite               |

## Key Features

- **High-Performance Trading**: Process thousands of orders per second with minimal latency
- **Real-Time Market Data**: Live order book updates, trade history, and ticker information
- **WebSocket API**: Real-time data streaming for developers
- **Horizontal Scalability**: All components designed to scale independently

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE.txt) file for details.