import { test, before, after } from 'node:test';
import assert from 'node:assert/strict';
import { chromium } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { createFixtureServer } from '../apps/demo/server.mjs';
let server, browser;
before(async()=>{server=await createFixtureServer();browser=await chromium.launch({headless:true});});
after(async()=>{await browser?.close();await server?.close();});
const tags=['wcag2a','wcag2aa','wcag21a','wcag21aa'];
const details=result=>result.violations.map(v=>({id:v.id,impact:v.impact,nodes:v.nodes.map(n=>({target:n.target,summary:n.failureSummary}))}));
for(const variant of ['rail','compact','palette','capability','compare','allowance','preset','mobile']){
 test(`showroom ${variant}: no automated WCAG A/AA violations`,async()=>{
  const context=await browser.newContext({viewport:variant==='mobile'?{width:390,height:844}:{width:1440,height:1000}});
  const page=await context.newPage();
  try{
   await page.goto(server.url);await page.locator('#mode-banner.synthetic').waitFor();
   await page.locator(`[data-variant="${variant}"]`).click();
   const result=await new AxeBuilder({page}).withTags(tags).analyze();
   assert.deepEqual(details(result),[]);
  }finally{await context.close();}
 });
}
for(const section of ['Overview','Connections','Models','Usage','Playground','Diagnostics']){
 test(`showroom ${section}: no automated WCAG A/AA violations`,async()=>{
  const context=await browser.newContext();const page=await context.newPage();
  try{
   await page.goto(server.url);await page.locator('#mode-banner.synthetic').waitFor();
   await page.getByRole('button',{name:section,exact:true}).click();
   assert.deepEqual(details(await new AxeBuilder({page}).withTags(tags).analyze()),[]);
  }finally{await context.close();}
 });
}
