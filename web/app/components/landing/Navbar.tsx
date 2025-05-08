import { DOCS_URL } from "~/lib/config";
import DiscordButton from "~/components/landing/DiscordButton";

export default function Navbar() {
  return (
    <nav className="bg-[rgba(20, 20, 20, 0.5)] border-b border-[rgba(38,38,38,.7)] backdrop-blur-lg fixed top-0 w-full z-10">
      <div className="flex justify-between items-center max-w-[90em] mx-auto w-full h-16">
        <div className="text-lg font-medium text-[#dfdfd6]">SBX</div>
        <div className="flex space-x-6 items-center">
          <a
            href={`${DOCS_URL}`}
            className="text-[#dfdfd6] hover:text-blue-400 text-sm transition duration-300"
          >
            Docs
          </a>

          <a
            href="/blog"
            className="text-[#dfdfd6] hover:text-blue-400 text-sm transition duration-300"
          >
            Blog
          </a>

          <DiscordButton />
        </div>
      </div>
    </nav>
  );
}
