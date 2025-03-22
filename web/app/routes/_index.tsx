import React, { useEffect, ReactNode, FC } from "react";
import Splide from "@splidejs/splide";
import { AutoScroll } from "@splidejs/splide-extension-auto-scroll";
import "@splidejs/splide/dist/css/splide.min.css";

const AutoScrollingCarousel = () => {
  useEffect(() => {
    const splide = new Splide(".splide", {
      type: "loop",
      drag: false,
      focus: "center",
      perPage: 5, // Number of items per page
      autoScroll: {
        speed: 1 / 10, // Speed of auto-scrolling (higher is faster)
        pauseOnHover: false,
      },
      pagination: false,
      arrows: false,
      gap: 4,
    });

    splide.mount({ AutoScroll }); // Mount the splide instance with AutoScroll extension
  }, []);

  return (
    <div className="splide flex justify-center items-center h-full w-full">
      <div className="splide__track">
        <ul className="splide__list">
          {/* First Image */}
          <li className="splide__slide">
            <CardImage
              src="https://wiki.hypixel.net/images/1/18/SkyBlock_items_divan_pendant.png"
              alt="Divan Pendant"
            />
          </li>

          {/* Second Image */}
          <li className="splide__slide">
            <CardImage
              src="https://wiki.hypixel.net/images/7/72/SkyBlock_items_jasper_crystal.png"
              alt="Jasper Crystal"
            />
          </li>

          {/* Third Image */}
          <li className="splide__slide">
            <div className="w-20 h-20 bg-[rgba(255,255,255,0.06)] border border-[rgba(75,75,75,.7)] rounded-xl flex items-center justify-center p-4">
              <img
                className="scale-150"
                src="/Vanguard_Helmet.png"
                alt="Divan Pendant"
              />
            </div>
          </li>

          {/* Fourth Image */}
          <li className="splide__slide">
            <CardImage
              src="https://wiki.hypixel.net/images/7/75/SkyBlock_pets_golden_dragon.png"
              alt="Jasper Crystal"
            />
          </li>
        </ul>
      </div>
    </div>
  );
};

interface CardProps {
  width?: string; // Allows for width override (default "w-96")
  aspectRatio?: string; // Allows for aspect ratio override (default "aspect-square")
  title: string;
  description: string;
  children: ReactNode; // This will be for dynamic content like images, code, etc.
  textPosition: string;
}

const Card: React.FC<CardProps> = ({
  width = "w-96",
  aspectRatio = "aspect-square",
  title,
  description,
  children,
  textPosition = "center",
}) => {
  return (
    <div
      className={`${width} h-96 bg-[#141414] border border-[rgba(38,38,38,.7)] rounded-xl p-8 flex flex-col`}
    >
      {/* Center the children content vertically */}
      <div className="flex-grow flex items-center justify-center text-gray-500">
        {children}
      </div>

      {/* Title and description */}
      <div className={`text-${textPosition}`}>
        <h4 className="text-white font-medium text-xl mb-2">{title}</h4>
        <p className="text-gray-400">{description}</p>
      </div>
    </div>
  );
};

interface CardImageProps {
  height?: string; // Allow override of height (default "h-20")
  src: string; // Image source (required)
  alt: string; // Alt text for the image (required)
}

const CardImage: React.FC<CardImageProps> = ({
  height = "h-20", // Default height is "h-20"
  src,
  alt,
}) => {
  return (
    <div
      className={`aspect-square ${height} bg-[rgba(255,255,255,0.06)] border border-[rgba(75,75,75,.7)] rounded-xl p-4`}
    >
      <img src={src} alt={alt} />
    </div>
  );
};

