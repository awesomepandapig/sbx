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
            timestamp: "2025-04-16T08:50:26.000Z",
            sequence_num: 0,
            events: [
              {
                type: "snapshot",
                tickers: [
                  {
                    type: "ticker",
                    product_id: "JSP",
                    price: "21932.98",
                    volume_24_h: "16038.28770938",
                    low_24_h: "21835.29",
                    high_24_h: "23011.18",
                    low_52_w: "15460",
                    high_52_w: "48240",
                    price_percent_chg_24_h: "-4.15775596190603",
                    best_bid: "21931.98",
                    best_bid_quantity: "8000.21",
                    best_ask: "21933.98",
                    best_ask_quantity: "8038.07770938",
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
