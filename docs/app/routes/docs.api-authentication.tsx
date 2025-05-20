import CodeHighlight from "~/components/docs/CodeHighlight";

export default function Page() {
  return (
    <>
      <main>
        <h1 className="text-4xl font-bold mb-8">API Authentication</h1>

        <p className="mb-4">
          To access protected endpoints, you'll need an API key. Follow the
          steps below to generate one from the Exchange dashboard.
        </p>

        <img
          src="/api-key.png"
          className="rounded-xl mb-6"
          alt="API key generation screenshot"
        />

        <ol className="list-decimal list-inside space-y-2">
          <li>
            Click your user icon in the top-right corner of the Exchange
            dashboard.
          </li>
          <li>
            In the left sidebar, select <strong>API</strong>.
          </li>
          <li>
            Click the <strong>"Generate New Key"</strong> button.
          </li>
          <li>
            Your new API key will be displayed—make sure to copy and store it
            securely.
          </li>
        </ol>
      </main>
    </>
  );
}