const Section1: FC = () => {
  return (
    <div className="mt-28">
      {/* Heading Section */}
      <div className="w-full flex flex-col justify-center">
        <h1 className="text-center m-auto w-fit text-6xl mb-4 font-black text-white bg-clip-text text-transparent">
          {/* A new way to trade structures */}
          Skyblock Exchange
        </h1>
        <p className="text-gray-300 text-center m-auto text-xl">
          Trade skyblock structures using a real-time order book
        </p>
      </div>

      {/* Main Content Section */}
      <div
        className="flex flex-row gap-8 p-8 justify-center"
        style={{
          maxWidth: "75em", // Restrict the max-width to 75em
          width: "100%", // Allow it to take the full available width up to the max-width
          marginLeft: "auto", // Center the div horizontally
          marginRight: "auto", // Center the div horizontally
        }}
      >
        <Card
          title="Trade structures"
          description="Trade Fairy & Jasper Mineshafts, Dragon's Lairs, and more."
          width="w-7/12" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="left"
        >
          <AutoScrollingCarousel />
        </Card>

        <Card
          title="Data at your fingertips"
          description="Access historical trade data instantly."
          width="w-96" // Can adjust width as needed
          aspectRatio="aspect-square" // Aspect ratio control
          textPosition="center"
        >
          <p></p>
        </Card>
      </div>

      {/* Second Main Content Section */}
      <div
        className="flex flex-row gap-8 pl-8 pr-8 pb-8 justify-center"
        style={{
          maxWidth: "75em", // Restrict the max-width to 75em
          width: "100%", // Allow it to take the full available width up to the max-width
          marginLeft: "auto", // Center the div horizontally
          marginRight: "auto", // Center the div horizontally
        }}
      >
        <Card
          title="Party invites"
          description="Party invites are automatically generated when a trade executes."
          width="w-96" // Can adjust width as needed
          aspectRatio="aspect-square" // Aspect ratio control
          textPosition="left"
        >
          <p className="shadow-green-500 shadow-2xl p-4 border-green-500 border-2 rounded-xl font-regular">
            <span className="text-green-600">/party</span>
            &nbsp;
            <span className="text-green-600">invite</span>
            &nbsp;
            <span className="text-green-600">Hypixel</span>
          </p>
        </Card>

        <Card
          title="Lightning fast"
          description="Our real-time matching engine powers instant trade executions."
          width="w-7/12" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="center"
        >
          <div className="flex flex-row gap-4 p-8 justify-center">
            <CardImage src="/Redis_Mark_Red_RGB.svg" alt="Redis logo" />
            <CardImage src="/rust-logo-blk.svg" alt="Rust logo" />
          </div>
        </Card>
      </div>
    </div>
  );
};

const Section2: FC = () => {
  return (
    <div className="mt-28">
      {/* Heading Section */}
      <div className="w-full flex justify-center">
        <h1 className="text-center m-auto w-fit font-semibold mb-4 text-[44px] bg-gradient-to-r from-orange-200 via-orange-300 to-70% to-white bg-clip-text text-transparent">
          Built for developers
        </h1>
      </div>

      {/* Main Content Section */}
      <div
        className="flex flex-row gap-8 p-8 justify-center"
        style={{
          maxWidth: "75em", // Restrict the max-width to 75em
          width: "100%", // Allow it to take the full available width up to the max-width
          marginLeft: "auto", // Center the div horizontally
          marginRight: "auto", // Center the div horizontally
        }}
      >
        <Card
          title="Open source"
          description="All code is MIT licensed, free for you to use as you please!"
          width="w-96" // Can adjust width as needed
          aspectRatio="aspect-square" // Aspect ratio control
          textPosition="center"
        >
          <a
            className="rounded-xl hover:scale-95"
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
          width="w-7/12" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="left"
        >
          <p></p>
        </Card>
      </div>

      {/* Second Main Content Section */}
      <div
        className="flex flex-row gap-8 pl-8 pr-8 pb-8 justify-center"
        style={{
          maxWidth: "75em", // Restrict the max-width to 75em
          width: "100%", // Allow it to take the full available width up to the max-width
          marginLeft: "auto", // Center the div horizontally
          marginRight: "auto", // Center the div horizontally
        }}
      >
        <Card
          title="Lightning fast"
          description="Rust + Redis powered matching engine powers instant trade executions."
          width="w-7/12" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="center"
        >
          <div className="flex flex-row gap-4 p-8 justify-center">
            <CardImage src="/Redis_Mark_Red_RGB.svg" alt="Redis logo" />
            <CardImage src="/rust-logo-blk.svg" alt="Rust logo" />
          </div>
        </Card>

        <Card
          title="A foundation to build upon"
          description="Create your own client mods with our REST API & WebSocket feeds."
          width="w-96" // Can adjust width as needed
          aspectRatio="aspect-square" // Aspect ratio control
          textPosition="left"
        >
          <p
            className="text-gray-500 text-center"
            style={{
              background:
                "linear-gradient(to bottom, rgba(107, 114, 128, 1), rgba(107, 114, 128, 0))",
              WebkitBackgroundClip: "text",
              color: "transparent",
            }}
            aria-hidden="true"
          >
            <code>
              &#123; time:'2025-03-21', value:91.72 &#125;, &#123;
              time:'2025-03-21', value:18.23 &#125;, &#123; time:'2025-03-21',
              value:63.20 &#125;, &#123; time:'2025-03-21', value:70.56 &#125;,
              &#123; time:'2025-03-21', value:95.29 &#125;, &#123;
              time:'2025-03-21', value:35.85 &#125;, &#123; time:'2025-03-21',
              value:31.65 &#125;, &#123; time:'2025-03-21', value:24.91 &#125;,
              &#123;
            </code>
          </p>
        </Card>
      </div>
    </div>
  );
};

