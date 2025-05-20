"use client";

import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import { ChevronUp, ChevronDown } from "lucide-react";
import { API_URL } from "~/lib/config";
import { WS_URL } from "~/lib/config";

// TODO: import activeProducts
const activeProducts = ["JSP"];

async function getToken() {
  try {
    const result = await fetch(`${API_URL}/auth/token`, {
      credentials: "include",
    });
    if (!result.ok) {
      throw new Error("Authentication failed");
    }

    const data = await result.json();
    return data.token;
  } catch (error) {}
}

async function cancelOrder(orderId: string, productId: string) {
  try {
    const response = await fetch(
      `${API_URL}/orders/${orderId}?product_id=${productId}`,
      {
        method: "DELETE",
        credentials: "include",
      },
    );
    if (!response.ok) {
      throw new Error("Unable to cancel order");
    }
  } catch (error) {
    // TODO: Handle error
    console.error(error);
  }
}

function formatTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });
}

function formatPrice(price: string | null): string {
  if (price === null) return "Market";
  const p = parseFloat(price);
  const formatted = p < 100 ? p.toFixed(2) : p.toFixed(0);
  return Number(formatted).toLocaleString();
}

interface OrderDrawerProps {
  authenticated: boolean;
}

export default function OrderDrawer({ authenticated }: OrderDrawerProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [orders, setOrders] = useState([]);
  const [selectedProducts, setSelectedProducts] = useState(activeProducts);

  async function createSocket(jwt: string) {
    const ws = new WebSocket(WS_URL);
    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          type: "subscribe",
          product_ids: selectedProducts,
          channel: "user",
          jwt: jwt,
        }),
      );
    };

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (!data || data.channel !== "user") return;

      const events = data.events;
      if (!events) return;

      for (const evt of events) {
        if (evt.type == "snapshot") {
          const temp = [];
          for (const update of evt.updates) {
            temp.push(update);
          }
          setOrders(temp);
        }
        if (evt.type == "update") {
          // TODO:
        }
      }
    };
  }

  useEffect(() => {
    if (!authenticated) return;
    // Get a JWT

    async function init() {
      const jwt = await getToken();
      if (jwt) {
        // TODO: Uncomment
        createSocket(jwt);
      }
    }

    init();
  }, [authenticated, selectedProducts]);

  function DrawerHeader() {
    return (
      <div className="flex justify-between">
        <div className="flex space-x-4">
          <button
            className={`text-white ${!authenticated ? "opacity-50" : ""}`}
            disabled={!authenticated}
          >
            Orders
          </button>
        </div>
        <div className="flex items-center">
          <button
            className={`flex items-center justify-between text-gray-400 bg-[#1E1E1E] border border-[#2a2a2a] rounded px-2 py-1 ${
              !authenticated ? "opacity-50" : ""
            }`}
            disabled={!authenticated}
          >
            <span>ALL MARKETS</span>
            <ChevronDown size={16} className="ml-2" />
          </button>
          <button
            className={`flex items-center text-gray-400 bg-[#1E1E1E] border border-[#2a2a2a] rounded px-2 py-1 ml-2 ${
              !authenticated ? "opacity-50" : ""
            }`}
            disabled={!authenticated}
          >
            <span>ALL STATUSES</span>
            <ChevronDown size={16} className="ml-2" />
          </button>

          {/* TODO: Add live data to status indicator */}
          <div
            className={`flex gap-2 items-center text-green-500 bg-[#1E1E1E] border border-green-900 rounded px-2 py-1 ml-2
            }`}
          >
            ONLINE{" "}
            <span className="inline-block h-2 w-2 rounded-full bg-green-500"></span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <motion.div
      className="fixed bottom-0 left-0 w-full bg-[#121212] border-t border-[#2a2a2a] text-sm rounded-t-2xl shadow-lg"
      animate={{ height: isExpanded ? "80vh" : "60px" }}
      transition={{ type: "spring", stiffness: 200, damping: 30 }}
      drag="y"
      initial={false}
      dragConstraints={{ top: 0, bottom: 0 }}
      onDragEnd={(_, info) => {
        if (info.offset.y < -50) setIsExpanded(true);
        if (info.offset.y > 50) setIsExpanded(false);
      }}
    >
      {/* Expand Button */}
      <div className="flex justify-center items-center relative">
        <button
          className="absolute -top-3 w-8 h-8 bg-[#1E1E1E] text-gray-400 rounded-full flex items-center justify-center border border-[#2a2a2a] shadow-md"
          onClick={() => setIsExpanded(!isExpanded)}
        >
          <ChevronUp
            size={16}
            className={`transition-transform ${isExpanded ? "rotate-180" : ""}`}
          />
        </button>
      </div>

      {/* Drawer Content */}
      <div className="p-4" onDoubleClick={() => setIsExpanded((prev) => !prev)}>
        <DrawerHeader />

        {orders.length === 0 ? (
          <p className="text-gray-400 mt-4">No orders found</p>
        ) : (
          <div className="overflow-x-auto mt-4 max-h-[calc(80vh-60px)] overflow-y-auto">
            <table className="w-full border-collapse text-white text-sm">
              <thead className="bg-[#121212] sticky top-0 z-10">
                <tr className="text-gray-400 border-b border-[#2a2a2a]">
                  {[
                    "Product",
                    "Side",
                    "Type",
                    "Created At",
                    "Executed Value",
                    "Status",
                    "Settled",
                    "Size",
                    "Price",
                    "Cancel After",
                    "",
                  ].map((header) => (
                    <th
                      key={header}
                      className="px-3 py-2 text-left font-medium"
                    >
                      {header}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {orders.map((order) => (
                  <tr
                    key={order.id}
                    className="border-b border-[#2a2a2a] hover:bg-[#1a1a1a] transition-colors"
                  >
                    <td className="px-3 py-2.5">
                      <span
                        className={`font-medium px-3 py-1 rounded text-center inline-block min-w-[60px] ${
                          order.product_id === "JSP"
                            ? "bg-[#1E1E1E]"
                            : order.product_id === "BTC"
                              ? "bg-[#332200] text-yellow-500"
                              : order.product_id === "ETH"
                                ? "bg-[#1a2b47] text-blue-400"
                                : "bg-[#2a1a30] text-purple-400"
                        }`}
                      >
                        {order.product_id}
                      </span>
                    </td>
                    <td className="px-3 py-2.5">
                      <span
                        className={`font-medium ${order.side === "buy" ? "text-green-500" : "text-red-500"}`}
                      >
                        {order.side}
                      </span>
                    </td>
                    <td className="px-3 py-2.5">
                      <span
                        className={`${order.type === "market" ? "text-yellow-500" : "text-gray-300"}`}
                      >
                        {order.type}
                      </span>
                    </td>
                    <td className="px-3 py-2.5 text-gray-400">
                      {formatTime(order.created_at)}
                    </td>
                    <td className="px-3 py-2.5">
                      {order.executed_value > 0 ? (
                        <span className="text-green-400">
                          ${order.executed_value.toLocaleString()}
                        </span>
                      ) : (
                        "0"
                      )}
                    </td>
                    <td className="px-3 py-2.5">
                      <span
                        className={`px-2 py-0.5 rounded ${
                          order.status === "open"
                            ? "bg-blue-900/30 text-blue-400"
                            : order.status === "filled"
                              ? "bg-green-900/30 text-green-400"
                              : order.status === "canceled"
                                ? "bg-red-900/30 text-red-400"
                                : "bg-gray-800 text-gray-400"
                        }`}
                      >
                        {order.status}
                      </span>
                    </td>
                    <td className="px-3 py-2.5 text-gray-400">
                      {order.settled ? "Yes" : "No"}
                    </td>
                    <td className="px-3 py-2.5 font-medium">{order.size}</td>
                    <td className="px-3 py-2.5 font-medium">
                      {formatPrice(order.price)}
                    </td>
                    <td className="px-3 py-2.5 text-gray-400">
                      {order.cancel_after}
                    </td>
                    <td className="px-3 py-2.5">
                      {order.status === "open" ? (
                        <button
                          className="flex items-center text-red-400 bg-red-900/20 border border-red-900/50 rounded px-3 py-1 text-xs font-medium hover:bg-red-900/30 transition-colors"
                          onClick={() =>
                            cancelOrder(order.id, order.product_id)
                          }
                        >
                          CANCEL
                        </button>
                      ) : (
                        <span className="text-gray-600 px-3 py-1 text-xs">
                          —
                        </span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </motion.div>
  );
}
