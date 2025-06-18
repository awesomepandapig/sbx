import Card, { CardImage } from "./Card";
import SVGCard from "./SVGCard";
import APICard from "./APICard";
import ThroughputCard from "./ThroughputCard";

export default function SectionTwo() {
  return (
    <div className="md:mt-12">
      {/* Heading Section */}
      <div className="w-full flex justify-center p-4">
        <h1 className="my-8 md:my-0 text-center leading-tight m-auto w-full font-semibold md:p-8 md:text-[44px] text-3xl bg-gradient-to-r from-orange-200 via-orange-300 to-70% to-white bg-clip-text text-transparent">
          Engineered for High-Frequency Trading
        </h1>
      </div>

      {/* Main Content Section */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <Card
          title="Open source"
          description="All code is OSI licensed, free for you to use as you please!"
          width="w-full" // Can adjust width as needed
          aspectRatio="aspect-square" // Aspect ratio control
          textPosition="center"
        >
          <a
            className="rounded-xl transition-transform hover:scale-110 duration-300"
            href="https://github.com/awesomepandapig/sbx"
            target="_blank"
          >
            <CardImage
              src="https://cdn-icons-png.flaticon.com/512/25/25231.png"
              alt="Redis logo"
            />
          </a>
        </Card>

        <ThroughputCard/>
        

        

        <APICard />
        
        <Card
          title="Lightning fast execution"
          description="Rust-based matching engine enables sub-millisecond tick-to-trade latency."
          width="w-full" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="center"
        >
          <div className="flex items-center justify-center text-gray-500">
          <div className="text-center relative">
            <div className="relative z-10">
              <div className="flex items-baseline justify-center gap-1">
                <span className="text-6xl font-light text-[#3CFFFF]">~100</span>
                <span className="text-2xl font-light text-[#3CFFFF] opacity-80">μs</span>
              </div>
              <div className="text-xs text-gray-400 uppercase tracking-wider mt-2 font-medium">tick-to-trade</div>
            </div>
          </div>
        </div>
        </Card>  
              
      </div>
    </div>
  );
}
