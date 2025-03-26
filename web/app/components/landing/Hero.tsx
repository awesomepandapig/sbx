import TickerCard from "./TickerCard";
import { getSession, signIn } from "~/lib/auth";
import { DOMAIN } from "~/lib/config";

const marketData = [
  {
    name: "FAIRY",
    symbol: "FRY",
    price: "1.2M",
    change: -12.5,
    type: "Mineshaft",
    color: "rgb(15,94,155,1)",
    position: "translate-y-4",
    img: "/Vanguard_Helmet.png",
  },
  {
    name: "DRAGON",
    symbol: "DRG",
    price: "4.8M",
    change: 8.3,
    type: "Lair",
    color: "rgba(251,178,31,1)",
    position: "translate-y-8",
    img: "https://wiki.hypixel.net/images/7/75/SkyBlock_pets_golden_dragon.png",
  },
  {
    name: "JASPER",
    symbol: "JSP",
    price: "2.4M",
    change: -3.7,
    type: "Mineshaft",
    color: "rgba(172,45,136,1)",
    position: "translate-y-4",
    img: "https://wiki.hypixel.net/images/7/72/SkyBlock_items_jasper_crystal.png",
  },
];

interface HeroProps {
  authenticated: boolean;
}

export default function Hero({ authenticated }: HeroProps) {
  async function getStarted() {
    if (authenticated) {
      window.location.href = `${DOMAIN}/trade/FRY`;
    } else {
      signIn();
    }
  }

  return (
    <div className="w-full flex justify-center mt-28">
      <div className="z-0 flex flex-col items-center text-left mb-16">
        <h1 className="text-5xl md:text-7xl font-black text-white tracking-tight mb-6">
          Skyblock<span className="text-blue-500">.</span>Exchange
        </h1>
        <p className="text-xl text-gray-400 max-w-2xl mb-8">
          A real-time marketplace for Skyblock structures
        </p>
        <div className="flex flex-col sm:flex-row gap-4">
          <button
            className="bg-blue-600 hover:bg-blue-700/80 h-12 text-white px-4 py-2 rounded-xl duration-300"
            onClick={getStarted}
          >
            Get Started
          </button>
          <button className="border bg-[rgba(38,38,38,.7)] border-gray-700 hover:bg-gray-700/80 duration-300 h-12 text-white px-4 py-2 rounded-xl">
            Explore Docs
          </button>
        </div>
        <TickerGrid />
      </div>
    </div>
  );
}

function TickerGrid() {
  return (
    <div className="grid grid-cols-2 md:grid-cols-3 gap-4 mt-16 mb-16 max-w-4xl">
      {marketData.map((ticker, index) => (
        <div
          key={index}
          className={`${ticker.position} transition-transform hover:scale-110 duration-300`}
        >
          {/* TODO: Make each card a link to the respective market */}
          <TickerCard
            key={index}
            name={ticker.name}
            symbol={ticker.symbol}
            price={ticker.price}
            change={ticker.change}
            type={ticker.type}
            color={ticker.color}
            img={ticker.img}
          />
        </div>
      ))}
    </div>
  );
}
