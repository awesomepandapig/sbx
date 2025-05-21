import { ReactNode } from "react";

interface CardProps {
  width?: string;
  title: string;
  description: string;
  children: ReactNode;
  textPosition?: "left" | "center" | "right";
}

export default function SVGCard({
  width = "w-96",
  title,
  description,
  children,
  textPosition = "center",
}: CardProps) {
  return (
    <div
      className={`${width} h-96 bg-[#0e0e0e] border border-[rgba(38,38,38,.7)] rounded-[20px] relative overflow-hidden`}
    >
      <div className="absolute inset-0 w-full h-full">{children}</div>

      <div className="relative h-full flex flex-col">
        <div className="flex-grow"></div>
        <div className={`text-${textPosition} p-8`}>
          <h4 className="text-white font-medium text-xl mb-2">{title}</h4>
          <p className="text-gray-400">{description}</p>
        </div>
      </div>
    </div>
  );
}
