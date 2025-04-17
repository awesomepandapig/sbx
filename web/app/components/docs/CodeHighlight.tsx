export default function CodeHighlight({ code }: { code: string }) {
    return (
      <code
        className="p-1 bg-gray-800 rounded-lg text-sm leading-[1.375] text-[#ebf4ff]"
        style={{
          fontFamily: `Consolas, Menlo, Monaco, "Andale Mono WT", "Andale Mono", "Lucida Console",
            "Lucida Sans Typewriter", "DejaVu Sans Mono", "Bitstream Vera Sans Mono",
            "Liberation Mono", "Nimbus Mono L", "Courier New", Courier, monospace`
        }}
      >
        {code}
      </code>
    );
}  