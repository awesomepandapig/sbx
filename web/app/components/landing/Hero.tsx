const cubes: { position: string }[] = [
  { position: "translate-y-4" },
  { position: "translate-y-8" },
  { position: "translate-y-4" },
  { position: "translate-y-8" },
  { position: "translate-y-4" },
];

import TickerCard from "./TickerCard";

const marketData = [
    {
      name: "FAIRY",
      symbol: "FRY",
      price: "1.2M",
      change: -12.5,
      type: "Mineshaft",
      color: "cyan",
      chartData: [
        62, 60, 58, 55, 53, 50, 48, 45, 43, 40, 38, 35, 33, 30, 32, 35, 33, 30,
        28, 25,
      ],
      position: "translate-y-4",
      img: "/Vanguard_Helmet.png"
    },
    {
      name: "DRAGON",
      symbol: "DRG",
      price: "4.8M",
      change: 8.3,
      type: "Lair",
      color: "amber",
      chartData: [
        30, 32, 35, 33, 35, 38, 40, 42, 45, 47, 50, 48, 50, 53, 55, 58, 60, 62,
        65, 68,
      ],
      position: "translate-y-8",
      img: "https://wiki.hypixel.net/images/7/75/SkyBlock_pets_golden_dragon.png"
    },
    {
      name: "JASPER",
      symbol: "JSP",
      price: "2.4M",
      change: -3.7,
      type: "Mineshaft",
      color: "fuchsia",
      chartData: [
        50, 48, 50, 47, 45, 43, 45, 47, 45, 43, 40, 42, 40, 38, 40, 38, 35, 37,
        35, 33,
      ],
      position: "translate-y-4",
      img: "https://wiki.hypixel.net/images/7/72/SkyBlock_items_jasper_crystal.png"
    },
  ];

export default function Hero() {
  return (
    <div className="w-full flex justify-center mt-28">
      <div className="z-0 flex flex-col items-center text-left mb-16">
        <h1 className="text-5xl md:text-7xl font-black tracking-tight mb-6">
          Skyblock<span className="text-blue-500">.</span>Exchange
        </h1>
        <p className="text-xl text-gray-400 max-w-2xl mb-8">
          A real-time marketplace for Skyblock structures
        </p>
        <div className="flex flex-col sm:flex-row gap-4">
          <button className="bg-blue-600 hover:bg-blue-700/80 h-12 text-white px-4 py-2 rounded-xl duration-300">
            Start Trading
          </button>
          <button className="border bg-[rgba(38,38,38,.7)] border-gray-700 hover:bg-gray-700/80 duration-300 h-12 text-white px-4 py-2 rounded-xl">
            Explore Docs
          </button>
        </div>
        <CubeGrid cubes={cubes} />
      </div>
    </div>
  );
}

function CubeGrid({ cubes }: { cubes: { position: string }[] }) {
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
                  chartData={ticker.chartData}
                  img={ticker.img}
                />
        </div>
      ))}
    </div>
  );
}
