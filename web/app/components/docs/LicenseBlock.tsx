import { useEffect, useState } from "react";

export function LicenseBlock({ url }: { url: string }) {
  const [text, setText] = useState<string>("Loading...");

  useEffect(() => {
    fetch(url)
      .then((res) => res.text())
      .then(setText)
      .catch(() => setText("Failed to load license."));
  }, [url]);

  const lines = text.split("\n");

  return (
    <div className="bg-neutral-900 text-neutral-100 rounded-md overflow-hidden shadow-lg max-h-96 overflow-y-auto text-sm font-mono border border-neutral-700 mt-4">
      <div className="flex">
        <div className="bg-neutral-800 text-neutral-500 px-4 py-2 text-right select-none border-r border-neutral-700">
          {lines.map((_, i) => (
            <div key={i}>{i + 1}</div>
          ))}
        </div>
        <pre className="px-4 py-2 whitespace-pre-wrap break-words">{text}</pre>
      </div>
    </div>
  );
}
