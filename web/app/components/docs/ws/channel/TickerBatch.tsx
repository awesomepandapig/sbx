import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";
import CodeHighlight from "~/components/docs/CodeHighlight";

export default function TickerBatchChannelDescription() {
  return (
    <div id="ticker_batch" className="mb-12 scroll-mt-20">
      <h2 className="text-2xl font-bold mb-6">Ticker Batch Channel</h2>
      <p className="mb-6">
        The <CodeHighlight code="ticker_batch"/> channel provides batched price updates every 5 seconds.
        The response schema is identical to the ticker channel.
      </p>
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Request\n` +
          escapeJSONString({
            type: "subscribe",
            channel: "ticker_batch",
            product_ids: ["JSP"],
          })
        }
      />
    </div>
  );
}
