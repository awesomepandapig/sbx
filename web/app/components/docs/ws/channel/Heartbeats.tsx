import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";
import CodeHighlight from "~/components/docs/CodeHighlight";

export default function HeartbeatsChannelDescription() {
  return (
    <div id="heartbeats" className="mb-12 scroll-mt-20">
      <h2 className="text-2xl font-bold mb-6">Heartbeats Channel</h2>
      <p className="mb-6">
        The <CodeHighlight code="heartbeats"/> channel emits a message every second to maintain an
        active connection. Each message includes a <CodeHighlight code="heartbeat_counter"/> to help
        detect missed messages.
      </p>
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Request\n` +
          escapeJSONString({
            type: "subscribe",
            channel: "heartbeats",
          })
        }
      />
      <CodeBlock
        language="json"
        code={
          `// Response\n` +
          escapeJSONString({
            channel: "heartbeats",
            client_id: "",
            timestamp: "2025-04-16T08:50:26.000Z",
            sequence_num: 0,
            events: [
              {
                current_time:
                  "2023-06-23 20:31:56.121961769 +0000 UTC m=+91717.525857105",
                heartbeat_counter: 1,
              },
            ],
          })
        }
      />
    </div>
  );
}
