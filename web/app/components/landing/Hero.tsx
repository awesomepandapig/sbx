import { ArrowUpRight} from "lucide-react";
import TickerTable from "./TickerTable";

export default function Hero() {
  return (
    <div className="mt-4 grid grid-cols-1 md:grid-cols-5 md:grid-rows-1 gap-4">
      <div className="md:col-span-3 bg-[#0e0e0e] border border-[rgba(38,38,38,.7)] rounded-[20px] p-8 flex flex-col justify-between">
        <h1 className="text-[#3CFFFF] text-[clamp(2rem,6vw,4.5rem)] font-bold leading-none mb-4">
          EXCHANGE
          <br />
          SKYBLOCK
          <br />
          STRUCTURES
        </h1>

        <div>
          <p className="text-gray-400 mb-6 max-w-md">
            SBX is a high-performance trading platform designed for Hypixel
            SkyBlock.
          </p>
          <a href="/trade/JSP">
            <button className="bg-[#3CFFFF] text-black px-6 py-3 rounded-full text-sm font-medium flex flex-row items-center">
              GET STARTED
              <ArrowUpRight className="ml-2" size={18} />
            </button>
          </a>
        </div>
      </div>

      <TickerTable/>
    </div>
  );
}