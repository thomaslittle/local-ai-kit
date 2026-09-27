import {el,button,badge,code,known,checkbox,empty,section,titleCase} from './ui.mjs';
import {configuration} from './configuration.mjs';
import {connectionAuthLabel} from './profile_auth.mjs';
import {isLocalRuntimeProvider} from './catalog_visibility.mjs';
import {variants,renderSelector} from './selectors.mjs';
const compactIdentity=value=>{const text=String(value);return text.length>28?`${text.slice(0,25)}…`:text;};
const authenticated=c=>known(c.auth)==='authenticated'&&c.auth.freshness==='fresh';
const previouslyAuthenticated=c=>known(c.auth)==='authenticated'&&c.auth.freshness==='stale';

export function selectorsPage(ctx){
 const {state}=ctx;
 const search=el('input',{type:'search',id:'catalog-query','data-focus':'catalog-query',placeholder:'Search models, providers, accounts…','aria-label':'Search models',value:state.query,onInput:e=>{state.query=e.target.value;ctx.render();}});
 return el('div',{},el('div',{class:'variant-tabs','aria-label':'Selector variants'},variants.map(([id,label,description],i)=>button(el('span',{},el('small',{},String(i+1).padStart(2,'0')),label),()=>{state.variant=id;ctx.render();},{'data-variant':id,class:'variant-tab'+(state.variant===id?' active':''),'aria-pressed':String(state.variant===id),title:description}))),
  el('div',{class:'catalog-toolbar'},el('div',{class:'search-wrap'},el('span',{'aria-hidden':'true'},'⌕'),search),checkbox('Only eligible routes',state.onlyEligible,on=>{state.onlyEligible=on;ctx.render();}),checkbox('Include metered routes',state.allowPaid,on=>{state.allowPaid=on;ctx.render();})),
  el('div',{class:'selector-grid'},renderSelector(ctx),configuration(ctx)));
}
export function overviewPage(ctx){
 const {snapshot,mode}=ctx.state;
 const connections=snapshot?.connections??[],verified=connections.filter(authenticated),previous=connections.filter(previouslyAuthenticated);
 const metrics=el('div',{class:'metrics'},[[connections.length,'Detected profiles'],[verified.length,'Verified profiles'],[snapshot?.models.length??0,'Route-scoped models'],[ctx.all.filter(c=>c.eligible).length,'Eligible selections']].map(([value,label])=>el('div',{class:'metric'},el('span',{class:'metric-value'},value),el('span',{},label),el('small',{},mode==='synthetic'?'Example data':'Runtime-reported'))));
 const verifiedSection=verified.length?section('Verified profiles','Only fresh authenticated profiles appear here.',el('div',{class:'example-grid'},verified.slice(0,4).map(c=>el('div',{class:'mini-card'},el('h3',{},c.displayName),badge('Authenticated','supported'),el('p',{},known(c.plan)?.label??'Plan not reported'))))):previous.length?null:section('Verified profiles','Only fresh authenticated profiles appear here.',empty('No authenticated profiles yet','Detected clients remain unverified until an explicit account and model check succeeds.'));
 const previousSection=previous.length?section('Previously authenticated','These profiles passed an earlier check, but that authentication evidence is stale.',el('div',{class:'example-grid','data-testid':'previously-authenticated-overview'},previous.slice(0,4).map(c=>el('div',{class:'mini-card'},el('h3',{},c.displayName),badge('Previously authenticated','warning'),el('p',{},`${ctx.providers.find(p=>p.id===c.providerId)?.name??c.providerId} · Check is stale`),el('p',{},known(c.plan)?.label??'Plan not reported'))))):null;
 return el('div',{class:'stack'},metrics,
  section('One catalog. Every application.','Inspect first, select explicitly, execute only with permission.',el('div',{class:'steps'},['Discover connections','Inspect the evidence','Select a route','Run with permission'].map((s,i)=>el('div',{},badge(String(i+1),'accent'),el('h3',{},s))))),
  section('Live showroom','This page uses the public SDK against the native local service.',el('p',{},'Passive discovery reports installed clients. Account access, model catalogs, and usage remain tied to evidence from authorized probes. Empty results stay empty; no example records are inserted.'),button('Explore model selectors',()=>{ctx.state.page='Selector lab';ctx.render();},{class:'primary'})),verifiedSection,previousSection);
}
export function connectionsPage(ctx){
 const localCatalogAvailable=c=>isLocalRuntimeProvider(c)&&(ctx.state.snapshot?.routes??[]).some(route=>
  route.connectionId===c.id&&known(route.billing)==='local'&&route.billing.freshness==='fresh'
   &&ctx.state.snapshot.models.some(model=>model.routeId===route.id));
 const locallyUnauthenticated=c=>known(c.auth)==='unauthenticated'&&c.auth.freshness==='fresh';
 const connectionCard=c=>{
  const provider=ctx.providers.find(p=>p.id===c.providerId)?.name??c.providerId;
  const status=connectionAuthLabel(c);
  const local=isLocalRuntimeProvider(c),catalogReady=localCatalogAvailable(c);
  const card=section(c.displayName,provider,
   el('div',{class:'config-badges'},badge(status,authenticated(c)?'supported':local?catalogReady?'supported':'muted':'warning'),badge(local?catalogReady?'Local catalog ready':'Catalog not reported':c.auth.freshness??'unavailable',catalogReady?'supported':'muted')),
   el('dl',{class:'facts'},el('dt',{},'Plan'),el('dd',{},known(c.plan)?.label??'Not reported'),el('dt',{},'Profile'),el('dd',{class:'mono identity-value',title:c.profileId,'aria-label':`Profile identifier ${c.profileId}`},compactIdentity(c.profileId)),el('dt',{},'Account'),el('dd',{class:'mono identity-value',title:c.accountId??'Unverified','aria-label':`Account identifier ${c.accountId??'Unverified'}`},compactIdentity(c.accountId??'Unverified')),el('dt',{},'Installation'),el('dd',{class:'mono identity-value',title:c.installationId,'aria-label':`Installation identifier ${c.installationId}`},compactIdentity(c.installationId))),
   ctx.state.probeResults?.[c.id]?el('p',{class:`metadata-result ${ctx.state.probeResults[c.id].ok?'success':'warning'}`,role:'status','aria-live':'polite'},ctx.state.probeResults[c.id].message):null,
   el('details',{},el('summary',{},'Evidence and freshness'),code({auth:c.auth,plan:c.plan})),ctx.state.mode==='live'?button(ctx.state.probing===c.id?'Checking…':local?'Refresh local catalog':'Check account and models',()=>ctx.probeConnection(c.id),{disabled:!!ctx.state.probing||!!ctx.state.metadataBatch}):null,button('Inspect models',()=>{ctx.state.page='Selector lab';ctx.state.variant='rail';ctx.state.provider=c.providerId;ctx.state.query=c.displayName;ctx.render();}));
  card.classList.add('profile-card');card.dataset.connectionId=c.id;return card;
 };
 const cards=connections=>el('div',{class:'connections-grid'},connections.map(connectionCard));
 const verified=ctx.connections.filter(authenticated),previous=ctx.connections.filter(previouslyAuthenticated),local=ctx.connections.filter(isLocalRuntimeProvider),unauthenticated=ctx.connections.filter(locallyUnauthenticated),detected=ctx.connections.filter(c=>!authenticated(c)&&!previouslyAuthenticated(c)&&!isLocalRuntimeProvider(c)&&!locallyUnauthenticated(c));
 const uncheckedDetected=detected.filter(c=>!ctx.state.probeResults?.[c.id]);
 const verifiedSection=verified.length?section('Authenticated profiles','Only fresh provider-confirmed authentication appears as a connected profile.',el('div',{class:'connections-grid','data-testid':'verified-profiles'},verified.map(connectionCard))):previous.length?null:section('Authenticated profiles','Only fresh provider-confirmed authentication appears as a connected profile.',empty('No authenticated profiles yet','Run the metadata-only check below. No model prompts are sent.'));
 const batch=ctx.state.metadataBatch;
 const metadataControls=ctx.state.mode==='live'?section('Profile metadata','Metadata-only: check every detected account and local catalog in sequence. No model prompts are sent.',batch?el('p',{role:'status','aria-live':'polite'},`${batch.completed} of ${batch.total} profiles checked`):null,button(batch?`Checking profile ${Math.min(batch.completed+1,batch.total)} of ${batch.total}…`:Object.keys(ctx.state.probeResults??{}).length?'Recheck all profiles':'Check all detected profiles',()=>ctx.probeAllConnections?.(),{class:'primary','data-testid':'check-all-profiles',disabled:!!ctx.state.probing||!!batch})):null;
 return el('div',{class:'stack'},
  metadataControls,
  verifiedSection,
  previous.length?section('Previously authenticated','These profiles passed an earlier check, but that evidence is stale. Check account and models again to confirm their current status.',el('div',{class:'connections-grid','data-testid':'previously-authenticated-profiles'},previous.map(connectionCard))):null,
  local.length?section('Local runtimes','Local model servers do not have account authentication. Their routes appear when the server reports a real model catalog.',el('div',{class:'connections-grid','data-testid':'local-runtimes'},local.map(connectionCard))):null,
  unauthenticated.length?section('Not authenticated','These explicit checks did not confirm a signed-in account. Their models are excluded; check again after signing in.',el('div',{class:'connections-grid','data-testid':'unauthenticated-profiles'},unauthenticated.map(connectionCard))):null,
  el('details',{class:'detected-profiles','data-testid':'detected-profiles',...(ctx.state.detectedExpanded??true?{open:true}:{}),onToggle:event=>{ctx.state.detectedExpanded=event.currentTarget.open;}},el('summary',{},uncheckedDetected.length?`Other detected clients · ${uncheckedDetected.length} need a metadata check`:`Other detected clients · ${detected.length} checked, not confirmed`),el('p',{class:'muted'},uncheckedDetected.length?'Installation alone does not prove an account is signed in. Check profiles to request supported metadata; unsupported sources remain unverified.':'These profiles have a metadata result but are not confirmed as authenticated. Each result explains whether authentication was absent or the provider could not be checked.'),detected.length?cards(detected):el('p',{class:'muted'},'Every remaining profile has a current result above.')));
}
export function usagePage(ctx){
 return el('div',{class:'stack'},el('p',{class:'intro'},'Windows belong to quota pools, not model rows. Shared pools appear once; unknown totals stay unknown.'),
  el('div',{class:'usage-grid'},(ctx.state.snapshot?.quotaPools??[]).map(pool=>el('section',{class:'card','data-pool':pool.id},el('div',{class:'section-heading'},el('h2',{},pool.windows[0]?.label??pool.id),badge(pool.sharing==='verified'?'Verified sharing':'Sharing unknown',pool.sharing==='verified'?'accent':'muted')),
   pool.windows.map(window=>{const reading=known(window.reading);return el('div',{class:'usage-window'},reading?el('div',{},el('div',{class:'usage-number'},reading.used,el('span',{},` ${reading.unit}${reading.limit===null?'':` / ${reading.limit}`}`)),reading.limit!==null&&typeof reading.usedFraction==='number'?el('progress',{value:Math.max(0,Math.min(1,reading.usedFraction)),max:1,'aria-label':`${window.label} used fraction`}):el('p',{class:'muted'},'No published total · no percentage implied'),el('div',{class:'usage-foot'},badge(window.reading.freshness??'unknown',window.reading.freshness==='stale'?'warning':'muted'),el('span',{},window.resetsAt?`Resets ${new Date(window.resetsAt).toLocaleTimeString()}`:'Reset not reported'))):empty('Usage unavailable',window.reading.reason));}),
   el('p',{class:'fine-print'},`${pool.routeIds.length} linked route${pool.routeIds.length===1?'':'s'}. ${ctx.state.mode==='synthetic'?'Illustrative allowance only.':''}`)))));
}
export function playgroundPage(ctx){
 const {state}=ctx;
 const live=state.mode!=='synthetic';
 const executable=state.client?.handshake.methods.includes('turns.start');
 const busy=['Running','Awaiting approval','Starting'].includes(state.sessionStatus);
 const controls=section('Session playground','Review the live session controls. Nothing runs until you select a route and authorize a turn.',
  el('div',{class:'notice'},live?'A live turn uses your selected account allowance. The host controls allowed workspaces and permissions; this demo requests read-only execution.':'Simulation only. The fixture service returns predetermined text; it does not call an AI provider.'),
  live&&!executable?el('p',{class:'notice'},'This runtime allows metadata checks only. Enable execution and approve a workspace in the host application before sending a prompt.'):null,
  el('label',{class:'field'},el('span',{},'Prompt'),el('textarea',{rows:4,'aria-label':'Prompt',value:state.prompt,disabled:busy||!executable,onInput:e=>{state.prompt=e.target.value;}})),
  checkbox(live?'Allow this live request to use the selected account':'Allow this simulated session',state.grantConsent,on=>{state.grantConsent=on;ctx.render();}),
  live?el('label',{class:'field'},'Host workspace name',el('input',{'aria-label':'Host workspace name',value:state.workspaceId,onInput:e=>{state.workspaceId=e.target.value;}})):checkbox('Simulate a tool approval',state.simulateApproval,on=>{state.simulateApproval=on;ctx.render();}),
  el('div',{class:'actions'},
   button(live?'Run live turn':'Simulate turn',ctx.startTurn,{class:'primary',disabled:busy||!executable||!state.grantConsent||!state.selection}),
   button('Cancel turn',ctx.cancelTurn,{disabled:!busy||!state.turnId}),
   state.client?.handshake.methods.includes('sessions.suspend')?button('Suspend native session',ctx.suspendSession,{disabled:busy||!state.sessionId||!state.nativeResumption}):null,
   state.client?.handshake.methods.includes('sessions.resume')?button('Resume native session',ctx.resumeSession,{disabled:busy||!state.resumeToken}):null,
   badge(state.sessionStatus,busy?'accent':'muted')));
 const approval=state.approval?el('div',{class:'approval'},el('h3',{},state.approval.action),el('p',{},state.approval.detail),
  el('div',{class:'actions'},button('Approve',()=>ctx.answerApproval(true),{class:'primary'}),button('Deny',()=>ctx.answerApproval(false),{class:'danger'}))):null;
 const output=section('Stream output',null,
  el('div',{id:'session-status',class:'session-status',role:'status','aria-live':'polite'},state.sessionStatus),
  el('pre',{id:'response-text',class:'response'},state.output||'Output appears only after you explicitly start a turn.'),approval,
  el('div',{class:'actions'},state.events.filter(e=>e.sessionId===state.sessionId&&e.payload.type==='artifact').map(e=>button('Download result ('+e.payload.size+' bytes)',()=>ctx.downloadArtifact(e),{disabled:!state.client?.handshake.methods.includes('artifacts.read')}))));
 const timeline=section('Event timeline',`${state.events.length} recent events; bounded to 100.`,
  el('div',{class:'timeline'},state.events.slice(-12).reverse().map(e=>el('div',{class:'event-row'},
   el('span',{class:'mono'},e.sequence),badge(e.payload.type),el('small',{class:'mono'},e.sessionId)))));
 return el('div',{class:'selector-grid playground-grid'},el('div',{class:'stack'},controls,output,timeline),configuration(ctx));
}
export function diagnosticsPage(ctx){
 return el('div',{class:'stack'},section('SDK / protocol inspector','Only normalized public responses are shown. No credentials are displayed.',
  button('Refresh diagnostics',ctx.refreshDiagnostics),ctx.state.diagnostics?code(ctx.state.diagnostics):el('p',{class:'muted'},'Diagnostics have not been requested.'),
  el('details',{},el('summary',{},'Negotiated public methods'),code(ctx.state.client?.handshake??{}))),
  section('Integration recipe','Use the same selection with a compatible runtime. Token values are never included.',
  el('pre',{class:'code'},`import { createLocalAI, createHttpTransport } from "@local-ai-kit/sdk";\n\nconst ai = await createLocalAI({\n  transport: createHttpTransport({\n    baseUrl: "http://127.0.0.1:4499",\n    token: appSessionToken, // obtained through explicit local pairing\n  }),\n});\n\nconst selection = ${JSON.stringify(ctx.state.selection,null,2)};\n// Request grants and validate before starting a session.\n// No turn is started merely by creating or selecting a client.`)),
  section('Known preview boundaries','These are implementation limitations, not hidden successful fallbacks.',el('p',{},'This showcase consumes the public SDK. Native discovery, model/account metadata and execution are reported per adapter. Inspect the negotiated methods and repository support matrix; unavailable features are not silently substituted.')));
}
export function renderPage(ctx){
 if(!ctx.state.snapshot)return empty('No connected catalog',ctx.state.connectionError??'Pair this browser with the running local helper to load its discovery snapshot.');
 switch(ctx.state.page){
  case 'Overview':return overviewPage(ctx);
  case 'Connections':return connectionsPage(ctx);
  case 'Usage':return usagePage(ctx);
  case 'Playground':return playgroundPage(ctx);
  case 'Diagnostics':return diagnosticsPage(ctx);
  default:return selectorsPage(ctx);
 }
}
