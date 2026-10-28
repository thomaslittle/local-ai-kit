import { chromium } from "playwright-core";
const browser = await chromium.launch({ executablePath: "/usr/bin/google-chrome", headless: true });
const page = await browser.newPage({ viewport: { width: 1200, height: 630 }, deviceScaleFactor: 1 });
await page.goto("file:///home/tomlit/Projects/local-ai-kit-wt/site/apps/site/public/og-poster.svg");
await page.waitForTimeout(300);
await page.screenshot({ path: "/home/tomlit/Projects/local-ai-kit-wt/site/apps/site/public/og-poster.png" });
await browser.close();
