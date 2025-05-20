export const PROD = process.env.NODE_ENV === "production";

const BASE_DOMAIN = PROD ? "skyblock.exchange" : "localhost";

export const DOMAIN = PROD
  ? `https://${BASE_DOMAIN}`
  : `http://${BASE_DOMAIN}:5173`;

export const API_URL = PROD
  ? `https://api.${BASE_DOMAIN}/api`
  : `http://${BASE_DOMAIN}:8000/api`;

export const WS_URL = PROD
  ? `wss://advanced-trade-ws.${BASE_DOMAIN}`
  : `ws://${BASE_DOMAIN}:8080`;

export const DOCS_URL = PROD
  ? `https://docs.${BASE_DOMAIN}`
  : `http://${BASE_DOMAIN}:5174`;
