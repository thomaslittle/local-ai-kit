import {createHttpTransport,createLocalAI,getPairingCode,issuePairingCode} from '/modules/sdk/index.js';
import {mountDemo} from './app.mjs';
import {el,button} from './ui.mjs';
async function connect(endpoint,token){return createLocalAI({clientName:'local-ai-kit-showcase',transport:createHttpTransport({baseUrl:endpoint,token})});}
function pairing(){
 return new Promise(resolve=>{
  const input=el('input',{id:'pairing-code',type:'text',maxLength:12,autoComplete:'off',spellcheck:false,placeholder:'12-character code'});
  const status=el('p',{'aria-live':'polite',class:'muted'});
  const showCode=button('Show current pairing code',async()=>{
   showCode.disabled=true;status.textContent='Getting the current code…';status.removeAttribute('role');
   try{
    let pairing;let issued=false;
    try{pairing=await getPairingCode({baseUrl:window.location.origin});}
    catch(error){if(error?.code!=='HTTP_403')throw error;pairing=await issuePairingCode({baseUrl:window.location.origin});issued=true;}
    input.value=pairing.code;status.textContent=`${issued?'New code issued. ':''}Code expires in about ${pairing.expiresInSeconds} seconds.`;input.focus();input.select();
   }catch(error){
    status.setAttribute('role','alert');
    status.textContent='Could not get a pairing code from this local runtime. Check that the helper is running and reload the page.';
   }finally{showCode.disabled=false;}
  });
  const submit=button('Pair with this runtime',async()=>{
   submit.disabled=true;status.textContent='Connecting…';status.removeAttribute('role');
   try{
    const response=await fetch('/v1/pair',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({code:input.value.trim()}),credentials:'omit',cache:'no-store',redirect:'error'});
    if(!response.ok)throw Error('PAIRING_DENIED');const data=await response.json();
    if(typeof data.token!=='string')throw Error('INVALID_PAIRING');
    const client=await connect(window.location.origin,data.token);input.value='';resolve(client);
   }catch{status.setAttribute('role','alert');status.textContent='Pairing failed. Get a fresh code and try again.';submit.disabled=false;}
  },{class:'primary'});
  input.addEventListener('keydown',e=>{if(e.key==='Enter'&&!submit.disabled){e.preventDefault();submit.click();}});
  document.querySelector('#app').replaceChildren(el('main',{class:'pairing-screen'},el('section',{class:'card pairing-card'},el('span',{class:'eyebrow'},'LOCAL AI KIT'),el('h1',{},'Connect your local runtime'),el('p',{},'Connect to the local helper to inspect available providers and models. Pairing checks metadata only; model turns still require your action.'),el('label',{for:'pairing-code'},'Pairing code'),input,el('div',{class:'pairing-actions'},showCode,submit),status,el('p',{class:'fine-print'},'Access is limited to the permissions granted when starting this runtime. Your provider credentials stay with the native client.'))));input.focus();
 });
}
async function start(){
 const response=await fetch('/runtime/info',{credentials:'omit',cache:'no-store',redirect:'error'});
 if(response.ok){const info=await response.json();if(info.mode!=='live'||info.pairingRequired!==true)throw Error('INVALID_RUNTIME');const client=await pairing();await mountDemo({connectLive:connect,initialClient:client,initialMode:'live'});}
 else throw Error('RUNTIME_UNAVAILABLE');
}
start().catch(()=>{const status=document.createElement('p');status.textContent='The native runtime is unavailable. Start the Local AI Kit demo command and reload this page. No example data has been substituted.';status.setAttribute('role','alert');document.querySelector('#app').replaceChildren(status);});
