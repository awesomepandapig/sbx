import { ChevronDown } from "lucide-react";
import AvatarWithMenu from "./Menu";

interface HeaderProps {
  ticker: string;
  userImg?: string;
}

type SignInButtonsProps = {
  userImg?: string;
};

const SignInButtons = ({ userImg }: SignInButtonsProps) => (
  <div className="flex items-center space-x-3">
    {userImg ? (
      <AvatarWithMenu userImg={userImg} />
    ) : (
      <>
        <button className="px-4 py-2 bg-[#1E1E1E] text-white rounded-full">
          Sign in
        </button>
        <button className="px-4 py-2 bg-[#4169E1] text-white rounded-full">
          Sign up
        </button>
      </>
    )}
  </div>
);

const StatBlock = ({
  label,
  value,
  priceChange,
}: {
  label: string;
  value: string | number;
  priceChange?: number;
}) => (
  <div className="flex flex-col">
    <span className="text-gray-400 text-xs">{label}</span>
    <div className="flex items-center">
      <span className="text-white font-medium">${value}</span>
      {priceChange && (
        <span
          className={`ml-2 ${priceChange < 0 ? "text-red-500" : "text-green-500"}`}
        >
          {priceChange > 0 ? "+" : ""}
          {priceChange}%
        </span>
      )}
    </div>
  </div>
);

const TickerSelect = ({ ticker }: { ticker: string }) => (
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
      <span className="text-white font-semibold ml-6">{ticker}</span>
      <button className="text-white ml-1">
        <ChevronDown size={18} />
      </button>
    </div>
  </div>
);

export default function Header({ ticker, userImg }: HeaderProps) {
  let lastPrice = 88021.75;
  let priceChange = -0.45;
  let volume = "968,643,980.90";
  let high = 88600.0;
  let low = 86321.97;

  return (
    <header className="flex flex-row items-center justify-between p-3 bg-[#121212] border-b border-[#2a2a2a]">
      <div className="flex items-center">
        <TickerSelect ticker={ticker} />
        <div className="flex space-x-6">
          <StatBlock
            label="Last Price (24H)"
            value={lastPrice.toLocaleString()}
            priceChange={priceChange}
          />
          <StatBlock label="24H Volume" value={volume} />
          <StatBlock label="24H High" value={high.toLocaleString()} />
          <StatBlock label="24H Low" value={low.toLocaleString()} />
        </div>
      </div>
      <SignInButtons userImg={userImg} />
    </header>
  );
}
