"use client";

import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import { ChevronUp } from "lucide-react";
import { API_URL } from "~/lib/config";
import { WS_URL } from "~/lib/config";

// import activeProducts

const activeProducts = ["JSP", "DRG", "FRY"];

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
      // if (!data || data.channel !== "user") return;

      // const events = data.events;
      // if (!events) return;

      // console.log(events);
      console.log(data);

      // if(event.type == 'snapshot') {
      //   setOrders(orders);
      // }

      // for (const event of events) {

      // }
    };
  }

  useEffect(() => {
    if (!authenticated) return;
    // Get a JWT

    async function init() {
      const jwt = await getToken();
      if (jwt) {
        // TODO: Uncomment
        // createSocket(jwt);
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
            className={`flex items-center text-gray-400 bg-[#1E1E1E] border border-[#2a2a2a] rounded px-2 py-1 ${
              !authenticated ? "opacity-50" : ""
            }`}
            disabled={!authenticated}
          >
            ALL MARKETS
          </button>
          <button
            className={`flex items-center text-gray-400 bg-[#1E1E1E] border border-[#2a2a2a] rounded px-2 py-1 ml-2 ${
              !authenticated ? "opacity-50" : ""
            }`}
            disabled={!authenticated}
          >
            ALL STATUSES
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

        {/* {orders.length === 0 ? (
          <p className="text-gray-400 mt-4">No orders found</p>
        ) : (
          <div className="overflow-x-auto mt-4 max-h-[calc(80vh-60px)] overflow-y-auto">
            <table className="w-full border-collapse border border-[#2a2a2a] text-white text-sm">
              <thead className="bg-[#1E1E1E] sticky top-0 z-10">
                <tr className="bg-[#1E1E1E] text-gray-400">
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
                  ].map((header) => (
                    <th
                      key={header}
                      className="border border-[#2a2a2a] px-2 py-1 text-left"
                    >
                      {header}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {orders.map((order) => (
                  <tr key={order.id} className="border border-[#2a2a2a]">
                    <td className="px-2 py-1">{order.product_id}</td>
                    <td className="px-2 py-1">{order.side}</td>
                    <td className="px-2 py-1">{order.type}</td>
                    <td className="px-2 py-1">{order.created_at ?? "N/A"}</td>
                    <td className="px-2 py-1">{order.executed_value}</td>
                    <td className="px-2 py-1">{order.status}</td>
                    <td className="px-2 py-1">
                      {order.settled ? "Yes" : "No"}
                    </td>
                    <td className="px-2 py-1">{order.size}</td>
                    <td className="px-2 py-1">{order.price ?? "N/A"}</td>
                    <td className="px-2 py-1">{order.cancel_after}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )} */}
      </div>
    </motion.div>
  );
}
