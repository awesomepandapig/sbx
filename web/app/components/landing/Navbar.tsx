import { DOCS_URL } from "~/lib/config";
import DiscordButton from "~/components/landing/DiscordButton";

export default function Navbar() {
  return (
    <header className="fixed top-0 left-0 right-0 z-10 bg-[rgba(20,20,20,0.5)] backdrop-blur-lg">
      <div className="max-w-7xl mx-auto px-4 w-full flex justify-between items-center h-16">
        <div className="flex items-center gap-4">
          <div className="bg-[#333333] rounded-full p-2">
            <div className="w-8 h-8 bg-white rounded-full flex items-center justify-center">
              <svg
                viewBox="0 0 24 24"
                width="20"
                height="20"
                className="text-black"
              >
                <path
                  fill="currentColor"
                  d="M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M12,4A8,8 0 0,1 20,12A8,8 0 0,1 12,20A8,8 0 0,1 4,12A8,8 0 0,1 12,4M12,6.5A5.5,5.5 0 0,0 6.5,12A5.5,5.5 0 0,0 12,17.5A5.5,5.5 0 0,0 17.5,12A5.5,5.5 0 0,0 12,6.5M12,9A3,3 0 0,1 15,12A3,3 0 0,1 12,15A3,3 0 0,1 9,12A3,3 0 0,1 12,9Z"
                />
              </svg>
            </div>
          </div>
          <nav className="hidden sm:flex items-center gap-2">
            <button className="bg-[#e9e9e9] text-black px-6 py-2 rounded-full text-sm font-medium">
              HOME
            </button>
            <a className="bg-[#333333] text-white px-6 py-2 rounded-full text-sm font-medium">
              TRADE
            </a>
            <a
              href={`${DOCS_URL}`}
              className="bg-[#333333] text-white px-6 py-2 rounded-full text-sm font-medium"
            >
              DOCS
            </a>
            <button className="bg-[#333333] text-white px-6 py-2 rounded-full text-sm font-medium">
              FEATURES
            </button>
          </nav>
        </div>
        <button className="bg-[#3CFFFF] text-black px-6 py-2 rounded-full text-sm font-medium">
          GET STARTED
        </button>
      </div>
    </header>
  );
}
