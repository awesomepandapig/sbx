import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";
import CodeHighlight from "~/components/docs/CodeHighlight";

export default function Level2ChannelDescription() {
  return (
    <div id="level2" className="mb-12 scroll-mt-20">
      <h2 className="text-2xl font-bold mb-6">Level2 Batch Channel</h2>
      <p className="mb-6">
        The <CodeHighlight code="level2" /> channel streams a real-time,
        depth-aggregated view of the order book, showing current bids and asks
        at each price level.
      </p>
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Request\n` +
          escapeJSONString({
            type: "subscribe",
            channel: "level2",
            product_ids: ["JSP"],
          })
        }
      />
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Response\n` +
          escapeJSONString({
            channel: "l2_data",
            client_id: "",
            timestamp: "2025-04-16T08:50:26.000Z",
            sequence_num: 0,
            events: [
              {
                type: "snapshot",
                product_id: "JSP",
                updates: [
                  {
                    side: "buy",
                    event_time: "1970-01-01T00:00:00Z",
                    price_level: "21921.73",
                    new_quantity: "0.06317902",
                  },
                  {
                    side: "buy",
                    event_time: "1970-01-01T00:00:00Z",
                    price_level: "21921.3",
                    new_quantity: "0.02",
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
