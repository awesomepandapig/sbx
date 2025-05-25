export default function Footer() {
  const year = new Date().getFullYear();
  const DOCS_URL = "https://docs.skyblock.exchange";

  return (
    <footer className="m-auto max-w-7xl">
      <div className="bg-[#0e0e0e] md:border border-[rgba(38,38,38,.7)] md:rounded-3xl md:mx-4 md:my-4 lg:my-8 p-8 text-sm">
        <div className="grid grid-cols-1 md:grid-cols-6 lg:grid-cols-7 gap-12">
          {/* SBX Column */}
          <div className="lg:col-span-4 md:col-span-3">
            <h2 className="text-white text-3xl md:text-6xl font-medium mb-2 md:mb-4">
              SBX
            </h2>
            <p className="text-neutral-400 text-sm mb-1">
              Not affiliated with or endorsed by Hypixel Inc.
              <br />
              &copy; {year} SBX. All rights reserved.
            </p>

            <a
              className="flex items-center space-x-2 mt-2 md:mt-6"
              href="https://status.skyblock.exchange"
            >
              <span className="w-2.5 h-2.5 rounded-full bg-green-500 animate-pulse" />
              <span className="text-green-400 text-xs font-medium">
                All systems operational
              </span>
            </a>
          </div>

          {/* Product Column */}
          <div>
            <h3 className="text-white font-medium mb-2 text-lg">Product</h3>
            <ul className="space-y-2">
              <li>
                <a
                  href="/trade/JSP"
                  className="text-neutral-400 hover:text-cyan-400 transition-colors duration-200"
                >
                  Trade
                </a>
              </li>
            </ul>
          </div>

          {/* Developers Column */}
          <div>
            <h3 className="text-white font-medium mb-2 text-lg">Developers</h3>
            <ul className="space-y-2">
              <li>
                <a
                  href={DOCS_URL}
                  className="text-neutral-400 hover:text-cyan-400 transition-colors duration-200"
                >
                  Documentation
                </a>
              </li>
              <li>
                <a
                  href={`${DOCS_URL}/docs/api-endpoints`}
                  className="text-neutral-400 hover:text-cyan-400 transition-colors duration-200"
                >
                  API Reference
                </a>
              </li>
              <li>
                <a
                  href={`${DOCS_URL}/docs/use-cases`}
                  className="text-neutral-400 hover:text-cyan-400 transition-colors duration-200"
                >
                  Use Cases
                </a>
              </li>
            </ul>
          </div>

          {/* Company Column */}
          <div>
            <h3 className="text-white font-medium mb-2 text-lg">Company</h3>
            <ul className="space-y-2">
              <li>
                <a
                  href="/about"
                  className="text-neutral-400 hover:text-cyan-400 transition-colors duration-200"
                >
                  About
                </a>
              </li>
              <li>
                <a
                  href="/terms"
                  className="text-neutral-400 hover:text-cyan-400 transition-colors duration-200"
                >
                  Terms
                </a>
              </li>
              <li>
                <a
                  href="/privacy"
                  className="text-neutral-400 hover:text-cyan-400 transition-colors duration-200"
                >
                  Privacy
                </a>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </footer>
  );
}
