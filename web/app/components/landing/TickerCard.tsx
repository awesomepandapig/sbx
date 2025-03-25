// import PriceChart from "./price-chart";
import { API_URL } from "~/lib/config";

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
  const randomChartData = () =>
    Array.from({ length: 20 }, () => Math.floor(Math.random() * 30) + 30);

  const data = chartData || randomChartData();
  const isPositiveChange = change >= 0;

  const changeStyle = isPositiveChange
    ? "bg-green-500/20 text-green-400"
    : "bg-red-500/20 text-red-400";

  return (
    <div className="min-w-[200px] rounded-lg border border-zinc-800 bg-zinc-900/80 backdrop-blur-sm hover:border-zinc-700 transition-all duration-300 overflow-hidden shadow-lg">

      <div className="flex items-center p-3 border-b border-zinc-800">
        <div className={`w-10 h-10 rounded bg-${color}-500/10 flex items-center justify-center mr-3`}>
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
        <div className="flex items-center justify-between mb-2">
          <div className="text-lg font-bold">{price}</div>
          <div className={`text-sm font-medium px-2 py-1 rounded ${changeStyle}`}>
            {isPositiveChange && "+"}
            {change.toFixed(1)}%
          </div>
        </div>
        {/* <PriceChart
          data={data}
          color={isPositiveChange ? "#10B981" : "#EF4444"}
          height={40}
        /> */}
      </div>
    </div>
  );
};

export default TickerCard;