import { useEffect } from "react";
import { CardImage } from "./Card";
import Splide from "@splidejs/splide";
import { AutoScroll } from "@splidejs/splide-extension-auto-scroll";

// ⬇️ important CSS import
import "@splidejs/splide/dist/css/splide.min.css";

export default function AutoScrollingCarousel() {
  useEffect(() => {
    const splide = new Splide(".splide", {
      type: "loop",
      direction: "ltr",
      drag: false,
      focus: "center",
      perPage: 3,
      autoScroll: {
        speed: 0.1,
        pauseOnHover: false,
      },
      pagination: false,
      arrows: false,
      gap: "0.5rem", // Use a string value for gap
    });

    splide.mount({ AutoScroll });
  }, []);

  return (
    <div className="w-full h-full flex justify-center items-center">
      {/* no flex or weird layout on this container */}
      <div className="splide w-full max-w-6xl">
        <div className="splide__track">
          <ul className="splide__list">
            <li className="splide__slide">
              <div className="w-20 h-20 bg-[rgba(255,255,255,0.06)] border border-[rgba(75,75,75,.7)] rounded-xl flex items-center justify-center p-4">
                <img
                  className="scale-150"
                  src="/Jasper_Crystal.png"
                  alt="Jasper Crystal"
                />
              </div>
            </li>
            <li className="splide__slide">
              <div className="w-20 h-20 bg-[rgba(255,255,255,0.06)] border border-[rgba(75,75,75,.7)] rounded-xl flex items-center justify-center p-4">
                <img
                  className="scale-150"
                  src="/Vanguard_Helmet.png"
                  alt="Vanguard Helmet"
                />
              </div>
            </li>
            <li className="splide__slide">
              <div className="w-20 h-20 bg-[rgba(255,255,255,0.06)] border border-[rgba(75,75,75,.7)] rounded-xl flex items-center justify-center p-4">
                <img
                  className="scale-150"
                  src="/Golden_Dragon_Egg.png"
                  alt="Golden Dragon Egg"
                />
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}
