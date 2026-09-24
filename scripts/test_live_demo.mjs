import {test} from 'node:test';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {mkdtemp,mkdir,rm,readFile} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {resolve,join} from 'node:path';
import {chromium} from '@playwright/test';
const root=resolve(import.meta.dirname,'..');
const binary=resolve(root,'target/debug',process.platform==='win32'?'local-ai-kit.exe':'local-ai-kit');
async function setup(){
 const home=await mkdtemp(join(tmpdir(),'lai-browser-'));await mkdir(join(home,'.codex'));
 const child=spawn(binary,['serve','--http','--port','0','--home',home,'--assets',resolve(root,'apps/demo/dist'),'--allow-passive-discovery'],{stdio:['ignore','pipe','pipe'],windowsHide:true});
 let output='';const config=await new Promise((res,rej)=>{const timer=setTimeout(()=>rej(Error('HOST_START_TIMEOUT')),10000);child.on('error',rej);child.once('exit',()=>rej(Error('HOST_EXIT')));child.stdout.on('data',b=>{output+=b;const url=output.match(/http:\/\/127\.0\.0\.1:\d+/)?.[0],code=output.match(/minutes\): ([0-9a-f]{12})/)?.[1];if(url&&code){clearTimeout(timer);res({url,code});}});child.stderr.on('data',()=>{});});
 let browser;try{browser=await chromium.launch({headless:true,channel:process.env.LAI_BROWSER_CHANNEL||undefined});}catch(error){child.kill();await rm(home,{recursive:true,force:true});throw error;}const page=await browser.newPage();page.setDefaultTimeout(5000);const errors=[];page.on('pageerror',e=>errors.push(e.message));
 return {...config,page,browser,errors,async close(){await browser.close();child.kill();await new Promise(r=>{if(child.exitCode!==null)return r();child.once('exit',r);});await rm(home,{recursive:true,force:true});}};
}
test('real native host requires explicit pairing, then discovers real empty-home metadata',async()=>{
 const app=await setup();try{
  await app.page.goto(app.url);await app.page.getByRole('heading',{name:'Connect your local runtime'}).waitFor({timeout:5000});
  assert.equal(await app.page.locator('#mode-banner').count(),0);
  await app.page.getByLabel('Pairing code').fill(app.code);
  await app.page.getByRole('button',{name:'Pair with this runtime'}).click();
  await app.page.locator('#mode-banner.live').waitFor();
  await app.page.getByRole('button',{name:'Scan local clients'}).click();
  await app.page.getByRole('button',{name:'Connections',exact:true}).click();
  await app.page.getByText('Codex',{exact:false}).first().waitFor();
  assert.match(await app.page.locator('#mode-banner').innerText(),/Live connection/);
  assert.equal(await app.page.getByText('Synthetic showcase',{exact:true}).count(),0);
  assert.equal(app.errors.length,0,app.errors.join('\n'));
 }finally{await app.close();}
});
test('pairing rejects an incorrect code without revealing accounts',async()=>{
 const app=await setup();try{await app.page.goto(app.url);await app.page.getByLabel('Pairing code').fill('000000000000');await app.page.getByRole('button',{name:'Pair with this runtime'}).click();await app.page.getByRole('alert').waitFor();assert.equal(await app.page.locator('#mode-banner').count(),0);}finally{await app.close();}
});
