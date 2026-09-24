import {createHttpTransport,createLocalAI} from '/modules/sdk/index.js';
import {mountDemo} from './app.mjs';
import {el,button} from './ui.mjs';
async function connect(endpoint,token){return createLocalAI({clientName:'local-ai-kit-showcase',transport:createHttpTransport({baseUrl:endpoint,token})});}
async function fixture(){
 const response=await fetch('/fixture/bootstrap',{credentials:'omit',cache:'no-store',redirect:'error'});
 if(!response.ok)throw Error('FIXTURE_BOOTSTRAP_FAILED');const bootstrap=await response.json();
 if(bootstrap.mode!=='synthetic'||typeof bootstrap.sessionToken!=='string')throw Error('INVALID_FIXTURE_BOOTSTRAP');
 return connect(window.location.origin,bootstrap.sessionToken);
}
function pairing(){
 return new Promise(resolve=>{
  const input=el('input',{id:'pairing-code',type:'text',maxLength:12,autoComplete:'off',spellcheck:false,placeholder:'12-character code'});
  const status=el('p',{'aria-live':'polite',class:'muted'});
  const submit=button('Pair with this runtime',async()=>{
   submit.disabled=true;status.textContent='Connectingâ€¦';status.removeAttribute('role');
   try{
    const response=await fetch('/v1/pair',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({code:input.value.trim()}),credentials:'omit',cache:'no-store',redirect:'error'});
    if(!response.ok)throw Error('PAIRING_DENIED');const data=await response.json();
    if(typeof data.token!=='string')throw Error('INVALID_PAIRING');
    const client=await connect(window.location.origin,data.token);input.value='';resolve(client);
   }catch{status.setAttribute('role','alert');status.textContent='Pairing failed. Check the single-use code in your local terminal, or restart the host for a new code.';submit.disabled=false;}
  },{class:'primary'});
  input.addEventListener('keydown',e=>{if(e.key==='Enter'&&!submit.disabled){e.preventDefault();submit.click();}});
  document.querySelector('#app').replaceChildren(el('main',{class:'pairing-screen'},el('section',{class:'card pairing-card'},el('span',{class:'eyebrow'},'LOCAL AI KIT'),el('h1',{},'Connect your local runtime'),el('p',{},'Enter the pairing code displayed by the native helper. Nothing is scanned until you request it.'),el('label',{for:'pairing-code'},'Pairing code'),input,submit,status,el('p',{class:'fine-print'},'Access is limited to the permissions granted when starting this runtime. Your provider credentials stay with the native client.'))));input.focus();
 });
}
async function start(){
 const response=await fetch('/runtime/info',{credentials:'omit',cache:'no-store',redirect:'error'});
 if(response.ok){const info=await response.json();if(info.mode!=='live'||info.pairingRequired!==true)throw Error('INVALID_RUNTIME');const client=await pairing();await mountDemo({connectLive:connect,initialClient:client,initialMode:'live'});}
 else if(response.status===404){await mountDemo({connectFixture:fixture,connectLive:connect});}
 else throw Error('RUNTIME_UNAVAILABLE');
}
start().catch(()=>{const status=document.createElement('p');status.textContent='The runtime could not start. No example account data has been substituted.';status.setAttribute('role','alert');document.querySelector('#app').replaceChildren(status);});
