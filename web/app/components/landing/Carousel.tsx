// import Splide from "@splidejs/splide";
// import { AutoScroll } from "@splidejs/splide-extension-auto-scroll";

// const AutoScrollingCarousel = () => {
//   useEffect(() => {
//     const splide = new Splide(".splide", {
//       type: "loop",
//       drag: false,
//       focus: "center",
//       perPage: 5, // Number of items per page
//       autoScroll: {
//         speed: 1 / 10, // Speed of auto-scrolling (higher is faster)
//         pauseOnHover: false,
//       },
//       pagination: false,
//       arrows: false,
//       gap: 4,
//     });

//     splide.mount({ AutoScroll }); // Mount the splide instance with AutoScroll extension
//   }, []);

//   return (
//     <div className="splide flex justify-center items-center h-full w-full z-0">
//       <div className="splide__track">
//         <ul className="splide__list">
//           {/* First Image */}
//           <li className="splide__slide">
//             <CardImage
//               src="https://wiki.hypixel.net/images/1/18/SkyBlock_items_divan_pendant.png"
//               alt="Divan Pendant"
//             />
//           </li>

//           {/* Second Image */}
//           <li className="splide__slide">
//             <CardImage
//               src="https://wiki.hypixel.net/images/7/72/SkyBlock_items_jasper_crystal.png"
//               alt="Jasper Crystal"
//             />
//           </li>

//           {/* Third Image */}
//           <li className="splide__slide">
//             <div className="w-20 h-20 bg-[rgba(255,255,255,0.06)] border border-[rgba(75,75,75,.7)] rounded-xl flex items-center justify-center p-4">
//               <img
//                 className="scale-150"
//                 src="/Vanguard_Helmet.png"
//                 alt="Divan Pendant"
//               />
//             </div>
//           </li>

//           {/* Fourth Image */}
//           <li className="splide__slide">
//             <CardImage
//               src="https://wiki.hypixel.net/images/7/75/SkyBlock_pets_golden_dragon.png"
//               alt="Jasper Crystal"
//             />
//           </li>
//         </ul>
//       </div>
//     </div>
//   );
// };
