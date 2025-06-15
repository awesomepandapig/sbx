import React, { useState } from "react"
import { ReactNode } from "react";
import SVGCard from "./SVGCard" // Assuming SVGCard is in the same directory
import { ArrowUpRight } from "lucide-react";
import { DOCS_URL } from "~/lib/config";

interface CardProps {
  width?: string;
  title: string;
  description: string;
  children: ReactNode;
  textPosition?: "left" | "center" | "right";
}

export default function APICard() {
  const [activeTab, setActiveTab] = useState("python")

  // Define the code snippets for each language
  const snippets = {
    python: {
      code: [
        { type: "keyword", content: "import" },
        { type: "space", content: " " },
        { type: "module", content: "requests" },
        { type: "newline", content: "\n\n" },
        { type: "module", content: "requests" },
        { type: "operator", content: "." },
        { type: "function", content: "post" },
        { type: "bracket", content: "(" },
        { type: "newline", content: "\n  " },
        { type: "string", content: "'https://api.skyblock.exchange/v1/orders'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n  " },
        { type: "param", content: "headers" },
        { type: "operator", content: "=" },
        { type: "bracket", content: "{" },
        { type: "newline", content: "\n    " },
        { type: "string", content: "'x-api-key'" },
        { type: "operator", content: ": " },
        { type: "string", content: "'YOUR_API_KEY'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "string", content: "'Content-Type'" },
        { type: "operator", content: ": " },
        { type: "string", content: "'application/json'" },
        { type: "newline", content: "\n  " },
        { type: "bracket", content: "}" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n  " },
        { type: "param", content: "json" },
        { type: "operator", content: "=" },
        { type: "bracket", content: "{" },
        { type: "newline", content: "\n    " },
        { type: "string", content: "'product_id'" },
        { type: "operator", content: ": " },
        { type: "string", content: "'JSP'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "string", content: "'side'" },
        { type: "operator", content: ": " },
        { type: "string", content: "'buy'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "string", content: "'type'" },
        { type: "operator", content: ": " },
        { type: "string", content: "'limit'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "string", content: "'size'" },
        { type: "operator", content: ": " },
        { type: "number", content: "1" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "string", content: "'price'" },
        { type: "operator", content: ": " },
        { type: "number", content: "90" },
        { type: "newline", content: "\n  " },
        { type: "bracket", content: "}" },
        { type: "newline", content: "\n" },
        { type: "bracket", content: ")" },
      ],
    },
    curl: {
      code: [
        { type: "command", content: "curl" },
        { type: "space", content: " " },
        { type: "url", content: "https://api.skyblock.exchange/v1/orders" },
        { type: "operator", content: " \\" },
        { type: "newline", content: "\n  " },
        { type: "param", content: "-H" },
        { type: "space", content: " " },
        { type: "string", content: '"x-api-key: YOUR_API_KEY"' },
        { type: "operator", content: " \\" },
        { type: "newline", content: "\n  " },
        { type: "param", content: "-H" },
        { type: "space", content: " " },
        { type: "string", content: '"Content-Type: application/json"' },
        { type: "operator", content: " \\" },
        { type: "newline", content: "\n  " },
        { type: "param", content: "-d" },
        { type: "space", content: " " },
        { type: "bracket", content: "{" },
        { type: "newline", content: "\n    " },
        { type: "property", content: '"product_id"' },
        { type: "operator", content: ":" },
        { type: "space", content: " " },
        { type: "string", content: '"JSP"' },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: '"side"' },
        { type: "operator", content: ":" },
        { type: "space", content: " " },
        { type: "string", content: '"buy"' },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: '"type"' },
        { type: "operator", content: ":" },
        { type: "space", content: " " },
        { type: "string", content: '"limit"' },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: '"size"' },
        { type: "operator", content: ":" },
        { type: "space", content: " " },
        { type: "number", content: "1" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: '"price"' },
        { type: "operator", content: ":" },
        { type: "space", content: " " },
        { type: "number", content: "90" },
        { type: "newline", content: "\n" },
        { type: "bracket", content: "}" },
      ],
    },
    js: {
      code: [
        { type: "function", content: "fetch" },
        { type: "bracket", content: "(" },
        { type: "string", content: "'https://api.skyblock.exchange/v1/orders'" },
        { type: "operator", content: ", " },
        { type: "bracket", content: "{" },
        { type: "newline", content: "\n  " },
        { type: "property", content: "method" },
        { type: "operator", content: ": " },
        { type: "string", content: "'POST'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n  " },
        { type: "property", content: "headers" },
        { type: "operator", content: ": " },
        { type: "bracket", content: "{" },
        { type: "newline", content: "\n    " },
        { type: "property", content: "'x-api-key'" },
        { type: "operator", content: ": " },
        { type: "string", content: "'YOUR_API_KEY'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: "'Content-Type'" },
        { type: "operator", content: ": " },
        { type: "string", content: "'application/json'" },
        { type: "newline", content: "\n  " },
        { type: "bracket", content: "}" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n  " },
        { type: "property", content: "body" },
        { type: "operator", content: ": " },
        { type: "function", content: "JSON.stringify" },
        { type: "bracket", content: "({" },
        { type: "newline", content: "\n    " },
        { type: "property", content: "product_id" },
        { type: "operator", content: ": " },
        { type: "string", content: "'JSP'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: "side" },
        { type: "operator", content: ": " },
        { type: "string", content: "'buy'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: "type" },
        { type: "operator", content: ": " },
        { type: "string", content: "'limit'" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: "size" },
        { type: "operator", content: ": " },
        { type: "number", content: "1" },
        { type: "operator", content: "," },
        { type: "newline", content: "\n    " },
        { type: "property", content: "price" },
        { type: "operator", content: ": " },
        { type: "number", content: "90" },
        { type: "newline", content: "\n  " },
        { type: "bracket", content: "})" },
        { type: "newline", content: "\n" },
        { type: "bracket", content: "})" },
      ],
    },
  }

  // Define color mapping for syntax highlighting
  const colorMap = {
    command: "#57718e",
    url: "#47ebb4",
    param: "#47ebb4",
    string: "#47ebb4",
    property: "#7eb6f6",
    number: "#0aa370",
    operator: "#4a5f78",
    bracket: "#4a5f78",
    function: "#47ebb4",
    keyword: "#c586c0",
    module: "#569cd6",
  }

  // Function to render code with syntax highlighting
  const renderCode = (codeArray) => {
    return codeArray.map((segment, index) => {
      if (segment.type === "newline") {
        return <React.Fragment key={index}>{segment.content}</React.Fragment>
      }
      return (
        <span key={index} style={{ color: colorMap[segment.type] || "#e0e0e0" }}>
          {segment.content}
        </span>
      )
    })
  }

  const handleTabClick = (tab) => {
    console.log("Tab clicked:", tab); // Add this log
    setActiveTab(tab);
  };

  return (
    <div
      className={`hidden md:col-span-2 h-96 bg-[#0e0e0e] border border-[rgba(38,38,38,.7)] md:grid grid-cols-2 rounded-[20px] overflow-hidden`}
    >
        <div className="h-full">
        
        <div className={`text-left p-8`}>
          <h4 className="text-white font-medium text-xl mb-2">A foundation to build upon</h4>
          <p className="text-gray-400">Create your own client with our FIX, REST, & WebSocket APIs.</p>
        </div>
      </div>

      <div className="w-full h-full">

        <div className="bg-[#1e1e1e] h-full">
        <div className="flex border-b border-[#333]">
          {Object.keys(snippets).map((tab) => (
            <button
              key={tab}
              type="button" // Explicitly setting type for clarity, though often default for <button>
              className={`px-3 py-2 text-xs font-medium transition-colors duration-200 ${
                activeTab === tab ? "text-[#47ebb4] border-b border-b-[#47ebb4] border-r-[#333] border-r" : "text-gray-400 hover:text-gray-300 border-r border-[#333]"
              }`}
              onClick={() => handleTabClick(tab)}
            >
              {tab.toUpperCase()}
            </button>
          ))}
        </div>
        <div className="relative overflow-x-scroll">
          <pre className="p-4 text-sm font-mono">
            <code>{renderCode(snippets[activeTab].code)}</code>
          </pre>
        </div>
      </div>
      </div>
    </div>
  );
}