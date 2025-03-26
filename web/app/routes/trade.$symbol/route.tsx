import { useParams } from "@remix-run/react";
import TradingInterface from "~/components/trading/TradingInterface";
import Header from "~/components/trading/Header";
import OrderBook from "~/components/trading/OrderBook";
import { ChevronDown } from "lucide-react";
import { useEffect } from "react";
import { useNavigate } from "@remix-run/react";
import { useLoaderData } from "@remix-run/react";

import { authLoader } from "~/lib/auth"
export const loader = authLoader;

export default function Trade() {
  const { symbol } = useParams();
  if(!symbol) {
    // TODO: redirect to 404 page
    return 404;
  }

  const { user } = useLoaderData<typeof loader>();
  const navigate = useNavigate();

  // If user is logged in but not verified redirect to verification page
  useEffect(() => {
    if (user && !user.minecraftId) {
      navigate("/verify");
    }
  }, [user, navigate]);

  return (
    <main className="flex flex-col h-screen bg-black">
      <Header ticker={symbol} userImg={user?.image ?? undefined} />

      <div className="flex flex-1 overflow-hidden">
        <div className="flex flex-1 overflow-hidden">
          {/* Left section - Chart */}
          <div className="flex-1 min-w-0">
            <p className="text-white p-4">trading view chart</p>
          </div>

          {/* Middle section - Order Book */}
          <div className="w-64">
            <OrderBook />
          </div>

          {/* Right section - Trading Interface */}
          <div className="w-72">
            <TradingInterface symbol={symbol} authenticated={user != null}/>
          </div>
        </div>
      </div>

      <footer className="flex items-center justify-between px-4 py-2 bg-[#121212] border-t border-[#2a2a2a] text-sm">
        <div className="flex space-x-4">
          <button className="text-white">Orders</button>
          <button className="text-gray-400">Positions</button>
        </div>
        <div className="flex items-center">
          <button className="text-red-500 mr-4">Cancel all</button>
          <button className="flex items-center text-gray-400 bg-[#1E1E1E] border border-[#2a2a2a] rounded px-2 py-1">
            ALL MARKETS <ChevronDown size={14} className="ml-1" />
          </button>
          <button className="flex items-center text-gray-400 bg-[#1E1E1E] border border-[#2a2a2a] rounded px-2 py-1 ml-2">
            ALL STATUSES <ChevronDown size={14} className="ml-1" />
          </button>
        </div>
      </footer>
    </main>
  );
}
