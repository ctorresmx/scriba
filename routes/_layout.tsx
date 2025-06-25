import { PageProps } from "$fresh/server.ts";
import Footer from "../components/Footer.tsx";
import Header from "../components/Header.tsx";

export default function Layout({ Component }: PageProps) {
  return (
    <div class="layout flex flex-col min-h-screen">
      <Header />
      <div class="flex-grow">
        <Component />
      </div>
      <Footer />
    </div>
  );
}
