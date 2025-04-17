const channelData = [
  {
    id: "heartbeats",
    title: "Heartbeats Channel",
    description:
      "The heartbeats channel emits a message every second to maintain an active connection. Each message includes a heartbeat_counter to help detect missed messages.",
    authRequired: false,
  },
  {
    id: "candles",
    title: "Candles Channel",
    description:
      "The candles channel streams OHLCV (Open, High, Low, Close, Volume) data once per second. Each update reflects price action grouped into 5-minute intervals.",
    authRequired: false,
  },
  {
    id: "ticker",
    title: "Ticker Channel",
    description:
      "The ticker channel delivers live price updates for each trade match, reflecting the most recent market activity in real time.",
    authRequired: false,
  },
  {
    id: "ticker_batch",
    title: "Ticker Batch Channel",
    description:
      "The ticker_batch channel provides batched price updates every 5 seconds. The response schema is identical to the ticker channel.",
    authRequired: false,
  },
  {
    id: "level2",
    title: "Level2 Channel",
    description:
      "The level2 channel streams a real-time, depth-aggregated view of the order book, showing current bids and asks at each price level.",
    authRequired: false,
  },
  {
    id: "user",
    title: "User Channel",
    description:
      "The user channel sends real-time updates related to the authenticated user's orders.",
    authRequired: true,
  },
];

export default function ChannelsTable() {
  return (
    <div className="border border-gray-800 rounded-lg overflow-hidden mb-8">
      <table className="w-full">
        <thead className="bg-gray-900">
          <tr>
            <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
              Channel
            </th>
            <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
              Description
            </th>
            <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
              Requires Authentication
            </th>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-800">
          {channelData.map((channel) => (
            <tr key={channel.id}>
              <td className="px-6 py-4">
                <a
                  href={`#${channel.id}`}
                  className={`text-${channel.id === "level2" ? "blue" : channel.id === "user" || channel.id === "ticker" ? "amber" : "red"}-500`}
                >
                  {channel.id}
                </a>
              </td>
              <td className="px-6 py-4">{channel.description}</td>
              <td className="px-6 py-4">
                {channel.authRequired ? "Yes" : "No"}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
