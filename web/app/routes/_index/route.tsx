import Navbar from "~/components/landing/Navbar";
import Hero from "~/components/landing/Hero";
import SectionOne from "~/components/landing/SectionOne";
import SectionTwo from "~/components/landing/SectionTwo";
import Footer from "~/components/landing/Footer";

export default function Home() {
  return (
    <>
      <div className="min-h-screen flex items-center justify-center p-4">
        <div className="w-full max-w-7xl rounded-[30px] p-6">
          <div className="bg-black rounded-[20px] overflow-hidden"></div>
          <Navbar />
          <main className="pt-8 pb-[65px]">
            <Hero />
            <SectionOne />
            <SectionTwo />
          </main>
          <Footer />
        </div>
      </div>
    </>
  );
}
