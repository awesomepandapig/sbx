export const PROD = process.env.NODE_ENV == "production";
export const API_URL = PROD
  ? "https://api.skyblock.exchange/api"
  : "http://localhost:8000/api";
export const WS_URL = PROD
  ? "wss://advanced-trade-ws.skyblock.exchange"
  : "ws://localhost:8080";
export const DOMAIN = PROD
  ? "https://skyblock.exchange"
  : "http://localhost:5173";
