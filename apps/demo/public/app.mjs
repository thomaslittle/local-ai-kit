import {createCatalog,resolveCandidates,reconcileSelection} from '/modules/selector/index.js';
import {readArtifactBytes,readCatalogSnapshot} from '/modules/sdk/index.js';
import {el,button,badge} from './ui.mjs';
import {renderPage} from './pages.mjs';
import {openPalette,openConnectionDialog} from './dialogs.mjs';
const pages=[['Overview','◎'],['Connections','⌘'],['Models','◈'],['Usage','◴'],['Selector lab','▥'],['Playground','▷'],['Diagnostics','⌁']];
const descriptions={
 'Overview':'Discover the shape of your AI access.',
 'Connections':'Every account, profile, and access route stays distinct.',
 'Models':'A route-aware catalog. Native options, never guessed settings.',
 'Usage':'The right units. The right windows. No fictional totals.',
 'Selector lab':'One selection engine. Eight ways to make it yours.',
 'Playground':'Inspect the complete flow before a real provider is involved.',
 'Diagnostics':'Understand the evidence behind every public response.',
};

/** UI host. Consumes public SDK clients only; no account or provider logic is embedded. */
export async function mountDemo({connectFixture,connectLive,initialClient=null,initialMode='synthetic'}){
 const root=document.querySelector('#app')??document.body;
 const state={page:'Selector lab',variant:'rail',mode:initialMode,snapshot:null,selection:null,provider:'codex',query:'',required:new Set(),onlyEligible:false,allowPaid:false,favorites:new Set(),sensitiveConsent:false,
  mobileStep:0,mobileProvider:'codex',mobileConnection:null,client:null,events:[],sessionId:null,nativeResumption:false,resumeToken:null,turnId:null,sessionStatus:'Ready',output:'',approval:null,grantConsent:false,simulateApproval:false,workspaceId:'default',probing:null,prompt:'Explain how an application can select an existing AI connection.',notice:'',connectionError:null,diagnostics:null};
 let catalog=null,all=[],epoch=0,eventController=null,closed=false;
 const ctx={state,get catalog(){return catalog;},get all(){return all;},get candidates(){return catalog?resolveCandidates(catalog,{query:state.query,requiredCapabilities:[...state.required],eligibleOnly:state.onlyEligible,allowPaidFallback:state.allowPaid}):[];},get providers(){return state.snapshot?.providers??[];},get connections(){return state.snapshot?.connections??[];},render,select,startTurn,cancelTurn,suspendSession,resumeSession,answerApproval,refreshDiagnostics,scan,probeConnection,revokeConnection,downloadArtifact,connectLive:attachLive,openPalette:()=>openPalette(ctx)};
 function rebuild(snapshot){
  if(!snapshot||!Array.isArray(snapshot.connections)||!Array.isArray(snapshot.models)||!Array.isArray(snapshot.routes)||!Array.isArray(snapshot.providers))throw Error('INVALID_SNAPSHOT');
  if(state.mode==='live'&&snapshot.mode!=='live')throw Error('SYNTHETIC_RUNTIME_REJECTED');
  if(state.mode==='synthetic'&&snapshot.mode!=='synthetic')throw Error('LIVE_RUNTIME_REJECTED');
  catalog=createCatalog(snapshot);state.snapshot=snapshot;
  all=resolveCandidates(catalog,{allowPaidFallback:state.allowPaid});
  if(state.selection){const reconciled=reconcileSelection(catalog,state.selection);state.selection=reconciled.selection;if(reconciled.needsReview)state.notice='The catalog changed. Review the updated selection before use.';}
  if(!state.selection){const first=all.find(c=>c.eligible);if(first)state.selection={routeId:first.route.id,modelId:first.model.id,revision:first.model.revision,options:{}};}
 }
 function select(candidate){
  state.selection={routeId:candidate.route.id,modelId:candidate.model.id,revision:candidate.model.revision,options:{}};
  state.sensitiveConsent=false;state.notice='';render();
 }
 function safeError(error){return typeof error?.code==='string'&&/^[A-Z_]{1,64}$/.test(error.code)?error.code:'REQUEST_FAILED';}
 async function operate(action){try{await action();}catch(error){state.notice=safeError(error);render();}}
 async function stopClient(){eventController?.abort();eventController=null;const client=state.client;state.client=null;if(client)await client.close();}
 async function observe(client,currentEpoch){
  const controller=new AbortController();eventController=controller;
  try{
   for await(const event of client.events.subscribe({after:'0'},{signal:controller.signal})){
    if(closed||epoch!==currentEpoch)break;
    if(!event?.payload||typeof event.payload.type!=='string')continue;
    state.events.push(event);if(state.events.length>100)state.events.shift();
    if(event.sessionId===state.sessionId){
     const payload=event.payload;
     if(payload.type==='text_delta'&&typeof payload.text==='string')state.output=(state.output+payload.text).slice(-65536);
     if(payload.type==='approval_request'){state.approval=payload;state.sessionStatus='Awaiting approval';}
     if(payload.type==='approval_resolved')state.approval=null;
     if(payload.type==='final'){state.sessionStatus=payload.status==='cancelled'?'Cancelled':payload.status==='denied'?'Denied':payload.status==='completed'?'Completed':'Error';state.approval=null;}
     if(payload.type==='error'){state.sessionStatus='Error';state.notice='The session reported an error.';}
    }
    render();
   }
  }catch(error){if(!closed&&epoch===currentEpoch&&!controller.signal.aborted){state.notice=safeError(error);render();}}
 }
 async function attach(factory,mode){
  const current=++epoch;await stopClient();
  state.mode=mode;state.snapshot=null;catalog=null;all=[];state.selection=null;state.connectionError=null;state.events=[];state.sessionId=null;state.nativeResumption=false;state.resumeToken=null;state.turnId=null;state.sessionStatus='Ready';state.output='';state.approval=null;state.diagnostics=null;state.notice='';state.favorites.clear();state.grantConsent=false;render();
  let client;
  try{
   client=await factory();if(closed||epoch!==current){await client.close();return;}
   const snapshot=await readCatalogSnapshot(client);if(closed||epoch!==current){await client.close();return;}
   rebuild(snapshot);state.client=client;render();
   if(client.handshake.methods.includes('events.subscribe'))void observe(client,current);
  }catch(error){await client?.close().catch(()=>{});if(epoch===current){state.connectionError='Unable to connect. No account or model data has been substituted.';render();}}
 }
 async function attachLive(endpoint,token){return attach(()=>connectLive(endpoint,token),'live');}
 async function startTurn(){
  if(!state.grantConsent||!state.selection||!state.client||['Starting','Running','Awaiting approval'].includes(state.sessionStatus))return;
  const client=state.client,current=epoch,selection=structuredClone(state.selection);state.sessionStatus='Starting';state.output='';state.approval=null;state.notice='';render();
  await operate(async()=>{
   const model=all.find(c=>c.model.id===selection.modelId);if(!model)throw Error('SELECTION_STALE');
   if(state.sessionId)await client.sessions.close({sessionId:state.sessionId});
   if(state.mode==='synthetic')await client.grants.request({connectionIds:[model.connection.id],scopes:['execution'],allowPaid:state.allowPaid});
   const session=await client.sessions.create({selection,allowPaid:state.allowPaid,...(state.mode==='live'?{workspaceId:state.workspaceId,permissionMode:'read_only'}:{})});
   if(epoch!==current)return;state.sessionId=session.id;state.nativeResumption=Boolean(session.nativeResumption);state.resumeToken=null;state.turnId=null;state.sessionStatus='Running';render();
   const turn=await client.turns.start({sessionId:session.id,prompt:state.prompt,...(state.mode==='synthetic'?{simulateApproval:state.simulateApproval}:{requestId:crypto.randomUUID()})});
   if(epoch!==current)return;state.turnId=turn.turnId;render();
  });
  if(state.notice&&epoch===current){state.sessionStatus='Error';render();}
 }
 async function downloadArtifact(event){
  if(!state.client||event.sessionId!==state.sessionId||!state.client.handshake.methods.includes('artifacts.read'))return;
  const client=state.client,current=epoch;
  await operate(async()=>{
   const result=await readArtifactBytes(client,event.sessionId,event.payload.artifactId);
   if(closed||epoch!==current)return;
   // Always download as inert bytes; never navigate to or render a provider's executable content.
   const url=URL.createObjectURL(new Blob([result.bytes],{type:'application/octet-stream'}));
   const anchor=document.createElement('a');anchor.href=url;anchor.download=result.displayName.replace(/[^a-zA-Z0-9_. -]/g,'_').slice(0,128)||'result.bin';anchor.rel='noreferrer';document.body.append(anchor);anchor.click();anchor.remove();setTimeout(()=>URL.revokeObjectURL(url),1000);
  });
 }
 async function cancelTurn(){if(state.turnId&&state.client)await operate(async()=>{await state.client.turns.cancel({turnId:state.turnId});});}
 async function suspendSession(){
  if(!state.client||!state.sessionId||!state.nativeResumption||!state.client.handshake.methods.includes('sessions.suspend'))return;
  await operate(async()=>{const result=await state.client.sessions.suspend({sessionId:state.sessionId});state.resumeToken=result.resumeToken;state.sessionId=null;state.turnId=null;state.sessionStatus='Suspended';state.output='';state.approval=null;render();});
 }
 async function resumeSession(){
  if(!state.client||!state.resumeToken||!state.client.handshake.methods.includes('sessions.resume'))return;
  await operate(async()=>{const session=await state.client.sessions.resume({resumeToken:state.resumeToken});state.sessionId=session.id;state.nativeResumption=Boolean(session.nativeResumption);state.resumeToken=null;state.turnId=null;state.sessionStatus='Ready';state.output='';state.approval=null;render();});
 }
 async function answerApproval(allow){if(state.approval&&state.client)await operate(async()=>{await state.client.approvals.respond({approvalId:state.approval.approvalId,allow});});}
 async function scan(){if(state.client)await operate(async()=>{await state.client.discovery.refresh({level:'passive'});rebuild(await readCatalogSnapshot(state.client));state.page='Connections';render();});}
 async function probeConnection(id){if(!state.client||state.probing)return;state.probing=id;render();await operate(async()=>{await state.client.connections.probe({connectionId:id});rebuild(await readCatalogSnapshot(state.client));});state.probing=null;render();}
 async function revokeConnection(){if(!state.client)return;await operate(async()=>{await state.client.grants.revoke({});await stopClient();catalog=null;all=[];state.snapshot=null;state.selection=null;state.notice='Connection revoked. Restart the native host and reload to pair again.';render();});}
 async function refreshDiagnostics(){if(state.client)await operate(async()=>{state.diagnostics={report:await state.client.diagnostics.report(),adapters:await state.client.adapters.list(),grants:await state.client.grants.list()};render();});}
 function render(){
  if(closed)return;
  if(catalog)all=resolveCandidates(catalog,{allowPaidFallback:state.allowPaid});
  const active=document.activeElement,focus=active?.getAttribute('data-focus');
  let start,end;try{start=active?.selectionStart;end=active?.selectionEnd;}catch{}
  const shell=el('div',{class:'shell'},el('aside',{class:'sidebar'},el('a',{class:'brand',href:'#','aria-label':'Local AI Kit home',onClick:event=>{event.preventDefault();state.page='Overview';render();}},el('span',{class:'brand-mark','aria-hidden':'true'},'◈'),el('span',{},el('strong',{},'Local AI Kit'),el('small',{},'DEVELOPER SHOWCASE'))),
   el('div',{class:'sidebar-label'},'WORKSPACE'),el('nav',{'aria-label':'Showcase sections'},pages.map(([name,glyph])=>button(el('span',{class:'nav-inner'},el('span',{'aria-hidden':'true'},glyph),name),()=>{state.page=name;if(name==='Models')state.variant='compare';render();},{class:'nav-link'+(state.page===name?' active':''),'aria-label':name,'aria-current':state.page===name?'page':'false'}))),
   el('div',{class:'sidebar-bottom'},badge('Implementation preview'),el('p',{},'Build around access. Not assumptions.'),el('a',{href:'https://github.com/thomaslittle/local-ai-kit',target:'_blank',rel:'noreferrer'},'View repository ↗'))),
   el('main',{class:'main'},el('header',{class:'topbar'},el('span',{class:'breadcrumb'},'Workspace',el('span',{},'/'),state.page),el('div',{class:'top-actions'},button('⌘ K',ctx.openPalette,{'aria-label':'Open model command palette',class:'shortcut'}),state.mode==='live'?button('Scan local clients',scan,{class:'connect-button',disabled:!state.client}):button('Connect local runtime',()=>openConnectionDialog(ctx),{class:'connect-button'}))),
    el('div',{id:'mode-banner',class:'mode-banner '+state.mode,role:'status'},el('span',{class:'status-dot','aria-hidden':'true'}),el('strong',{},state.mode==='synthetic'?'Synthetic showcase':'Live connection'),el('span',{},state.mode==='synthetic'?'Example accounts and models. No credentials inspected. No inference used.':'Only native data is shown. Execution requires your explicit action and a host-granted workspace.'),state.mode==='live'?button('Revoke connection',revokeConnection):badge('NO ACCOUNT REQUIRED')),
    el('div',{class:'main-content'},el('div',{class:'page-heading'},el('div',{},el('span',{class:'eyebrow'},state.mode==='synthetic'?'SDK & SELECTION PREVIEW':'CONNECTED WORKSPACE'),el('h1',{id:'page-title'},state.page),el('p',{},descriptions[state.page])),badge(state.snapshot?`${state.snapshot.models.length.toLocaleString()} ${state.mode==='synthetic'?'example ':''}models`:'Not connected')),
     state.connectionError?el('div',{id:'connection-error',class:'notice warning',role:'alert'},state.connectionError):null,
     state.notice?el('div',{class:'notice warning',role:'alert'},state.notice):null,renderPage(ctx),
     el('footer',{class:'page-footer'},el('span',{},'Local-first by design. Honest about what is known.'),el('span',{},'v0.1 · work in progress')))));
  root.replaceChildren(shell);
  if(focus){const replacement=root.querySelector(`[data-focus="${CSS.escape(focus)}"]`);replacement?.focus({preventScroll:true});if(typeof start==='number'&&typeof end==='number')try{replacement?.setSelectionRange(start,end);}catch{}}
 }
 const keydown=event=>{if((event.ctrlKey||event.metaKey)&&event.key.toLowerCase()==='k'&&!document.querySelector('dialog[open]')){event.preventDefault();ctx.openPalette();}};
 document.addEventListener('keydown',keydown);render();await attach(initialClient?()=>Promise.resolve(initialClient):connectFixture,initialMode);
 return {async close(){if(closed)return;closed=true;++epoch;document.removeEventListener('keydown',keydown);await stopClient();}};
}
