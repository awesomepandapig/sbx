import Card, { CardImage } from "./Card";
import SVGCard from "./SVGCard";
import APICard from "./APICard";

export default function SectionTwo() {
  return (
    <div className="md:mt-12">
      {/* Heading Section */}
      <div className="w-full flex justify-center p-4">
        <h1 className="my-8 md:my-0 text-center leading-tight m-auto w-full font-semibold md:p-8 md:text-[44px] text-3xl bg-gradient-to-r from-orange-200 via-orange-300 to-70% to-white bg-clip-text text-transparent">
          Engineered for Extreme Throughput
        </h1>
      </div>

      {/* Main Content Section */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <Card
          title="Open source"
          description="All code is MIT licensed, free for you to use as you please!"
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

        <Card
          title="Trade structures"
          description="Trade Fairy & Jasper mineshafts, Dragon's Lairs, and more."
          width="lg:col-span-2" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="left"
        >
          <p></p>
        </Card>

        <Card
          title="Lightning fast"
          description="Aeron® powered matching engine enables sub-millisecond trade executions."
          width="w-full" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="center"
        >
          <p>Replace with animation of orders incoming</p>
        </Card>

        <APICard />
      </div>
    </div>
  );
}
