"use client";

import { useEffect, useRef } from "react";
import { WS_URL } from "~/lib/config";

interface OrderBookProps {
  symbol: string;
}

interface OrderLevel {
  price: number;
  quantity: number;
}

// Helper function to update the DOM for asks and bids
function updateDOMRungs(
  asksData: OrderLevel[], // Sorted lowest price first, padded to depth
  bidsData: OrderLevel[], // Sorted highest price first, padded to depth
  currentDepth: number,
) {
  // Update Asks
  // asksData is sorted lowest price first. For display, we want highest price at the top of the ask section.
  const reversedAsksData = [...asksData].reverse(); // Highest price is now at index 0

  for (let i = 0; i < currentDepth; i++) {
    const askRungDiv = document.getElementById(`ask-row-${i}`);
    const askQtyDiv = document.getElementById(`ask-qty-${i}`);
    const askPriceDiv = document.getElementById(`ask-price-${i}`);

    if (!askRungDiv || !askQtyDiv || !askPriceDiv) continue;

    const askLevel = reversedAsksData[i]; //  Data for the i-th row from the top of asks section

    if (askLevel && askLevel.price > 0) {
      askQtyDiv.textContent = askLevel.quantity.toFixed(0);
      askPriceDiv.textContent = askLevel.price.toFixed(0);
      askPriceDiv.className = "w-1/2 text-right text-[#ff4d4d]";
      askRungDiv.className =
        "flex justify-between px-4 text-xs py-1 border-l-2 border-[#ff4d4d] bg-[#121212]";
    } else {
      askQtyDiv.textContent = "-";
      askPriceDiv.textContent = "-";
      askPriceDiv.className = "w-1/2 text-right"; // Reset color
      askRungDiv.className =
        "flex justify-between px-4 text-xs py-1 border-l-2 border-[#ff4d4d] bg-[#121212] opacity-30";
    }
  }

  // Update Bids
  // bidsData is sorted highest price first. This matches display order (highest price at top of bid section).
  for (let i = 0; i < currentDepth; i++) {
    const bidRungDiv = document.getElementById(`bid-row-${i}`);
    const bidQtyDiv = document.getElementById(`bid-qty-${i}`);
    const bidPriceDiv = document.getElementById(`bid-price-${i}`);

    if (!bidRungDiv || !bidQtyDiv || !bidPriceDiv) continue;

    const bidLevel = bidsData[i]; // Data for the i-th row from the top of bids section

    if (bidLevel && bidLevel.price > 0) {
      bidQtyDiv.textContent = bidLevel.quantity.toFixed(0);
      bidPriceDiv.textContent = bidLevel.price.toFixed(0);
      bidPriceDiv.className = "w-1/2 text-right text-[#4caf50]";
      bidRungDiv.className =
        "flex justify-between px-4 py-1 text-xs border-l-2 border-[#4caf50] bg-[#121212]";
    } else {
      bidQtyDiv.textContent = "-";
      bidPriceDiv.textContent = "-";
      bidPriceDiv.className = "w-1/2 text-right"; // Reset color
      bidRungDiv.className =
        "flex justify-between px-4 py-1 text-xs border-l-2 border-[#4caf50] bg-[#121212] opacity-30";
    }
  }
}

function shouldIncludePrice(
  map: Map<number, number>,
  newPrice: number,
  depth: number,
  side: "buy" | "sell",
): boolean {
  const prices = [...map.keys()];
  prices.push(newPrice);
  const sorted = prices.sort((a, b) => (side === "buy" ? b - a : a - b));
  return sorted.indexOf(newPrice) < depth;
}

