import { ChevronDown } from "lucide-react";
import AvatarMenu from "./AvatarMenu";

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

interface HeaderProps {
  symbol: string;
  userImg?: string;
  tickerData: Ticker | undefined;
}

type SignInButtonsProps = {
  userImg?: string;
};

const StatBlock = ({
  label,
  value,
  priceChange,
}: {
  label: string;
  value: string | number | undefined;
  priceChange?: number;
}) => (
  <div className="flex flex-col">
    <span className="text-gray-400 text-xs">{label}</span>
    <div className="flex items-center">
      <span className="text-white text-sm font-medium tabular-nums">
        {typeof value === "number" ? (
          `$${value.toLocaleString()}`
        ) : (
          <span className="inline-block h-5 w-[10ch] bg-gray-700 animate-pulse rounded"></span>
        )}
      </span>
      {typeof priceChange === "number" && (
        <span
          className={`text-sm ml-4 ${priceChange < 0 ? "text-red-500" : "text-green-500"}`}
        >
          {priceChange > 0 ? "+" : ""}
          {priceChange.toFixed(4)}%
        </span>
      )}
    </div>
  </div>
);

const SymbolSelect = ({ symbol }: { symbol: string }) => (
  <div className="mr-8 bg-gray-800 p-2 rounded-full flex items-center">
    <div className="flex items-center">
      <div className="flex-row flex">
        <div className="w-8 h-8 rounded-full bg-blue-500 z-10">
          <img
            src="/Vanguard_Helmet.png"
            alt="Helmet"
            className="w-full h-full"
          />
        </div>
        <div className="w-8 h-8 rounded-full bg-amber-500 -ml-2 flex items-center justify-center text-amber-700">
          $
        </div>
      </div>
      <span className="text-white font-semibold ml-6">{symbol}</span>
      <button className="text-white ml-1">
        <ChevronDown size={18} />
      </button>
    </div>
  </div>
);

export default function Header({ symbol, userImg, tickerData }: HeaderProps) {
  return (
    <header className="flex flex-row items-center justify-between p-3 bg-[#121212] border-b border-[#2a2a2a]">
      <div className="flex items-center">
        <SymbolSelect symbol={symbol} />
        <div className="flex space-x-6">
          <StatBlock
            label="Last Price (24H)"
            value={tickerData?.price}
            priceChange={tickerData?.price_percent_chg_24h}
          />
          <StatBlock label="24H Volume" value={tickerData?.volume_24h} />
          <StatBlock label="24H High" value={tickerData?.high_24h} />
          <StatBlock label="24H Low" value={tickerData?.low_24h} />
        </div>
      </div>
      {userImg ? <AvatarMenu userImg={userImg} /> : <></>}
    </header>
  );
}
