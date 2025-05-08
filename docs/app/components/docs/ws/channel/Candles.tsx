import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";
import CodeHighlight from "~/components/docs/CodeHighlight";

export default function CandlesChannelDescription() {
  return (
    <div id="candles" className="mb-12 scroll-mt-20">
      <h2 className="text-2xl font-bold mb-6">Candles Channel</h2>
      <p className="mb-6">
        The <CodeHighlight code="candles" /> channel streams OHLCV (Open, High,
        Low, Close, Volume) data once per second. Each update reflects price
        action grouped into 5-minute intervals.
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
            timestamp: "2025-04-19T22:07:31.937Z",
            sequence_num: 0,
            events: [
              {
                type: "update",
                candles: [
                  {
                    start: "1745100300",
                    open: "545",
                    high: "549",
                    low: "511",
                    close: "544",
                    volume: "1234",
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
