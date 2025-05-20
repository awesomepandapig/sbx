import { ArrowDown, ArrowRight, ArrowUp } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { WS_URL } from "~/lib/config";
import { cn } from "~/lib/utils";
import React from "react";

const initialData = [
  {
    symbol: "JSP",
    bid: 7892892,
    ask: 7892890,
    direction: "down",
    icon: "https://wiki.hypixel.net/images/7/72/SkyBlock_items_jasper_crystal.png",
  },
  {
    symbol: "FRY",
    bid: 2577134,
    ask: 2577130,
    direction: "up",
    icon: "/Vanguard_Helmet.png",
  },
  {
    symbol: "DRG",
    bid: 15273182,
    ask: 15273181,
    direction: "up",
    icon: "https://wiki.hypixel.net/images/7/75/SkyBlock_pets_golden_dragon.png",
  },
  {
    symbol: "USD",
    bid: 105406,
    ask: 105405,
    direction: "neutral",
    icon: "https://wiki.hypixel.net/images/7/72/SkyBlock_items_jasper_crystal.png",
  },
  {
    symbol: "GBP",
    bid: 1.33555,
    ask: 1.3357,
    direction: "down",
    icon: "https://wiki.hypixel.net/images/7/72/SkyBlock_items_jasper_crystal.png",
  },
];

type Ticker = typeof initialData[number];

export default function TickerTable() {
  const [dataMap, setDataMap] = useState(() => {
    const map = new Map<string, Ticker>();
    for (const item of initialData) {
      map.set(item.symbol, { ...item });
    }
    return map;
  });

  useEffect(() => {
    const productNames = initialData.map((item) => item.symbol);
    const ws = new WebSocket(`${WS_URL}`);

    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          type: "subscribe",
          product_ids: productNames,
          channel: "ticker_batch",
        })
      );
    };

    ws.onmessage = (msg) => {
      const parsed = JSON.parse(msg.data);
        console.log(parsed);

      if (!parsed?.events) return;

      setDataMap((prev) => {
        const next = new Map(prev); // shallow clone
        for (const event of parsed.events) {
          if (event.type !== "update") continue;
          for (const ticker of event.tickers) {
            const existing = next.get(ticker.product_id);
            if (!existing) continue;

            next.set(ticker.product_id, {
              ...existing,
              bid: ticker.best_bid,
              ask: ticker.best_ask,
              direction:
                ticker.price_percent_chg_24_h > 0
                  ? "up"
                  : ticker.price_percent_chg_24_h < 0
                  ? "down"
                  : "neutral",
            });
          }
        }
        return next;
      });
    };

    return () => ws.close();
  }, []);

  return (
    <div className="hidden md:flex md:col-span-2 md:h-full bg-[#0e0e0e] rounded-[20px] overflow-hidden">
      <div className="flex-grow flex flex-col">
        <table className="w-full h-full text-sm text-left text-white table-fixed">
          <thead className="bg-[#1e1e1e] text-gray-400">
            <tr>
              <th className="px-6 py-4 font-light">SYMBOL</th>
              <th className="px-6 py-4 font-light">BID</th>
              <th className="px-6 py-4 font-light">ASK</th>
            </tr>
          </thead>
          <tbody>
            {[...dataMap.values()].map((item, index) => (
              <TickerRow key={item.symbol} item={item} index={index} />
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

const TickerRow = React.memo(function TickerRow({
  item,
  index,
}: {
  item: Ticker;
  index: number;
}) {
  return (
    <tr
      className={cn(
        "bg-[#0e0e0e] border-t border-[#1e1e1e] hover:bg-[#1e1e1e]",
        index === 0 && "border-t-0"
      )}
    >
      {/* symbol column */}
      <td className="px-6 py-4">
        <div className="flex items-center space-x-3">
          <div className="w-8 h-8 rounded-full flex items-center justify-center">
            <img
              src={item.icon || "/placeholder.svg"}
              alt={item.symbol}
              className="w-6 h-6 object-contain"
            />
          </div>
          <span className="text-base tracking-wide">{item.symbol}</span>
        </div>
      </td>

      {/* Bid */}
      <td className="px-6 py-4 text-base whitespace-nowrap">
        <div className="flex flex-row gap-2 items-center">
          {item.direction === "neutral" && (
            <span className="text-gray-400">
              <ArrowRight size={16} />
            </span>
          )}
          {item.direction === "up" && (
            <span className="text-green-500">
              <ArrowUp size={16} />
            </span>
          )}
          {item.direction === "down" && (
            <span className="text-red-500">
              <ArrowDown size={16} />
            </span>
          )}
          {item.bid}
        </div>
      </td>

      {/* Ask */}
      <td className="px-6 py-4 text-base whitespace-nowrap">{item.ask}</td>
    </tr>
  );
});