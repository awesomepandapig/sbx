import Card, { CardImage } from "./Card";

export default function SectionTwo() {
  const sectionStyle = {
    maxWidth: "75em",
    width: "100%",
    marginLeft: "auto",
    marginRight: "auto",
  };

  return (
    <div className="mt-24">
      {/* Heading Section */}
      <div className="w-full flex justify-center">
        <h1 className="text-center m-auto w-fit font-semibold mb-4 text-[44px] bg-gradient-to-r from-orange-200 via-orange-300 to-70% to-white bg-clip-text text-transparent">
          Built for developers
        </h1>
      </div>

      {/* Main Content Section */}
      <div
        className="flex flex-row gap-8 p-8 justify-center"
        style={sectionStyle}
      >
        <Card
          title="Open source"
          description="All code is MIT licensed, free for you to use as you please!"
          width="w-96" // Can adjust width as needed
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
          <p>Replace with animation of orders incoming</p>
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
            <code className="text-xs">
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
}
