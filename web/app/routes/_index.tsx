import Navbar from "~/components/landing/Navbar";
import Hero from "~/components/landing/Hero";
import SectionOne from "~/components/landing/SectionOne";
import SectionTwo from "~/components/landing/SectionTwo";
import Footer from "~/components/landing/Footer";

export default function Home() {
  return (
    <>
      <header>
        <Navbar />
      </header>
      <main>
        <Hero />
        <SectionOne />
        <SectionTwo />
      </main>
      <Footer />
    </>
  );
}
