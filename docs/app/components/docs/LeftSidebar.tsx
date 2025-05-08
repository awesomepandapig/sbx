import { ArrowUpRight } from "lucide-react";

export default function LeftSidebar() {
  return (
    <div className="text-white w-64 border-r border-gray-800 h-screen fixed">
      <div className="p-6 border-b border-gray-800 mt-8">
        <div className="text-sm font-semibold text-gray-500 mb-2">
          INTRODUCTION
        </div>
        <ul className="space-y-2">
          <li>
            <a href="/docs/welcome" className="">
              Welcome
            </a>
          </li>
          <li>
            <a href="/docs/getting-started" className="">
              Getting Started
            </a>
          </li>
        </ul>
      </div>
      <div className="p-6 border-b border-gray-800">
        <div className="text-sm font-semibold text-gray-500 mb-2">REST API</div>
        <ul className="space-y-2">
          <li>
            <a href="/docs/api-endpoints" className="">
              Endpoints
            </a>
          </li>
          <li>
            <a href="/docs/api-authentication" className="">
              Authentication
            </a>
          </li>
          <li>
            <a href="#" className="">
              Rate Limits
            </a>
          </li>
        </ul>
      </div>
      <div className="p-6 border-b border-gray-800">
        <div className="text-sm font-semibold text-gray-500 mb-2">
          WEBSOCKET FEED
        </div>
        <ul className="space-y-2">
          <li>
            <a href="/docs/ws-channels" className="">
              Channels
            </a>
          </li>
          <li>
            <a href="/docs/ws-authentication" className="">
              Authentication
            </a>
          </li>
          <li>
            <a href="#" className="">
              Rate Limits
            </a>
          </li>
        </ul>
      </div>
      <div className="p-6">
        <div className="text-sm font-semibold text-gray-500 mb-2">
          RELEASE NOTES
        </div>
        <ul className="space-y-2">
          <li>
            <a href="#" className="">
              Changelog
            </a>
          </li>
          <li>
            <a
              href="https://github.com/awesomepandapig/sbx"
              target="_blank"
              className="flex items-center justify-between w-fit"
            >
              <span className="w-32">GitHub</span>
              <ArrowUpRight className="w-32" />
            </a>
          </li>
        </ul>
      </div>
    </div>
  );
}
