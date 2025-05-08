import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";
import CodeHighlight from "~/components/docs/CodeHighlight";

export default function UserChannelDescription() {
  return (
    <div id="user" className="mb-12 scroll-mt-20">
      <h2 className="text-2xl font-bold mb-6">User Batch Channel</h2>
      <p className="mb-6">
        The <CodeHighlight code="user" /> channel sends real-time updates
        related to the authenticated user's orders.
      </p>
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Request\n` +
          escapeJSONString({
            type: "subscribe",
            channel: "user",
            jwt: "XYZ",
          })
        }
      />
      <p className="mb-6">
        Each matched order will contain a{" "}
        <CodeHighlight code="counterparty_minecraft_id" />. This is the
        minecraft UUID of the player who you traded with. You can use the{" "}
        <a
          href="https://minecraft.wiki/w/Mojang_API#Query_player's_username"
          className="text-blue-500"
          target="_blank"
        >
          Mojang API
        </a>{" "}
        to resolve this player's username so that you can party them in-game.
      </p>
      <CodeBlock
        className="mb-8"
        language="json"
        code={
          `// Response\n` +
          escapeJSONString({
            channel: "user",
            client_id: "",
            timestamp: "2025-04-16T08:50:26.000Z",
            sequence_num: 0,
            events: [
              {
                type: "update",
                product_id: "JSP",
                updates: [
                  {
                    id: "c9884d44-d3da-4e94-92fc-295a4c3c1df2",
                    product_id: "JSP",
                    user_id: "",
                    counterparty_minecraft_id: "",
                    side: "sell",
                    type: "limit",
                    created_at: "2025-04-16T08:50:26.000Z",
                    executed_value: 90,
                    status: "done",
                    settled: true,
                    price: 90,
                    cancel_after: "min",
                    size: 1,
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
