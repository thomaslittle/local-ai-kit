import { useEffect } from "react";
import { Nav } from "./components/Nav";
import { Footer } from "./components/Footer";
import { Hero } from "./sections/Hero";
import { Paths } from "./sections/Paths";
import { Comparison } from "./sections/Comparison";
import { Install } from "./sections/Install";
import { HowItWorks } from "./sections/HowItWorks";
import { Integrations } from "./sections/Integrations";
import { Providers } from "./sections/Providers";
import { Security } from "./sections/Security";
import { StatusDemo } from "./sections/StatusDemo";
import { Video } from "./sections/Video";
import { FinalCta } from "./sections/FinalCta";

function useReveal() {
  useEffect(() => {
    const nodes = Array.from(document.querySelectorAll<HTMLElement>(".reveal"));
    if (!("IntersectionObserver" in window)) {
      nodes.forEach((node) => node.classList.add("is-visible"));
      return;
    }
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            entry.target.classList.add("is-visible");
            observer.unobserve(entry.target);
          }
        }
      },
      { rootMargin: "0px 0px -12% 0px", threshold: 0.08 },
    );
    nodes.forEach((node) => observer.observe(node));
    // Off-screen sections become visible after first paint even if the visitor (or a
    // full-page screenshot) never scrolls them into view.
    const fallback = window.setTimeout(() => {
      nodes.forEach((node) => node.classList.add("is-visible"));
    }, 1400);
    return () => {
      observer.disconnect();
      window.clearTimeout(fallback);
    };
  }, []);
}

export default function App() {
  useReveal();

  return (
    <>
      <a className="skip-link" href="#main">Skip to content</a>
      <div className="ambient" aria-hidden="true" />
      <Nav />
      <main id="main">
        <Hero />
        <Paths />
        <Comparison />
        <Install />
        <HowItWorks />
        <Integrations />
        <Providers />
        <Security />
        <StatusDemo />
        <Video />
        <FinalCta />
      </main>
      <Footer />
    </>
  );
}
