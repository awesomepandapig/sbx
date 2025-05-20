import { ReactNode } from "react";

interface CardImageProps {
  height?: string;
  src: string;
  alt: string;
}

export function CardImage({ height = "h-20", src, alt }: CardImageProps) {
  return (
    <div
      className={`aspect-square ${height} bg-[rgba(255,255,255,0.06)] border border-[rgba(75,75,75,.7)] rounded-xl p-4`}
    >
      <img src={src} alt={alt} />
    </div>
  );
}

interface CardProps {
  width?: string;
  aspectRatio?: string;
  title: string;
  description: string;
  children: ReactNode;
  textPosition?: "left" | "center" | "right";
}

export default function Card({
  width = "w-96",
  aspectRatio = "aspect-square",
  title,
  description,
  children,
  textPosition = "center",
}: CardProps) {
  return (
    <div
      className={`${width} h-96 bg-[#0e0e0e] border border-[rgba(38,38,38,.7)] rounded-[20px] p-8 flex flex-col`}
    >
      <div className="flex-grow flex items-center justify-center text-gray-500">
        {children}
      </div>
      <div className={`text-${textPosition}`}>
        <h4 className="text-white font-medium text-xl mb-2">{title}</h4>
        <p className="text-gray-400">{description}</p>
      </div>
    </div>
  );
}
