import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";
import CodeHighlight from "~/components/docs/CodeHighlight";

export default function TickerChannelDescription() {
  return (
    <div id="ticker" className="mb-12 scroll-mt-20">
      <h2 className="text-2xl font-bold mb-6">Ticker Channel</h2>
      <p className="mb-6">
        The <CodeHighlight code="ticker" /> channel delivers live price updates
        for each trade match, reflecting the most recent market activity in real
        time.
      </p>
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Request\n` +
          escapeJSONString({
            type: "subscribe",
            channel: "ticker",
            product_ids: ["JSP"],
          })
        }
      />
      <CodeBlock
        language="json"
        code={
          `// Response\n` +
          escapeJSONString({
            channel: "ticker",
            client_id: "",
            timestamp: "2025-04-19T22:28:59.504Z",
            sequence_num: 0,
            events: [
              {
                type: "snapshot",
                tickers: [
                  {
                    product_id: "JSP",
                    price: "516",
                    volume_24_h: "1123",
                    low_24_h: "510",
                    high_24_h: "549",
                    low_52_w: "510",
                    high_52_w: "549",
                    price_percent_chg_24_h: "-1.338432122370937",
                    best_bid: "514",
                    best_bid_quantity: "9",
                    best_ask: "515",
                    best_ask_quantity: "11",
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
