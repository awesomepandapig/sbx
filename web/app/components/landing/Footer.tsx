export default function Footer() {
  const year = new Date().getFullYear();

  return (
    <footer className="border-t border-neutral-800 bg-black text-neutral-400 py-16 text-sm">
      <div className="max-w-7xl mx-auto px-6">
        {/* Main links section */}
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-[2fr_1fr_1fr_1fr_1fr] gap-12">
          <div className="flex flex-col justify-between h-full">
            <div>
              <h2 className="text-white text-2xl font-semibold mb-4">SBX</h2>
              <p className="text-gray-500 text-sm">
                Not affiliated with or endorsed by Hypixel Inc.
              </p>
              <p className="text-gray-500 text-sm">
                &copy; {year} SBX. All rights reserved.
              </p>
            </div>

            <a
              className="flex items-center space-x-2 mt-4"
              href="https://status.skyblock.exchange"
            >
              <span className="w-3 h-3 rounded-full bg-green-500 animate-pulse" />
              <span className="text-green-400 font-medium text-sm">
                All systems operational
              </span>
            </a>
          </div>

          <div>
            <h3 className="text-white text-base font-semibold tracking-wider mb-4">
              Product
            </h3>
            <ul className="space-y-3">
              <li>
                <a
                  href="/trade/JSP"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  Trade
                </a>
              </li>
            </ul>
          </div>

          <div>
            <h3 className="text-white text-base font-semibold tracking-wider mb-4">
              Resources
            </h3>
            <ul className="space-y-3">
              <li>
                <a
                  href="/blog"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  Blog
                </a>
              </li>
              <li>
                <a
                  href="/changelog"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  Changelog
                </a>
              </li>
            </ul>
          </div>

          <div>
            <h3 className="text-white text-base font-semibold tracking-wider mb-4">
              Developers
            </h3>
            <ul className="space-y-3">
              <li>
                <a
                  href="/docs"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  Documentation
                </a>
              </li>
              <li>
                <a
                  href="/docs/api"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  API Reference
                </a>
              </li>
              <li>
                <a
                  href="/docs/use-cases"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  Use Cases
                </a>
              </li>
            </ul>
          </div>

          <div>
            <h3 className="text-white text-base font-semibold tracking-wider mb-4">
              Company
            </h3>
            <ul className="space-y-3">
              <li>
                <a
                  href="/about"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  About
                </a>
              </li>
              <li>
                <a
                  href="/terms"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
                >
                  Terms
                </a>
              </li>
              <li>
                <a
                  href="/privacy"
                  className="text-gray-400 hover:text-white transition-colors duration-200 ease-in-out"
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
