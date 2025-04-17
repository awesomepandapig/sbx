import { useState } from "react";
import { useParams } from "@remix-run/react";
import TradingInterface from "~/components/trading/TradingInterface";
import Header from "~/components/trading/Header";
import OrderBook from "~/components/trading/OrderBook";
import Chart from "~/components/trading/Chart";
import { useEffect } from "react";
import { useNavigate } from "@remix-run/react";
import { useLoaderData } from "@remix-run/react";
import OrderDrawer from "~/components/trading/OrderDrawer";
import { getUserSession } from "~/lib/auth";
import { WS_URL } from "~/lib/config";

interface Ticker {
  best_ask: number;
  best_ask_quantity: number;
  best_bid: number;
  best_bid_quantity: number;
  high_24h: number;
  low_24h: number;
  price: number;
  price_percent_chg_24h: number;
  product_id: string;
  type: "ticker";
  volume_24h: number;
}

export const loader = getUserSession;

export default function Trade() {
  const [tickerData, setTickerData] = useState<Ticker>();

  const user = useLoaderData<typeof loader>();

  const navigate = useNavigate();
  const { symbol } = useParams();

  // TODO: Validate symbol
  if (!symbol) {
    throw new Response(null, {
      status: 404,
      statusText: "Not Found",
    });
  }

  // If user is logged in but not verified redirect to verification page
  useEffect(() => {
    if (user && !user.minecraftId) {
      navigate("/verify-ign");
    }
  }, [user, navigate]);

  // Get ticker data
  useEffect(() => {
    const ws = new WebSocket(WS_URL);
    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          type: "subscribe",
          product_ids: [symbol],
          channel: "ticker",
        }),
      );
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data?.channel !== "ticker") return;

        const [tickerUpdate] = data.events;
        setTickerData(tickerUpdate);
      } catch (err) {
        console.error("Error parsing ticker update:", err);
      }
    };
  }, [symbol]);

  return (
    <main className="flex flex-col h-screen bg-black">
      <Header
        symbol={symbol}
        userImg={user?.image ?? undefined}
        tickerData={tickerData}
      />

      <div className="flex flex-1 overflow-hidden">
        <div className="flex flex-1 overflow-hidden">
          {/* Left section - Chart */}
          <div className="flex-1 min-w-0 z-0">
            <Chart symbol={symbol} tickerData={tickerData} />
          </div>

          {/* Middle section - Order Book */}
          <div className="w-64">
            <OrderBook symbol={symbol} />
          </div>

          {/* Right section - Trading Interface */}
          <div className="w-72">
            <TradingInterface symbol={symbol} authenticated={user != null} />
          </div>
        </div>
      </div>

      <OrderDrawer authenticated={user != null} />
    </main>
  );
}