export default function OrderBook({ symbol }: OrderBookProps) {
  const depth = 20;
  const asksMapRef = useRef(new Map<number, number>());
  const bidsMapRef = useRef(new Map<number, number>());
  const scrollContainerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const ws = new WebSocket(WS_URL);
    const asksMap = asksMapRef.current;
    const bidsMap = bidsMapRef.current;

    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          type: "subscribe",
          product_ids: [symbol],
          channel: "level2",
        }),
      );
    };

    let msg_count = 0;

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data as string);
      if (!data || data.channel !== "l2_data" || !data.events) return;

      console.log(msg_count);
      msg_count++;

      let hasUpdates = false;

      for (const ev of data.events) {
        if (!ev.updates) continue;
        for (const update of ev.updates) {
          const price = Number(update.price_level);
          const quantity = Number(update.new_quantity);
          const isBid = update.side === "buy";
          const map = isBid ? bidsMap : asksMap;
          const sortSide = isBid ? "buy" : "sell";

          if (quantity > 0) {
            if (shouldIncludePrice(map, price, depth, sortSide)) {
              map.set(price, quantity);
              hasUpdates = true;
            } else if (map.has(price)) {
              // Price moved out of depth, remove it
              map.delete(price);
              hasUpdates = true;
            }
          } else {
            if (map.has(price)) {
              map.delete(price);
              hasUpdates = true;
            }
          }
        }
      }

      if (hasUpdates) {
        // 1. Get sorted data from maps
        const currentAsks = [...asksMap.entries()]
          .sort((a, b) => a[0] - b[0]) // Sort by price: lowest first
          .slice(0, depth)
          .map(([price, quantity]) => ({ price, quantity }));

        const currentBids = [...bidsMap.entries()]
          .sort((a, b) => b[0] - a[0]) // Sort by price: highest first
          .slice(0, depth)
          .map(([price, quantity]) => ({ price, quantity }));

        // 2. Pad data for DOM update (to ensure `depth` elements)
        const asksForDOM = [...currentAsks];
        while (asksForDOM.length < depth) {
          asksForDOM.push({ price: 0, quantity: 0 });
        }

        const bidsForDOM = [...currentBids];
        while (bidsForDOM.length < depth) {
          bidsForDOM.push({ price: 0, quantity: 0 });
        }

        updateDOMRungs(asksForDOM, bidsForDOM, depth);

        // 3. Update Spread
        const highestBidPrice = currentBids[0]?.price ?? 0;
        const lowestAskPrice = currentAsks[0]?.price ?? 0;
        let newSpread = 0;
        if (
          lowestAskPrice > 0 &&
          highestBidPrice > 0 &&
          lowestAskPrice > highestBidPrice
        ) {
          newSpread = lowestAskPrice - highestBidPrice;
        }

        const spreadValueElement = document.getElementById("spread-value");
        if (spreadValueElement) {
          spreadValueElement.textContent = newSpread.toFixed(1); // Adjust precision as needed
        }
      }
    };

    ws.onerror = (error) => {
      console.error("WebSocket Error:", error);
    };

    ws.onclose = (event) => {
      console.log("WebSocket closed:", event);
      // Optionally clear the maps or show a disconnected state
      asksMap.clear();
      bidsMap.clear();
      const emptyAsks = Array(depth).fill({
        price: 0,
        quantity: 0,
      }) as OrderLevel[];
      const emptyBids = Array(depth).fill({
        price: 0,
        quantity: 0,
      }) as OrderLevel[];
      updateDOMRungs(emptyAsks, emptyBids, depth);
      const spreadValueElement = document.getElementById("spread-value");
      if (spreadValueElement) {
        spreadValueElement.textContent = (0).toFixed(1);
      }
    };

    // Scroll to the middle when the component mounts
    if (scrollContainerRef.current) {
      const container = scrollContainerRef.current;
      // Ensure this runs after the DOM elements are rendered and sized
      setTimeout(() => {
        const middle = container.scrollHeight / 2 - container.clientHeight / 2;
        container.scrollTop = middle;
      }, 0);
    }

    return () => {
      if (
        ws.readyState === WebSocket.OPEN ||
        ws.readyState === WebSocket.CONNECTING
      ) {
        ws.close();
      }
    };
  }, [symbol, depth]); // WS_URL can be added if it's dynamic

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
        {/* Asks - Render `depth` placeholders. `ask-row-0` is topmost, visually. */}
        {Array.from({ length: depth }).map((_, i) => (
          <div
            key={`ask-row-wrapper-${i}`}
            id={`ask-row-${i}`}
            className="flex justify-between px-4 text-xs py-1 border-l-2 border-[#ff4d4d] bg-[#121212] opacity-30"
          >
            <div id={`ask-qty-${i}`} className="w-1/2 text-left">
              -
            </div>
            <div id={`ask-price-${i}`} className="w-1/2 text-right">
              -
            </div>
          </div>
        ))}

        {/* Spread Div - Now part of the normal scroll flow */}
        <div className="flex justify-between px-4 py-2 bg-[#1a1a1a] border-t text-xs border-b border-gray-800">
          <div className="text-gray-500">Spread</div>
          <div id="spread-value" className="text-white">
            0.0
          </div>
        </div>

        {/* Bids - Render `depth` placeholders. `bid-row-0` is topmost, visually. */}
        {Array.from({ length: depth }).map((_, i) => (
          <div
            key={`bid-row-wrapper-${i}`}
            id={`bid-row-${i}`}
            className="flex justify-between px-4 py-1 text-xs border-l-2 border-[#4caf50] bg-[#121212] opacity-30"
          >
            <div id={`bid-qty-${i}`} className="w-1/2 text-left">
              -
            </div>
            <div id={`bid-price-${i}`} className="w-1/2 text-right">
              -
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}