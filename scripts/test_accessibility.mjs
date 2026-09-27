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
async function mountFixture(page){
 await page.route('**/bootstrap.mjs',route=>route.fulfill({status:200,contentType:'text/javascript',body:'// The accessibility fixture mounts the public demo explicitly.'}));
 await page.goto(server.url);
 await page.evaluate(async()=>{
  const [{mountDemo},{createHttpTransport,createLocalAI}]=await Promise.all([import('/app.mjs'),import('/modules/sdk/index.js')]);
  const response=await fetch('/fixture/bootstrap',{credentials:'omit',cache:'no-store'}),bootstrap=await response.json();
  const client=await createLocalAI({clientName:'local-ai-kit-a11y-fixture',transport:createHttpTransport({baseUrl:location.origin,token:bootstrap.sessionToken})});
  await mountDemo({connectFixture:async()=>client,initialMode:'synthetic'});
 });
 await page.locator('#mode-banner.synthetic').waitFor();
}
for(const variant of ['rail','compact','palette','capability','compare','allowance','preset','mobile']){
 test(`showroom ${variant}: no automated WCAG A/AA violations`,async()=>{
  const context=await browser.newContext({viewport:variant==='mobile'?{width:390,height:844}:{width:1440,height:1000}});
  const page=await context.newPage();
  try{
   await mountFixture(page);
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
   await mountFixture(page);
   await page.getByRole('button',{name:section,exact:true}).click();
   assert.deepEqual(details(await new AxeBuilder({page}).withTags(tags).analyze()),[]);
 }finally{await context.close();}
});
}
test('showroom presents only authenticated profiles as connected',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await mountFixture(page);
  await page.getByRole('button',{name:'Connections',exact:true}).click();
  const verified=page.getByTestId('verified-profiles');
  assert.equal(await verified.locator('[data-connection-id]').count(),6);
  assert.match(await verified.innerText(),/Personal workspace/);
  assert.doesNotMatch(await verified.innerText(),/Expired sign-in/);
  const previouslyAuthenticated=page.getByTestId('previously-authenticated-profiles');
  assert.equal(await previouslyAuthenticated.locator('[data-connection-id]').count(),1);
  assert.match(await previouslyAuthenticated.innerText(),/Work workspace/);
  assert.match(await previouslyAuthenticated.innerText(),/Previously authenticated/);
  assert.match(await previouslyAuthenticated.innerText(),/stale/i);
  assert.equal(await previouslyAuthenticated.isVisible(),true);
  const detected=page.getByTestId('detected-profiles');
  assert.equal(await detected.locator('[data-connection-id]').count(),1);
  assert.equal(await detected.getByText('Expired sign-in').isVisible(),true);
  assert.match(await detected.innerText(),/Expired sign-in/);
  assert.match(await detected.innerText(),/Expired/);
 }finally{await context.close();}
});
test('overview and connections distinguish stale evidence from never-verified profiles',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await page.route('**/bootstrap.mjs',route=>route.fulfill({status:200,contentType:'text/javascript',body:'// Render the public pages against the fixture SDK.'}));
  await page.goto(server.url);
  await page.evaluate(async()=>{
   const [{overviewPage,connectionsPage},{createHttpTransport,createLocalAI}]=await Promise.all([import('/pages.mjs'),import('/modules/sdk/index.js')]);
   const response=await fetch('/fixture/bootstrap',{credentials:'omit',cache:'no-store'}),bootstrap=await response.json();
   const client=await createLocalAI({clientName:'local-ai-kit-stale-overview-fixture',transport:createHttpTransport({baseUrl:location.origin,token:bootstrap.sessionToken})});
   const snapshot=await client.snapshot.get();await client.close();
   snapshot.connections=snapshot.connections.map(connection=>connection.auth.value==='authenticated'?{...connection,auth:{...connection.auth,freshness:'stale'}}:connection);
   const ctx={state:{snapshot,mode:'live'},connections:snapshot.connections,providers:snapshot.providers,all:[],probeConnection(){},render(){}};
   document.querySelector('#app').replaceChildren(overviewPage(ctx),connectionsPage(ctx));
  });
  const text=await page.locator('body').innerText();
  assert.match(text,/Previously authenticated/);
  assert.match(text,/Personal workspace/);
  assert.doesNotMatch(text,/No authenticated profiles yet/);
  assert.doesNotMatch(text,/No fresh authenticated profiles|No fresh authentication checks/);
  assert.equal(await page.getByRole('heading',{name:'Verified profiles',exact:true}).count(),0);
  assert.equal(await page.getByRole('heading',{name:'Authenticated profiles',exact:true}).count(),0);
  const overviewStale=page.getByTestId('previously-authenticated-overview');
  assert.equal(await overviewStale.isVisible(),true);
  assert.match(await overviewStale.innerText(),/Personal workspace/);
  assert.equal(await page.getByTestId('previously-authenticated-profiles').isVisible(),true);
 }finally{await context.close();}
});
test('detected client disclosure remains open across page rerenders unless the user closes it',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await mountFixture(page);
  await page.getByRole('button',{name:'Connections',exact:true}).click();
  let detected=page.getByTestId('detected-profiles');
  assert.equal(await detected.evaluate(e=>e.open),true);
  await detected.locator(':scope > summary').click();
  await page.getByRole('button',{name:'Overview',exact:true}).click();
  await page.getByRole('button',{name:'Connections',exact:true}).click();
  detected=page.getByTestId('detected-profiles');
  assert.equal(await detected.evaluate(e=>e.open),false);
 }finally{await context.close();}
});
test('local runtimes and confirmed unauthenticated profiles have separate live states',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await page.route('**/bootstrap.mjs',route=>route.fulfill({status:200,contentType:'text/javascript',body:'// Render public connection cards against the fixture SDK.'}));
  await page.goto(server.url);
  await page.evaluate(async()=>{
   const [{connectionsPage},{createHttpTransport,createLocalAI}]=await Promise.all([import('/pages.mjs'),import('/modules/sdk/index.js')]);
   const response=await fetch('/fixture/bootstrap',{credentials:'omit',cache:'no-store'}),bootstrap=await response.json();
   const client=await createLocalAI({clientName:'local-ai-kit-profile-states-fixture',transport:createHttpTransport({baseUrl:location.origin,token:bootstrap.sessionToken})});
   const snapshot=await client.snapshot.get();await client.close();
   const source={kind:'native',adapterId:'fixture',adapterVersion:'1',reference:'fixture'};
   const localBilling={state:'known',value:'local',source,observedAt:new Date().toISOString(),freshness:'fresh',verification:'observed'};
   snapshot.providers.push({id:'lmstudio',name:'LM Studio',kind:'local_runtime'});
   snapshot.connections.push({id:'local-lm',providerId:'lmstudio',displayName:'LM Studio · Default profile',installationId:'install-lm',profileId:'profile-lm',accountId:null,auth:{state:'unknown',reason:'LOCAL_RUNTIME_HAS_NO_ACCOUNT_AUTH'},plan:{state:'unknown',reason:'NOT_REPORTED'},status:'needs_auth',diagnostics:[]});
   snapshot.connections.push({id:'signed-out',providerId:'codex',displayName:'Codex · Additional profile',installationId:'install-codex',profileId:'profile-codex',accountId:null,auth:{state:'known',value:'unauthenticated',source,observedAt:new Date().toISOString(),freshness:'fresh',verification:'observed'},plan:{state:'unknown',reason:'NOT_REPORTED'},status:'needs_auth',diagnostics:[]});
   snapshot.routes.push({id:'route-lm',connectionId:'local-lm',providerId:'lmstudio',displayName:'LM Studio',billing:localBilling,capabilities:[],catalogRevision:'fixture-lm'});
   snapshot.models.push({id:'model-lm',routeId:'route-lm',nativeId:'ornith-1.5',name:'Ornith 1.5',revision:'fixture-lm',isDefault:false,capabilities:[],options:[],contextWindow:{state:'unknown',reason:'NOT_REPORTED'}});
   const ctx={state:{snapshot,mode:'live',detectedExpanded:true},connections:snapshot.connections,providers:snapshot.providers,all:[],probeConnection(){}};
   document.querySelector('#app').replaceChildren(connectionsPage(ctx));
  });
  await page.getByRole('heading',{name:'Local runtimes',exact:true}).waitFor();
  assert.equal(await page.getByRole('heading',{name:'LM Studio · Default profile',exact:true}).isVisible(),true);
  assert.equal(await page.getByRole('button',{name:'Refresh local catalog',exact:true}).isVisible(),true);
  const signedOut=page.getByTestId('unauthenticated-profiles');
  assert.equal(await signedOut.locator('[data-connection-id="signed-out"]').count(),1);
  assert.match(await signedOut.innerText(),/Not authenticated/);
  assert.equal(await page.getByTestId('detected-profiles').locator('[data-connection-id="local-lm"]').count(),0);
 }finally{await context.close();}
});
test('live connections expose an explicit bulk metadata check',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await page.route('**/bootstrap.mjs',route=>route.fulfill({status:200,contentType:'text/javascript',body:'// Render public connection cards against the fixture SDK.'}));
  await page.goto(server.url);
  await page.evaluate(async()=>{
   const [{connectionsPage},{createHttpTransport,createLocalAI}]=await Promise.all([import('/pages.mjs'),import('/modules/sdk/index.js')]);
   const response=await fetch('/fixture/bootstrap',{credentials:'omit',cache:'no-store'}),bootstrap=await response.json();
   const client=await createLocalAI({clientName:'local-ai-kit-bulk-check-fixture',transport:createHttpTransport({baseUrl:location.origin,token:bootstrap.sessionToken})});
   const snapshot=await client.snapshot.get();await client.close();snapshot.mode='live';
   const ctx={state:{snapshot,mode:'live',detectedExpanded:true},connections:snapshot.connections,providers:snapshot.providers,all:[],probeConnection(){},probeAllConnections(){document.body.dataset.bulkProbe='started';}};
   document.querySelector('#app').replaceChildren(connectionsPage(ctx));
  });
  await page.getByRole('button',{name:'Check all detected profiles',exact:true}).click();
  assert.equal(await page.locator('body').getAttribute('data-bulk-probe'),'started');
  assert.match(await page.locator('#app').innerText(),/metadata-only/i);
 }finally{await context.close();}
});
test('detected profiles with completed checks are not labeled as still unchecked',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await page.route('**/bootstrap.mjs',route=>route.fulfill({status:200,contentType:'text/javascript',body:'// Render checked profile outcomes.'}));
  await page.goto(server.url);
  await page.evaluate(async()=>{
   const [{connectionsPage},{createHttpTransport,createLocalAI}]=await Promise.all([import('/pages.mjs'),import('/modules/sdk/index.js')]);
   const response=await fetch('/fixture/bootstrap',{credentials:'omit',cache:'no-store'}),bootstrap=await response.json();
   const client=await createLocalAI({clientName:'local-ai-kit-checked-profile-label-fixture',transport:createHttpTransport({baseUrl:location.origin,token:bootstrap.sessionToken})});
   const snapshot=await client.snapshot.get();await client.close();snapshot.mode='live';
   const probeResults=Object.fromEntries(snapshot.connections.map(connection=>[connection.id,{ok:false,message:'Metadata check unavailable · provider runtime unavailable'}]));
   const ctx={state:{snapshot,mode:'live',detectedExpanded:true,probeResults},connections:snapshot.connections,providers:snapshot.providers,all:[],probeConnection(){}};
   document.querySelector('#app').replaceChildren(connectionsPage(ctx));
  });
  const detected=page.getByTestId('detected-profiles');
  assert.match(await detected.innerText(),/checked, not confirmed/);
  assert.doesNotMatch(await detected.innerText(),/need a metadata check/);
  assert.match(await detected.innerText(),/Metadata check unavailable/);
 }finally{await context.close();}
});
test('showroom keeps revision details in the exact-payload disclosure',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await mountFixture(page);
  await page.getByRole('button',{name:'Selector lab',exact:true}).click();
  await page.locator('[data-variant="rail"]').click();
  const text=await page.locator('#variant-view').innerText();
  assert.doesNotMatch(text,/input\.(?:text|image)/);
  const config=await page.locator('#configuration-panel').innerText();
  assert.doesNotMatch(config,/catalog_[a-f0-9]{20}/);
  assert.equal(await page.locator('#configuration-panel details[open]').count(),0);
 }finally{await context.close();}
});
test('showroom explains when required capability evidence is unknown',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await mountFixture(page);
  await page.getByRole('button',{name:'Selector lab',exact:true}).click();
  await page.locator('[data-variant="capability"]').click();
  await page.getByLabel('Image input').check();
  assert.equal(await page.getByLabel('Image input').isChecked(),true);
  const candidates=await page.locator('#variant-view').innerText();
  assert.match(candidates,/Image input is not confirmed/);
  assert.doesNotMatch(candidates,/CAPABILITY_UNKNOWN/);
  await page.getByLabel('Only eligible routes').check();
  await page.getByText('No routes have confirmed all selected capabilities.',{exact:true}).waitFor();
  await page.getByText('Unknown support is not treated as supported.',{exact:true}).waitFor();
 }finally{await context.close();}
});
test('showroom selectors with a populated catalog stay within responsive viewports',async()=>{
 const context=await browser.newContext({viewport:{width:1440,height:960}});const page=await context.newPage();
 try{
  await mountFixture(page);
  await page.getByRole('button',{name:'Selector lab',exact:true}).click();
  for(const width of [1440,1324,1024,390]){
   await page.setViewportSize({width,height:900});
   for(const variant of ['rail','compact','palette','capability','compare','allowance','preset','mobile']){
    await page.locator(`[data-variant="${variant}"]`).click();
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth),true,`${variant} overflows at ${width}px`);
    assert.equal(await page.evaluate(()=>document.querySelector('.configuration').getBoundingClientRect().right<=window.innerWidth),true,`${variant} configuration is clipped at ${width}px`);
    assert.equal(await page.evaluate(()=>document.querySelector('.top-actions').getBoundingClientRect().right<=window.innerWidth),true,`${variant} top actions are clipped at ${width}px`);
   }
  }
 }finally{await context.close();}
});
test('showroom rendered text has no common mojibake markers',async()=>{
 const context=await browser.newContext();const page=await context.newPage();
 try{
  await mountFixture(page);
  for(const section of ['Overview','Connections','Models','Usage','Selector lab','Playground','Diagnostics']){
   await page.getByRole('button',{name:section,exact:true}).click();
   const text=await page.locator('body').innerText();
   assert.doesNotMatch(text,/(?:Â|Ã|â€|â€¦|ï»¿|ðŸ)/,`${section} contains garbled text`);
  }
 }finally{await context.close();}
});
