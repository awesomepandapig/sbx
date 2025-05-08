import CodeHighlight from "~/components/docs/CodeHighlight";
import CodeBlock, { escapeJSONString } from "~/components/docs/CodeBlock";

type OrderSide = "buy" | "sell";
type OrderType = "limit" | "market";

interface CreateOrderRequest {
  product_id: string;
  side: OrderSide;
  type: OrderType;
  price?: number; // required if type is "limit"
  size: number;
}

interface OrderResponse {
  id: string;
  product_id: string;
  side: OrderSide;
  type: OrderType;
  price?: number;
  size: number;
  status: "pending" | "open" | "filled" | "partially_filled" | "canceled";
  created_at: string;
  updated_at: string;
}

interface HeartbeatEvent {
  current_time: string;
  heartbeat_counter: number;
}

interface HeartbeatResponse {
  channel: "heartbeats";
  client_id: string;
  timestamp: string;
  sequence_num: number;
  events: HeartbeatEvent[];
}

export default function CreateOrder() {
  return (
    


        // <div className="mb-12 scroll-mt-20">
        //   <h3 className="text-xl font-bold mb-4">Order Request Schema</h3>
        //   <div className="border border-gray-800 rounded-lg overflow-hidden mb-8">
        //     <table className="w-full">
        //       <thead className="bg-gray-900">
        //         <tr>
        //           <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
        //             Field
        //           </th>
        //           <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
        //             Type
        //           </th>
        //           <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
        //             Description
        //           </th>
        //           <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
        //             Required
        //           </th>
        //         </tr>
        //       </thead>
        //       <tbody className="divide-y divide-gray-800">
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="product_id" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code>
        //           </td>
        //           <td className="px-6 py-4">
        //             The unique identifier of the product being traded (e.g.,
        //             "BTC-USD").
        //           </td>
        //           <td className="px-6 py-4">Yes</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="side" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code> (enum: "buy", "sell")
        //           </td>
        //           <td className="px-6 py-4">
        //             Indicates whether the order is a buy or sell order.
        //           </td>
        //           <td className="px-6 py-4">Yes</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="type" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code> (enum: "limit", "market")
        //           </td>
        //           <td className="px-6 py-4">
        //             Specifies the order type. "limit" orders are executed at a
        //             specified price or better, while "market" orders are executed
        //             at the best available market price.
        //           </td>
        //           <td className="px-6 py-4">Yes</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="price" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>number</code>
        //           </td>
        //           <td className="px-6 py-4">
        //             The limit price for a "limit" order. This field must be
        //             present if <CodeHighlight code="type" /> is "limit" and
        //             should be omitted for "market" orders.
        //           </td>
        //           <td className="px-6 py-4">Conditional (required if type is "limit")</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="size" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>number</code>
        //           </td>
        //           <td className="px-6 py-4">
        //             The quantity of the product to buy or sell.
        //           </td>
        //           <td className="px-6 py-4">Yes</td>
        //         </tr>
        //       </tbody>
        //     </table>
        //   </div>

        //   <h3 className="text-xl font-bold mb-4">Order Schema</h3>
        //   <div className="border border-gray-800 rounded-lg overflow-hidden mb-8">
        //     <table className="w-full">
        //       <thead className="bg-gray-900">
        //         <tr>
        //           <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
        //             Field
        //           </th>
        //           <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
        //             Type
        //           </th>
        //           <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
        //             Description
        //           </th>
        //         </tr>
        //       </thead>
        //       <tbody className="divide-y divide-gray-800">
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="id" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code>
        //           </td>
        //           <td className="px-6 py-4">
        //             The unique identifier of the newly created order.
        //           </td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="product_id" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code>
        //           </td>
        //           <td className="px-6 py-4">
        //             The identifier of the product for this order.
        //           </td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="side" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code> (enum: "buy", "sell")
        //           </td>
        //           <td className="px-6 py-4">The side of the order.</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="type" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code> (enum: "limit", "market")
        //           </td>
        //           <td className="px-6 py-4">The type of the order.</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="price" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>number</code>
        //           </td>
        //           <td className="px-6 py-4">
        //             The limit price, if applicable.
        //           </td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="size" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>number</code>
        //           </td>
        //           <td className="px-6 py-4">The size of the order.</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="status" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code> (enum: "pending", "open", "filled",
        //             "partially_filled", "canceled")
        //           </td>
        //           <td className="px-6 py-4">The current status of the order.</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="created_at" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code> (ISO 8601 timestamp)
        //           </td>
        //           <td className="px-6 py-4">The timestamp when the order was created.</td>
        //         </tr>
        //         <tr>
        //           <td className="px-6 py-4">
        //             <CodeHighlight code="updated_at" />
        //           </td>
        //           <td className="px-6 py-4">
        //             <code>string</code> (ISO 8601 timestamp)
        //           </td>
        //           <td className="px-6 py-4">
        //             The timestamp when the order was last updated.
        //           </td>
        //         </tr>
        //       </tbody>
        //     </table>
        //   </div>



        // </div>

        <div>

          <CodeBlock
            language="json"
            code={`// Response\n${escapeJSONString({
              "id": "6593cbaa-63f9-49a9-a85b-b7b05a362c07",
              "product_id": "JSP",
              "user_id": "EFy0yLnePUXmb4q0ZVz2uxQxeBnkzCRv",
              "side": "buy",
              "type": "limit",
              "created_at": "2025-05-08T20:10:26.000Z",
              "executed_value": 0,
              "status": "open",
              "settled": false,
              "price": 90,
              "cancel_after": "hour",
              "size": 1
            })}`}
          
          />

        </div>
  );
}