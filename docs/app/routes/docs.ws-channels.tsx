import { useState, useEffect, useRef } from "react";
import { CircleAlert } from "lucide-react";

import HeartbeatsChannelDescription from "~/components/docs/ws/channel/Heartbeats";
import CandlesChannelDescription from "~/components/docs/ws/channel/Candles";
import TickerChannelDescription from "~/components/docs/ws/channel/Ticker";
import TickerBatchChannelDescription from "~/components/docs/ws/channel/TickerBatch";
import Level2ChannelDescription from "~/components/docs/ws/channel/Level2";
import UserChannelDescription from "~/components/docs/ws/channel/User";

import RightSidebar from "~/components/docs/RightSidebar";
import ChannelsTable from "~/components/docs/ws/ChannelTable";

interface Channel {
  id: string;
  title: string;
}

const channelData: Channel[] = [
  { id: "heartbeats", title: "Heartbeats Channel" },
  { id: "candles", title: "Candles Channel" },
  { id: "ticker", title: "Ticker Channel" },
  { id: "ticker_batch", title: "Ticker Batch Channel" },
  { id: "level2", title: "Level2 Batch Channel" },
  { id: "user", title: "User Batch Channel" },
];

export default function WebSocketChannelsPage() {
  const [activeSection, setActiveSection] = useState(channelData[0].id);
  const channelRefs = useRef<Record<string, HTMLElement | null>>({});

  useEffect(() => {
    // Initialize refs for all sections
    channelData.forEach((channel) => {
      channelRefs.current[channel.id] = document.getElementById(channel.id);
    });

    const handleScroll = () => {
      const scrollPosition = window.scrollY + 100; // Offset for header

      // Find the first section that's currently in view
      for (const id of channelData.map((channel) => channel.id)) {
        const element = channelRefs.current[id];
        if (!element) continue;

        const offsetTop = element.offsetTop;
        const offsetHeight = element.offsetHeight;

        if (
          scrollPosition >= offsetTop &&
          scrollPosition < offsetTop + offsetHeight
        ) {
          setActiveSection(id);
          break;
        }
      }
    };

    window.addEventListener("scroll", handleScroll);
    handleScroll(); // Initial run

    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <>
      <main>
        <h1 className="text-4xl font-bold mb-8">WebSocket Channels</h1>

        <div className="bg-amber-950 p-4 mb-8 rounded-xl">
          <div className="flex">
            <CircleAlert className="h-5 w-5 text-amber-500 mr-2 shrink-0" />
            <div>
              <p className="font-medium">
                Use heartbeats to keep subscriptions open
              </p>
              <p className="mt-1">
                Channels close after 60 seconds if no updates are sent.
                Subscribe to{" "}
                <a href="#heartbeats" className="text-blue-500">
                  heartbeats
                </a>{" "}
                to keep subscriptions open.
              </p>
            </div>
          </div>
        </div>

        <p className="mb-8">Our WebSocket provides the following channels:</p>

        <ChannelsTable />

        <p className="mb-8">
          Refer to the documentation on{" "}
          <a className="text-blue-500" href="/docs/ws-overview#subscribing">
            subscribing to a WebSocket channel
          </a>
          .
        </p>

        <HeartbeatsChannelDescription />
        <CandlesChannelDescription />
        <TickerChannelDescription />
        <TickerBatchChannelDescription />
        <Level2ChannelDescription />
        <UserChannelDescription />
      </main>
      <RightSidebar sections={channelData} activeSection={activeSection} />
    </>
  );
}
