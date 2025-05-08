import CodeHighlight from "~/components/docs/CodeHighlight";
import CreateOrder from "~/components/docs/api/endpoint/CreateOrder";

export default function Page() {
  return (
    <>
      <main>
        <h1 className="text-4xl font-bold mb-8">API Endpoints</h1>
        <p className="mb-8">
          Endpoint URL:{" "}
          <CodeHighlight code="https://api.skyblock.exchange/api/v1/trade/&#123;resource&#125;" />
        </p>

        <p className="mb-8">Our API provides the following endpoints:</p>

        <div className="border border-gray-800 rounded-lg overflow-hidden mb-8">
          <table className="w-full">
            <thead className="bg-gray-900">
              <tr>
                <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
                  API
                </th>
                <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
                  Method
                </th>
                <th className="px-6 py-4 text-left text-sm font-medium text-gray-400">
                  Resource
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-800">
              <tr>
                <td className="px-6 py-4">
                  <a href={`#create_order`} className="text-green-500">
                    Create Order
                  </a>
                </td>
                <td className="px-6 py-4">POST</td>
                <td className="px-6 py-4">
                  <CodeHighlight code="/orders" />
                </td>
              </tr>

              <tr>
                <td className="px-6 py-4">
                  <a href="#get_order" className="text-blue-500">
                    Get Order
                  </a>
                </td>
                <td className="px-6 py-4">GET</td>
                <td className="px-6 py-4">
                  <CodeHighlight code="/orders/:id" />
                </td>
              </tr>

              <tr>
                <td className="px-6 py-4">
                  <a href={`#get_orders`} className="text-blue-500">
                    Get All Orders
                  </a>
                </td>
                <td className="px-6 py-4">GET</td>
                <td className="px-6 py-4">
                  <CodeHighlight code="/orders" />
                </td>
              </tr>

              <tr>
                <td className="px-6 py-4">
                  <a href="#cancel_order" className="text-red-500">
                    Cancel Order
                  </a>
                </td>
                <td className="px-6 py-4">DELETE</td>
                <td className="px-6 py-4">
                  <CodeHighlight code="/orders/:order_id" />
                </td>
              </tr>

              <tr>
                <td className="px-6 py-4">
                  <a href={`#cancel_orders`} className="text-red-500">
                    Cancel All Orders
                  </a>
                </td>
                <td className="px-6 py-4">DELETE</td>
                <td className="px-6 py-4">
                  <CodeHighlight code="/orders" />
                </td>
              </tr>

            </tbody>
          </table>
        </div>

        <CreateOrder/>
        

      </main>
    </>
  );
}