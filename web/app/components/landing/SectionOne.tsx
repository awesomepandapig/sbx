import Card, { CardImage } from "./Card";
import { Heart, Plus, User } from "lucide-react";
import PartyInvite from "./PartyInvite";
import AutoScrollingCarousel from "./Carousel";
import { PriceGraph } from "./PriceGraph";
import SVGCard from "./SVGCard";

export default function SectionOne() {
  return (
    <div className="md:mt-24">
      {/* Heading Section */}
      <div className="w-full flex justify-center p-4">
        <h1 className="my-8 md:my-0 text-center leading-tight m-auto w-full font-semibold md:p-8 md:text-[44px] text-3xl bg-gradient-to-r from-pink-200 via-pink-300 to-70% to-white bg-clip-text text-transparent">
          Never Search for Structures Again
        </h1>
      </div>

      {/* Main Content Section */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <Card
          title="Trade structures"
          description="Trade Fairy & Jasper Mineshafts, Dragon's Lairs, and more."
          width="w-full"
          textPosition="left"
        >
          <AutoScrollingCarousel />
        </Card>

        <Card
          title="Automate invites"
          description="Party invites are automatically generated once a trade executes."
          width="w-full"
          aspectRatio="aspect-square"
          textPosition="left"
        >
          <PartyInvite />
        </Card>

        <Card
          title="Maximize your earnings"
          description="Sell to up to three players per structure."
          width="w-full"
          aspectRatio="aspect-square"
          textPosition="center"
        >
          <div className="gap-4 flex flex-row items-center">
            <div
              className={`aspect-square text-white bg-[rgba(255,255,255,0.06)] rounded-xl`}
            >
              <img
                src="https://mc-heads.net/avatar/f7c77d999f154a66a87dc4a51ef30d19"
                className="w-14 rounded-xl"
              />
            </div>

            <div
              className={`aspect-square text-white bg-[rgba(255,255,255,0.06)] rounded-xl`}
            >
              <img
                src="https://mc-heads.net/avatar/20934ef9488c465180a78f861586b4cf"
                className="w-14 rounded-xl"
              />
            </div>

            <div
              className={`aspect-square text-white bg-[rgba(255,255,255,0.06)] rounded-xl`}
            >
              <img
                src="https://mc-heads.net/avatar/70a54f854e5d49db83e8f9329912a3a9"
                className="w-14 rounded-xl"
              />
            </div>
          </div>
        </Card>

        <Card
          title="Donate structures"
          description="Donate structures to Ironmen players."
          width="w-full"
          aspectRatio="aspect-square"
          textPosition="center"
        >
          <Heart
            className="text-red-600 h-20 w-20"
            style={{ fill: "#dc2626" }}
          />
        </Card>

        <SVGCard
          title="Analyze the market"
          description="Access detailed historical trade data."
          width="lg:col-span-2"
          textPosition="center"
        >
          <PriceGraph />
        </SVGCard>
      </div>
    </div>
  );
}
