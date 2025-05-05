import { useEffect, useState } from "react";
import { WS_URL, DOMAIN } from "~/lib/config";

interface TickerCardProps {
  name: string;
  symbol: string;
  price: string;
  change: number;
  type?: string;
  color?: string;
  chartColor?: string;
  chartData?: number[];
  img: string;
}

interface Event {
  type: "snapshot" | "update";
  tickers: Ticker[];
}

interface Ticker {
  product_id: string;
  price: string;
  volume_24_h: string;
  low_24_h: string;
  high_24_h: string;
  low_52_w: string;
  high_52_w: string;
  price_percent_chg_24_h: string;
  best_bid: string;
  best_bid_quantity: string;
  best_ask: string;
  best_ask_quantity: string;
}

function formatPrice(price: number): string {
  if (price >= 1_000_000) return (price / 1_000_000).toFixed(1) + "M";
  if (price >= 1_000) return Math.round(price / 1_000) + "K";
  return price.toFixed(2);
}

export default function TickerCard({
  name,
  symbol,
  type = "Mineshaft",
  color = "blue",
  img,
}: TickerCardProps) {
  const [isHovered, setIsHovered] = useState(false);
  const [tickerData, setTickerData] = useState<Ticker | null>(null);

  useEffect(() => {
    const ws = new WebSocket(WS_URL);
    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          type: "subscribe",
          product_ids: [symbol],
          channel: "ticker_batch",
        })
      );
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data?.channel !== "ticker_batch") return;

        for (const evt of data.events as Event[]) {
          for (const ticker of evt.tickers) {
            if (ticker.product_id === symbol) {
              setTickerData(ticker);
              return;
            }
          }
        }
      } catch (err) {
        console.error("Error parsing ticker update:", err);
      }
    };

    return () => ws.close();
  }, [symbol]);

  const price = tickerData ? parseFloat(tickerData.price) : null;
  const change = tickerData ? parseFloat(tickerData.price_percent_chg_24_h) : null;
  const isPositiveChange = change !== null && change >= 0;
  const changeStyle = isPositiveChange
    ? "bg-green-500/20 text-green-400"
    : "bg-red-500/20 text-red-400";

  return (
    <a href={`${DOMAIN}/trade/${symbol}`}>
      <div
        className={`min-w-[200px] rounded-lg border border-zinc-800 bg-zinc-900/80 backdrop-blur-sm hover:border-zinc-700 transition-all duration-300`}
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
        style={{ boxShadow: isHovered ? `0 0 25px ${color}` : "none" }}
      >
        <div className="flex items-center p-3 border-b border-zinc-800">
          <div className="w-10 h-10 rounded flex items-center justify-center mr-3">
            <img
              src={img}
              alt={name}
              width={24}
              height={24}
              className="object-contain"
            />
          </div>
          <div>
            <div className="flex items-center">
              <span className="font-bold text-white">{name}</span>
              <span className="ml-2 text-xs px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-400">
                {symbol}
              </span>
            </div>
            <div className="text-xs text-zinc-500">{type}</div>
          </div>
        </div>
        <div className="p-3">
          <div className="flex items-center justify-between">
            <div className="text-lg text-white font-bold">
              {price !== null ? formatPrice(price) : "--"}
            </div>
            {change !== null && (
              <div
                className={`text-sm font-medium px-2 py-1 rounded ${changeStyle}`}
              >
                {isPositiveChange && "+"}
                {change.toFixed(1)}%
              </div>
            )}
          </div>
        </div>
      </div>
    </a>
  );
}