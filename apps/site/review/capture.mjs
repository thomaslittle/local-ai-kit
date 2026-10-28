import { chromium } from "playwright-core";
import { existsSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const out = join(root, "review");
mkdirSync(out, { recursive: true });

const candidates = [
  "/usr/bin/google-chrome",
  "/usr/bin/chromium-browser",
  join(process.env.HOME ?? "", ".cache/ms-playwright/chromium_headless_shell-1248/chrome-linux/headless_shell"),
];
const executablePath = candidates.find((path) => existsSync(path));

const base = process.env.SITE_URL ?? "http://127.0.0.1:5174/";

const browser = await chromium.launch({
  executablePath,
  headless: true,
  args: ["--force-color-profile=srgb", "--hide-scrollbars"],
});

async function settle(page, ms = 1600) {
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(ms);
  // Trigger every reveal, then return to the top.
  await page.evaluate(async () => {
    const step = window.innerHeight * 0.75;
    for (let y = 0; y < document.body.scrollHeight; y += step) {
      window.scrollTo(0, y);
      await new Promise((r) => setTimeout(r, 80));
    }
    window.scrollTo(0, 0);
  });
  await page.waitForTimeout(700);
}

const desktop = await browser.newContext({
  viewport: { width: 1440, height: 960 },
  deviceScaleFactor: 1,
  colorScheme: "dark",
  reducedMotion: "no-preference",
});
const page = await desktop.newPage();
const errors = [];
page.on("console", (message) => {
  if (message.type() === "error") errors.push(message.text());
});
page.on("pageerror", (error) => errors.push(error.message));

await page.goto(base, { waitUntil: "domcontentloaded" });
await settle(page, 2600);
await page.screenshot({ path: join(out, "desktop-hero.png") });
await page.screenshot({ path: join(out, "desktop-full.png"), fullPage: true });

const sections = [
  ["paths", "desktop-paths.png"],
  ["compare-title", "desktop-comparison.png"],
  ["install", "desktop-install.png"],
  ["how", "desktop-how.png"],
  ["integrations", "desktop-integrations.png"],
  ["providers", "desktop-providers.png"],
  ["security", "desktop-security.png"],
  ["video", "desktop-video.png"],
];
for (const [id, name] of sections) {
  const locator = page.locator(`#${id}`);
  if (await locator.count()) {
    await locator.scrollIntoViewIfNeeded();
    await page.waitForTimeout(450);
    await page.screenshot({ path: join(out, name) });
  }
}
await desktop.close();

const mobile = await browser.newContext({
  viewport: { width: 390, height: 844 },
  deviceScaleFactor: 2,
  isMobile: true,
  hasTouch: true,
  colorScheme: "dark",
  reducedMotion: "no-preference",
});
const mobilePage = await mobile.newPage();
const mobileErrors = [];
mobilePage.on("pageerror", (error) => mobileErrors.push(error.message));
await mobilePage.goto(base, { waitUntil: "domcontentloaded" });
await settle(mobilePage, 2400);
await mobilePage.screenshot({ path: join(out, "mobile-hero.png") });
await mobilePage.screenshot({ path: join(out, "mobile-full.png"), fullPage: true });
await mobile.close();

const reduced = await browser.newContext({
  viewport: { width: 1440, height: 960 },
  reducedMotion: "reduce",
  colorScheme: "dark",
});
const reducedPage = await reduced.newPage();
await reducedPage.goto(base, { waitUntil: "networkidle" });
await reducedPage.waitForTimeout(800);
await reducedPage.screenshot({ path: join(out, "desktop-reduced-motion.png") });
await reduced.close();

const recorder = await browser.newContext({
  viewport: { width: 1440, height: 900 },
  recordVideo: { dir: join(out, "video"), size: { width: 1440, height: 900 } },
});
const recordPage = await recorder.newPage();
await recordPage.goto(base, { waitUntil: "networkidle" });
await recordPage.waitForTimeout(2400);
await recordPage.mouse.wheel(0, 420);
await recordPage.waitForTimeout(2200);
await recordPage.mouse.wheel(0, -420);
await recordPage.waitForTimeout(1200);
await recorder.close();

await browser.close();

const report = { executablePath, errors, mobileErrors };
console.log(JSON.stringify(report, null, 2));
