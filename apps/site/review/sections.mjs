import { chromium } from "playwright-core";
import { join, resolve } from "node:path";
const root = resolve(import.meta.dirname, "..");
const out = join(root, "review");
const browser = await chromium.launch({ executablePath: "/usr/bin/google-chrome", headless: true });

const targets = [
  [".paths", "paths"],
  ["section:has(.compare-wrap)", "comparison"],
  ["#install", "install"],
  ["#how", "how"],
  ["#integrations", "integrations"],
  ["#providers", "providers"],
  ["#security", "security"],
  ["section:has(.status-demo)", "status"],
  ["#video", "video"],
];

for (const [width, height, label, scale] of [[1440, 960, "desktop", 1], [390, 844, "mobile", 2]]) {
  const page = await browser.newPage({ viewport: { width, height }, deviceScaleFactor: scale, colorScheme: "dark", isMobile: width < 600, hasTouch: width < 600 });
  await page.goto("http://127.0.0.1:5174/?3d", { waitUntil: "load" });
  await page.waitForTimeout(2200);
  await page.evaluate(async () => {
    for (let y = 0; y < document.body.scrollHeight; y += window.innerHeight * 0.8) {
      window.scrollTo(0, y);
      await new Promise((r) => setTimeout(r, 100));
    }
    window.scrollTo(0, 0);
  });
  await page.waitForTimeout(800);
  for (const [selector, name] of targets) {
    const locator = page.locator(selector).first();
    if (await locator.count()) {
      await locator.scrollIntoViewIfNeeded();
      await page.waitForTimeout(500);
      await locator.screenshot({ path: join(out, `${label}-${name}.png`) });
    }
  }
  await page.locator(".hero-stage").scrollIntoViewIfNeeded();
  await page.waitForTimeout(1200);
  await page.screenshot({ path: join(out, `${label}-stage.png`) });
  await page.close();
}
await browser.close();
console.log("sections captured");