export default function Home() {
  return (
    <div className="">
      <nav
        className="flex justify-between items-center p-4 bg-[rgba(20, 20, 20, 0.5)] border-b border-[rgba(38,38,38,.7)] backdrop-blur-lg sticky top-0 w-full"
        style={{
          maxWidth: "80em", // Restrict the max-width to 75em
          width: "100%", // Allow it to take the full available width up to the max-width
          marginLeft: "auto", // Center the div horizontally
          marginRight: "auto", // Center the div horizontally
        }}
      >
        <div className="text-2xl font-medium text-white">SBX</div>
        <div className="flex space-x-6">
          <a
            href="/docs"
            className="px-4 py-2 text-white hover:bg-white hover:bg-opacity-10 rounded-lg transition duration-300"
          >
            Docs
          </a>

          <button
            className="h-10 text-center flex flex-row justify-center items-center border border-[rgba(82,82,82,0.7)] text-[#5865F2] bg-[#2b2b2b] dark:text-white px-6 py-2 rounded-full transition-all duration-300 ease-in-out hover:filter hover:drop-shadow-[0_0_10px_rgba(88,101,242,0.7)]"
            aria-label="Sign in with Discord"
          >
            Sign in with Discord
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              className="ml-2"
            >
              <g>
                <path
                  fill="currentColor"
                  d="M19.27 5.33C17.94 4.71 16.5 4.26 15 4a.1.1 0 0 0-.07.03c-.18.33-.39.76-.53 1.09a16.1 16.1 0 0 0-4.8 0c-.14-.34-.35-.76-.54-1.09c-.01-.02-.04-.03-.07-.03c-1.5.26-2.93.71-4.27 1.33c-.01 0-.02.01-.03.02c-2.72 4.07-3.47 8.03-3.1 11.95c0 .02.01.04.03.05c1.8 1.32 3.53 2.12 5.24 2.65c.03.01.06 0 .07-.02c.4-.55.76-1.13 1.07-1.74c.02-.04 0-.08-.04-.09c-.57-.22-1.11-.48-1.64-.78c-.04-.02-.04-.08-.01-.11c.11-.08.22-.17.33-.25c.02-.02.05-.02.07-.01c3.44 1.57 7.15 1.57 10.55 0c.02-.01.05-.01.07.01c.11.09.22.17.33.26c.04.03.04.09-.01.11c-.52.31-1.07.56-1.64.78c-.04.01-.05.06-.04.09c.32.61.68 1.19 1.07 1.74c.03.01.06.02.09.01c1.72-.53 3.45-1.33 5.25-2.65c.02-.01.03-.03.03-.05c.44-4.53-.73-8.46-3.1-11.95c-.01-.01-.02-.02-.04-.02M8.52 14.91c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.84 2.12-1.89 2.12m6.97 0c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.83 2.12-1.89 2.12"
                ></path>
              </g>
            </svg>
          </button>
        </div>
      </nav>

      <main>
        <Section1 />
        <Section2 />
      </main>

      <footer className="border-t border-gray-800 py-6 md:py-8 text-center">
        <p className="text-sm text-gray-500">Released under the MIT License.</p>
        <p className="text-sm text-gray-500">
          Copyright © {new Date().getFullYear()} SBX. All rights reserved.
        </p>
      </footer>
    </div>
  );
}
