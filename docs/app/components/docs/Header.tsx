export default function Header() {
  return (
    <header className="text-white border-b border-gray-800 fixed top-0 left-0 right-0 z-10 bg-black">
      <div className="container mx-6">
        <div className="flex space-x-8">
          <a
            href="/docs"
            className="py-4 text-sm font-medium border-b-2 border-blue-500 text-blue-500"
          >
            Home
          </a>
          <a href="/docs/getting-started" className="py-4 text-sm font-medium">
            Get Started
          </a>
          <a href="/docs/api-endpoints" className="py-4 text-sm font-medium">
            API Reference
          </a>
          <a href="/docs/ws-channels" className="py-4 text-sm font-medium">
            WebSocket
          </a>
        </div>
      </div>
    </header>
  );
}
