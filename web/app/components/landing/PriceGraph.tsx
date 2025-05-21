export function PriceGraph() {
  return (
    <svg
      width="100%"
      height="100%"
      viewBox="0 0 400 150"
      preserveAspectRatio="xMidYMid meet"
      className="w-full h-full rounded-[16px]"
    >
      {/* Define clip path for rounded corners */}
      <defs>
        <clipPath id="rounded-corners">
          <rect x="0" y="0" width="400" height="150" rx="16" ry="16" />
        </clipPath>
      </defs>

      {/* Background */}
      <rect
        x="0"
        y="0"
        width="400"
        height="150"
        fill="#0e0e0e"
        rx="16"
        ry="16"
      />

      {/* Content group with clip path applied */}
      <g clipPath="url(#rounded-corners)">
        {/* Grid lines for background */}

        {/* Main price line */}
        <path
          d="M0,75 C20,90 40,60 60,70 S100,100 120,85 S160,45 180,55 S220,75 240,60 S280,70 300,55 S340,40 360,45 S380,60 400,55"
          fill="none"
          stroke="#0066FF"
          strokeWidth="2.5"
          strokeLinecap="round"
          strokeLinejoin="round"
        />

        {/* Gradient area under the line */}
        <linearGradient id="gradient" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stopColor="#0066FF" stopOpacity="0.3" />
          <stop offset="100%" stopColor="#0066FF" stopOpacity="0" />
        </linearGradient>
        <path
          d="M0,75 C20,90 40,60 60,70 S100,100 120,85 S160,45 180,55 S220,75 240,60 S280,70 300,55 S340,40 360,45 S380,60 400,55 L400,150 L0,150 Z"
          fill="url(#gradient)"
        />

        {/* Data points */}
        <g className="data-points">
          {[
            [60, 70],
            [120, 85],
            [180, 55],
            [240, 60],
            [300, 55],
            [360, 45],
          ].map(([x, y], i) => (
            <circle
              key={i}
              cx={x}
              cy={y}
              r="2.5"
              fill="#0066FF"
              stroke="#0e0e0e"
              strokeWidth="1.5"
            />
          ))}
        </g>
      </g>
    </svg>
  );
}
