import { useParams } from "@remix-run/react";
import TradingInterface from "~/components/trading/TradingInterface";
import Header from "~/components/trading/Header";
import OrderBook from "~/components/trading/OrderBook";
import { useEffect } from "react";
import { useNavigate } from "@remix-run/react";
import { useLoaderData } from "@remix-run/react";
import OrderDrawer from "~/components/trading/OrderDrawer";

import { authLoader } from "~/lib/auth";
export const loader = authLoader;

export default function Trade() {
  const { symbol } = useParams();
  if (!symbol) {
    throw new Response(null, {
      status: 404,
      statusText: "Not Found",
    });
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
