import { LicenseBlock } from "~/components/LicenseBlock";

export default function Licenses() {
  return (
    <div className="text-white p-8 ml-auto mr-auto">
      <div className="mb-4">
        <h1 className="text-4xl font-bold">Third Party Licenses</h1>
        <br />
        <p>
          This page displays all the relevant licenses for third party tools we
          depend on.
        </p>
      </div>

      <div className="p-4 space-y-8 border-t border-blue-500">
        <section>
          <h2 className="text-lg font-semibold mb-4">Lightweight Charts™</h2>
          <a
            className="text-blue-500"
            href="https://www.tradingview.com/"
            target="_blank"
          >
            Visit website
          </a>
          <div className="mt-4 mb-4">
            <label className="text-gray-400">NOTICE:</label>
            <LicenseBlock url="/licenses/lightweight-charts-NOTICE.txt" />
          </div>
          <div className="mb-4">
            <label className="text-gray-400">LICENSE:</label>
            <LicenseBlock url="/licenses/lightweight-charts-LICENSE.txt" />
          </div>
        </section>
      </div>
    </div>
  );
}
