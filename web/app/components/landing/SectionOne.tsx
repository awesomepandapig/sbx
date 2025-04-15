import Card from "./Card";
import { Heart } from "lucide-react";
import PartyInvite from "./PartyInvite";
import AutoScrollingCarousel from "./Carousel";

export default function SectionOne() {
  const sectionStyle = {
    maxWidth: "75em",
    width: "100%",
    marginLeft: "auto",
    marginRight: "auto",
  };

  return (
    <div className="mt-8">
      {/* Heading Section */}
      <div className="w-full flex justify-center">
        <h1 className="text-center m-auto w-fit font-semibold mb-4 text-[44px] bg-gradient-to-r from-pink-200 via-pink-300 to-70% to-white bg-clip-text text-transparent">
          Never Search for Structures Again
        </h1>
      </div>

      {/* Main Content Section */}
      <div
        className="flex flex-row gap-8 p-8 justify-center"
        style={sectionStyle}
      >
        <Card
          title="Trade structures"
          description="Trade Fairy & Jasper Mineshafts, Dragon's Lairs, and more."
          width="w-7/12"
          textPosition="left"
        >
          <AutoScrollingCarousel />
        </Card>

        <Card
          title="Party invites"
          description="Party invites are automatically generated once a trade executes."
          width="w-96"
          aspectRatio="aspect-square"
          textPosition="left"
        >
          <PartyInvite />
        </Card>
      </div>

      {/* Second Main Content Section */}
      <div
        className="flex flex-row gap-8 pl-8 pr-8 pb-8 justify-center"
        style={sectionStyle}
      >
        <Card
          title="Charity Mode"
          description="Donate structures to Ironmen players."
          width="w-96"
          aspectRatio="aspect-square"
          textPosition="center"
        >
          <Heart
            className="text-red-600 h-20 w-20"
            style={{ fill: "#dc2626" }}
          />
        </Card>

        <Card
          title="Data at your fingertips"
          description="Access historical trade data instantly."
          width="w-7/12"
          textPosition="center"
        >
          <p>replace with graph</p>
        </Card>
      </div>
    </div>
  );
}
