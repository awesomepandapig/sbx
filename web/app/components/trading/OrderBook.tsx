"use client"

import { useState } from "react"

interface OrderData {
  amount: string
  price: string
}

export default function OrderBook() {
  const [activeTab, setActiveTab] = useState("orderbook")

  // Sample data for asks (sell orders) - in descending price order
  const asks: OrderData[] = [
    { amount: "0.1794131", price: "83453.27" },
    { amount: "0.00552137", price: "83453.26" },
    { amount: "0.00052542", price: "83449.90" },
    { amount: "0.02586384", price: "83449.69" },
    { amount: "0.1344236", price: "83449.52" },
    { amount: "0.07841755", price: "83449.49" },
    { amount: "0.17975058", price: "83448.97" },
    { amount: "0.01478802", price: "83445.90" },
    { amount: "0.0236296", price: "83445.52" },
    { amount: "0.16993152", price: "83444.45" },
    { amount: "0.00012465", price: "83444.44" },
    { amount: "0.13492766", price: "83443.88" },
  ]

  // Sample data for bids (buy orders) - in descending price order
  const bids: OrderData[] = [
    { amount: "0.00060992", price: "83443.87" },
    { amount: "0.00012465", price: "83438.77" },
    { amount: "0.01478928", price: "83438.76" },
    { amount: "0.00005992", price: "83438.00" },
    { amount: "0.16625526", price: "83436.89" },
    { amount: "0.0522170", price: "83436.47" },
    { amount: "0.02996319", price: "83435.68" },
    { amount: "0.00100000", price: "83434.37" },
    { amount: "0.00549173", price: "83434.28" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
    { amount: "0.11985783", price: "83432.17" },
    { amount: "0.02378584", price: "83430.67" },
    { amount: "0.1248000", price: "83430.50" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
    { amount: "0.02353571", price: "83434.27" },
    { amount: "0.11985522", price: "83433.99" },
    { amount: "0.06159327", price: "83433.38" },
    { amount: "0.2142951", price: "83432.79" },
  ]

  // Calculate the spread
  const lowestAsk = asks.length > 0 ? Number.parseFloat(asks[asks.length - 1].price) : 0
  const highestBid = bids.length > 0 ? Number.parseFloat(bids[0].price) : 0
  const spread = lowestAsk - highestBid
  const spreadFormatted = spread.toFixed(2)

  return (
    <div className="flex flex-col h-[calc(100%-60px)] bg-[#121212] w-full max-w-md text-white border-l border-r border-[#2a2a2a]">
      {/* Column headers */}
      <div className="flex justify-between px-4 py-2 text-gray-500 text-xs bg-[#121212] sticky top-0 z-10">
        <div className="w-1/2 text-left">Amount</div>
        <div className="w-1/2 text-right">Price</div>
      </div>

      {/* Single scrollable container for all content */}
      <div className="flex-1 overflow-y-auto scrollbar-thin scrollbar-thumb-gray-800">
        {/* Asks (sell orders) */}
        {asks.map((ask, index) => (
          <div key={`ask-${index}`} className="flex justify-between px-4 text-xs py-1 border-l-2 border-[#ff4d4d] bg-[#121212]">
            <div className="w-1/2 text-left">{ask.amount}</div>
            <div className="w-1/2 text-right text-[#ff4d4d]">{ask.price}</div>
          </div>
        ))}

        {/* Spread */}
        <div className="flex justify-between px-4 py-2 bg-[#1a1a1a] border-t text-xs border-b border-gray-800">
          <div className="text-gray-500">Spread</div>
          <div className="text-white">{spreadFormatted}</div>
        </div>

        {/* Bids (buy orders) */}
        {bids.map((bid, index) => (
          <div key={`bid-${index}`} className="flex justify-between px-4 py-1 text-xs border-l-2 border-[#4caf50] bg-[#121212]">
            <div className="w-1/2 text-left">{bid.amount}</div>
            <div className="w-1/2 text-right text-[#4caf50]">{bid.price}</div>
          </div>
        ))}
      </div>
    </div>
  )
}