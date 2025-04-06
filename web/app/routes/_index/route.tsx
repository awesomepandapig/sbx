import Navbar from "~/components/landing/Navbar";
import Hero from "~/components/landing/Hero";
import SectionOne from "~/components/landing/SectionOne";
import SectionTwo from "~/components/landing/SectionTwo";
import Footer from "~/components/landing/Footer";
import { useLoaderData } from "@remix-run/react";

import { authLoader } from "~/lib/auth";
export const loader = authLoader;

export default function Home() {
  const { user } = useLoaderData<typeof loader>();

  return (
    <>
      <header>
        <Navbar authenticated={user != null} />
      </header>
      <main>
        <Hero authenticated={user != null} />
        <SectionOne />
        <SectionTwo />
      </main>
      <Footer />
    </>
  );
}
