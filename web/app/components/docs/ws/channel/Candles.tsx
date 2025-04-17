import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";
import CodeHighlight from "~/components/docs/CodeHighlight";

export default function CandlesChannelDescription() {
  return (
    <div id="candles" className="mb-12 scroll-mt-20">
      <h2 className="text-2xl font-bold mb-6">Candles Channel</h2>
      <p className="mb-6">
        The <CodeHighlight code="candles"/> channel streams OHLCV (Open, High, Low, Close, Volume) data
        once per second. Each update reflects price action grouped into 5-minute
        intervals.
      </p>
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Request\n` +
          escapeJSONString({
            type: "subscribe",
            channel: "candles",
            product_ids: ["JSP"],
          })
        }
      />
      <CodeBlock
        language="json"
        code={
          `// Response\n` +
          escapeJSONString({
            channel: "candles",
            client_id: "",
            timestamp: "2025-04-16T08:50:26.000Z",
            sequence_num: 0,
            events: [
              {
                type: "snapshot",
                candles: [
                  {
                    start: "1688998200",
                    high: "1867.72",
                    low: "1865.63",
                    open: "1867.38",
                    close: "1866.81",
                    volume: "0.20269406",
                    product_id: "JSP",
                  },
                ],
              },
            ],
          })
        }
      />
    </div>
  );
}
