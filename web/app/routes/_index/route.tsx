import Navbar from "~/components/landing/Navbar";
import Hero from "~/components/landing/Hero";
import SectionOne from "~/components/landing/SectionOne";
import SectionTwo from "~/components/landing/SectionTwo";
import Footer from "~/components/landing/Footer";

export default function Home() {
  return (
    <>
      <div className="bg-black rounded-[20px] overflow-hidden"></div>
      <Navbar />
      <main className="pt-16 p-4 lg:pt-20 lg:mb-16 max-w-7xl m-auto items-center justify-center">
        <Hero />
        <SectionOne />
        <SectionTwo />
      </main>
      <Footer />
    </>
  );
}
