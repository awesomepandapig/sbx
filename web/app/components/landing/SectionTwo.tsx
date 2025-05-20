import Card, { CardImage } from "./Card";

export default function SectionTwo() {
  return (
    <div className="md:mt-12">
      {/* Heading Section */}
      <div className="w-full flex justify-center">
        <h1 className="text-center m-auto w-full font-semibold md:p-8 p-4 md:text-[44px] text-3xl bg-gradient-to-r from-orange-200 via-orange-300 to-70% to-white bg-clip-text text-transparent">
          Built for developers
        </h1>
      </div>

      {/* Main Content Section */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">

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
          width="md:col-span-2" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="left"
        >
          <p></p>
        </Card>
      
        <Card
          title="Lightning fast"
          description="Aeron® powered matching engine enables instant trade executions."
          width="w-full" // Can adjust width as needed
          aspectRatio="" // Aspect ratio control
          textPosition="center"
        >
          <p>Replace with animation of orders incoming</p>
        </Card>

        <Card
          title="A foundation to build upon"
          description="Create your own client mods with our FIX, REST, & WebSocket APIs."
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
              &123;
              "product_id": "JSP",
              "side": "buy",
              "type": "limit",
              "size": 1,
              "price": 90
              &125;
            </code>
          </p>
        </Card>
      </div>
    </div>
  );
}
