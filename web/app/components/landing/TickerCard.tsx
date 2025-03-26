// import PriceChart from "./price-chart";
import { useState } from "react";
import { API_URL, DOMAIN } from "~/lib/config";

interface TickerCardProps {
  name: string;
  symbol: string;
  price: string;
  change: number;
  type?: string;
  color?: string;
  chartColor?: string;
  chartData?: number[];
  img: string;
}

const TickerCard = ({
  name,
  symbol,
  price,
  change,
  type = "Mineshaft",
  color = "blue",
  chartData,
  img,
}: TickerCardProps) => {
  const [isHovered, setIsHovered] = useState(false);

  const randomChartData = () =>
    Array.from({ length: 20 }, () => Math.floor(Math.random() * 30) + 30);

  const isPositiveChange = change >= 0;

  const changeStyle = isPositiveChange
    ? "bg-green-500/20 text-green-400"
    : "bg-red-500/20 text-red-400";

  return (
    <a href={`${DOMAIN}/trade/${symbol}`}>
      <div
        className={`min-w-[200px] rounded-lg border border-zinc-800 bg-zinc-900/80 backdrop-blur-sm hover:border-zinc-700 transition-all duration-300  
      
      `}
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
        style={{ boxShadow: isHovered ? `0 0 25px ${color}` : "none" }}
      >
        <div className="flex items-center p-3 border-b border-zinc-800">
          <div
            className={`w-10 h-10 rounded flex items-center justify-center mr-3`}
          >
            <img
              src={img}
              alt={name}
              width={24}
              height={24}
              className="object-contain"
            />
          </div>
          <div>
            <div className="flex items-center">
              <span className="font-bold text-white">{name}</span>
              <span className="ml-2 text-xs px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-400">
                {symbol}
              </span>
            </div>
            <div className="text-xs text-zinc-500">{type}</div>
          </div>
        </div>
        <div className="p-3">
          <div className="flex items-center justify-between">
            <div className="text-lg text-white font-bold">{price}</div>
            <div
              className={`text-sm font-medium px-2 py-1 rounded ${changeStyle}`}
            >
              {isPositiveChange && "+"}
              {change.toFixed(1)}%
            </div>
          </div>
        </div>
      </div>
    </a>
  );
};

export default TickerCard;
