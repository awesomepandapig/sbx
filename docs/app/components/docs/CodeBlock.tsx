import { useState } from "react";
import Prism from "prismjs";
import { Copy, Check } from "lucide-react";
import "./prism-duotone-sea.css";
import "prismjs/components/prism-json.js";

interface CodeBlockProps {
  code: string;
  language: string;
  className?: string;
}

export function escapeJSONString(obj: Object) {
  return JSON.stringify(obj, null, 2);
}

export default function CodeBlock({
  className,
  code,
  language,
}: CodeBlockProps) {
  const [copied, setCopied] = useState(false);
  const highlighted = Prism.highlight(
    code,
    Prism.languages[language],
    language,
  );

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch (err) {
      console.error("Copy failed:", err);
    }
  };

  return (
    <div
      className={`${className} group relative bg-gray-800 text-white rounded-lg shadow-md`}
    >
      {/* Copy icon button */}
      <button
        onClick={handleCopy}
        className="absolute top-2 right-2 text-white p-1 rounded hover:bg-gray-700 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
        aria-label="Copy code"
      >
        {copied ? <Check size={18} /> : <Copy size={18} />}
      </button>

      <pre className="overflow-x-auto p-4">
        <code
          className={`language-${language}`}
          dangerouslySetInnerHTML={{ __html: highlighted }}
        />
      </pre>
    </div>
  );
}
