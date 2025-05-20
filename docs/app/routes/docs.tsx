import { Outlet } from "@remix-run/react";

import Header from "~/components/docs/Header";
import LeftSidebar from "~/components/docs/LeftSidebar";

export default function Home() {
  return (
    <>
      <Header />
      <LeftSidebar />
      <main className="text-white ml-64 mr-64 mt-12 p-12">
        <Outlet />
      </main>
    </>
  );
}
