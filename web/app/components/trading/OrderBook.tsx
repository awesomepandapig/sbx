"use client";

import { useEffect, useRef, useState } from "react";
import { WS_URL } from "~/lib/config";

interface OrderBookProps {
  symbol: string;
}

interface Level2Update {
  side: "ask" | "bid";
  price_level: string | number;
  new_quantity: string | number;
}

interface OrderLevel {
  price: number;
  quantity: number;
}

function AskRow({ price, quantity }: { price: number; quantity: number }) {
  return (
    <div className="flex justify-between px-4 text-xs py-1 border-l-2 border-[#ff4d4d] bg-[#121212]">
      <div className="w-1/2 text-left">{quantity.toFixed(0)}</div>
      <div className="w-1/2 text-right text-[#ff4d4d]">{price.toFixed(0)}</div>
    </div>
  );
}

function BidRow({ price, quantity }: { price: number; quantity: number }) {
  return (
    <div className="flex justify-between px-4 py-1 text-xs border-l-2 border-[#4caf50] bg-[#121212]">
      <div className="w-1/2 text-left">{quantity.toFixed(0)}</div>
      <div className="w-1/2 text-right text-[#4caf50]">{price.toFixed(0)}</div>
    </div>
  );
}

function EmptyRow({ side }: { side: "bid" | "ask" }) {
  return (
    <div
      className={`flex justify-between px-4 py-1 text-xs border-l-2 ${
        side === "ask" ? "border-[#ff4d4d]" : "border-[#4caf50]"
      } bg-[#121212] opacity-30`}
    >
      <div className="w-1/2 text-left">-</div>
      <div className="w-1/2 text-right">-</div>
    </div>
  );
}

export default function OrderBook({ symbol }: OrderBookProps) {
  const depth = 20;
  // Maps to maintain the client-side orderbook state
  const asksMapRef = useRef(new Map<number, number>()); // price -> quantity
  const bidsMapRef = useRef(new Map<number, number>()); // price -> quantity

  // Visible price levels for rendering
  const [visibleAsks, setVisibleAsks] = useState<OrderLevel[]>([]);
  const [visibleBids, setVisibleBids] = useState<OrderLevel[]>([]);

  // Spread calculation
  const [spread, setSpread] = useState<number>(0);

  const scrollContainerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const asksMap = asksMapRef.current;
    const bidsMap = bidsMapRef.current;

    // Updates the view based on current state
    const updateOrderbookView = () => {
      // Get sorted bids and asks
      const sortedBids = [...bidsMap.entries()].sort((a, b) => b[0] - a[0]); // Descending by price

      const sortedAsks = [...asksMap.entries()].sort((a, b) => a[0] - b[0]); // Ascending by price

      // Take only what we need to display
      const topBids = sortedBids.slice(0, depth).map(([price, quantity]) => ({
        price,
        quantity,
      }));

      const topAsks = sortedAsks.slice(0, depth).map(([price, quantity]) => ({
        price,
        quantity,
      }));

      // Calculate spread
      const highestBid = sortedBids.length > 0 ? sortedBids[0][0] : 0;
      const lowestAsk = sortedAsks.length > 0 ? sortedAsks[0][0] : Infinity;
      const currentSpread =
        lowestAsk !== Infinity && highestBid !== 0
          ? Math.max(0, lowestAsk - highestBid)
          : 0;

      // Update state
      setVisibleBids(topBids);
      setVisibleAsks(topAsks);
      setSpread(currentSpread);
    };

    const ws = new WebSocket(WS_URL);

    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          type: "subscribe",
          product_ids: [symbol],
          channel: "level2",
        }),
      );
    };

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (!data || data.channel !== "l2_data") return;

      const events = data.events;
      if (!events) return;

      let hasUpdates = false;

      // Process all updates
      for (const event of events) {
        if (!event.updates) continue;

        for (const update of event.updates) {
          const price = Number(update.price_level);
          const quantity = Number(update.new_quantity);

          if (update.side === "ask") {
            if (quantity > 0) {
              asksMap.set(price, quantity);
            } else {
              asksMap.delete(price);
            }
            hasUpdates = true;
          } else if (update.side === "bid") {
            if (quantity > 0) {
              bidsMap.set(price, quantity);
            } else {
              bidsMap.delete(price);
            }
            hasUpdates = true;
          }
        }
      }

      // Only update the view if we have changes
      if (hasUpdates) {
        updateOrderbookView();
      }
    };

    // Initial view update
    updateOrderbookView();

    // Scroll to the middle when the component mounts
    if (scrollContainerRef.current) {
      const container = scrollContainerRef.current;
      const middlePosition =
        container.scrollHeight / 2 - container.clientHeight / 2;
      container.scrollTop = middlePosition;
    }
  }, [symbol, depth]);

  // Pad arrays to ensure consistent display size
  const paddedAsks = [...visibleAsks];
  const paddedBids = [...visibleBids];

  while (paddedAsks.length < depth) {
    paddedAsks.push({ price: 0, quantity: 0 });
  }

  while (paddedBids.length < depth) {
    paddedBids.push({ price: 0, quantity: 0 });
  }

  return (
    <div className="flex flex-col h-[calc(100%-60px)] bg-[#121212] w-full max-w-md text-white border-l border-r border-[#2a2a2a]">
      <div className="flex justify-between px-4 py-2 text-gray-500 text-xs bg-[#121212] sticky top-0 z-10">
        <div className="w-1/2 text-left">Amount</div>
        <div className="w-1/2 text-right">Price</div>
      </div>

      <div
        ref={scrollContainerRef}
        className="flex-1 overflow-y-auto scrollbar-thin scrollbar-thumb-gray-800"
      >
        {/* Asks - display in ascending order (lowest ask at bottom) */}
        {paddedAsks.map((level, index) =>
          level.price > 0 ? (
            <AskRow
              key={`ask-${level.price || index}`}
              price={level.price}
              quantity={level.quantity}
            />
          ) : (
            <EmptyRow key={`empty-ask-${index}`} side="ask" />
          ),
        )}

        <div className="flex justify-between px-4 py-2 bg-[#1a1a1a] border-t text-xs border-b border-gray-800">
          <div className="text-gray-500">Spread</div>
          <div className="text-white">{spread.toFixed(0)}</div>
        </div>

        {/* Bids - display in descending order (highest bid at top) */}
        {paddedBids.map((level, index) =>
          level.price > 0 ? (
            <BidRow
              key={`bid-${level.price || index}`}
              price={level.price}
              quantity={level.quantity}
            />
          ) : (
            <EmptyRow key={`empty-bid-${index}`} side="bid" />
          ),
        )}
      </div>
    </div>
  );
}
