import {el,button,badge,code,known,checkbox,empty,section,titleCase} from './ui.mjs';
import {configuration} from './configuration.mjs';
import {variants,renderSelector} from './selectors.mjs';

export function selectorsPage(ctx){
 const {state}=ctx;
 const search=el('input',{type:'search',id:'catalog-query','data-focus':'catalog-query',placeholder:'Search models, providers, accounts…','aria-label':'Search models',value:state.query,onInput:e=>{state.query=e.target.value;ctx.render();}});
 return el('div',{},el('div',{class:'variant-tabs','aria-label':'Selector variants'},variants.map(([id,label,description],i)=>button(el('span',{},el('small',{},String(i+1).padStart(2,'0')),label),()=>{state.variant=id;ctx.render();},{'data-variant':id,class:'variant-tab'+(state.variant===id?' active':''),'aria-pressed':String(state.variant===id),title:description}))),
  el('div',{class:'catalog-toolbar'},el('div',{class:'search-wrap'},el('span',{'aria-hidden':'true'},'⌕'),search),checkbox('Only eligible routes',state.onlyEligible,on=>{state.onlyEligible=on;ctx.render();}),checkbox('Include metered routes',state.allowPaid,on=>{state.allowPaid=on;ctx.render();})),
  el('div',{class:'selector-grid'},renderSelector(ctx),configuration(ctx)));
}
export function overviewPage(ctx){
 const {snapshot,mode}=ctx.state;
 return el('div',{class:'stack'},el('div',{class:'metrics'},[[snapshot?.connections.length??0,'Connections'],[snapshot?.models.length??0,'Route-scoped models'],[snapshot?.quotaPools.length??0,'Quota pools'],[ctx.all.filter(c=>c.eligible).length,'Eligible selections']].map(([value,label])=>el('div',{class:'metric'},el('span',{class:'metric-value'},value),el('span',{},label),el('small',{},mode==='synthetic'?'Example data':'Runtime-reported')))),
  section('One catalog. Every application.','Inspect first, select explicitly, execute only with permission.',el('div',{class:'steps'},['Discover connections','Inspect the evidence','Select a route','Run with permission'].map((s,i)=>el('div',{},badge(String(i+1),'accent'),el('h3',{},s))))),
  section('Implementation preview','This showcase demonstrates the SDK and selection layer—using the same public interface for native and synthetic sources.',el('p',{},'Native mode uses the Rust discovery, account metadata, and permissioned session service. Synthetic mode remains a separate, clearly labeled demonstration and never replaces failed live data.'),button('Explore model selectors',()=>{ctx.state.page='Selector lab';ctx.render();},{class:'primary'})),
  section('Evidence, not assumptions','A present credential file does not prove access.',el('div',{class:'example-grid'},ctx.state.snapshot?.connections.slice(0,4).map(c=>el('div',{class:'mini-card'},el('h3',{},c.displayName),badge(known(c.auth)??'Unknown',known(c.auth)==='expired'?'warning':'muted'),el('p',{},known(c.plan)?.label??'Plan not reported'))))));
}
export function connectionsPage(ctx){
 return el('div',{class:'connections-grid'},ctx.connections.map(c=>section(c.displayName,ctx.providers.find(p=>p.id===c.providerId)?.name??c.providerId,
  el('div',{class:'config-badges'},badge(known(c.auth)??'Unknown',known(c.auth)==='authenticated'?'supported':'warning'),badge(c.auth.freshness??'unavailable')),
  el('dl',{class:'facts'},el('dt',{},'Plan'),el('dd',{},known(c.plan)?.label??'Not reported'),el('dt',{},'Profile'),el('dd',{class:'mono'},c.profileId),el('dt',{},'Account'),el('dd',{class:'mono'},c.accountId??'Unverified'),el('dt',{},'Installation'),el('dd',{class:'mono'},c.installationId)),
  el('details',{},el('summary',{},'Evidence and freshness'),code({auth:c.auth,plan:c.plan})),ctx.state.mode==='live'?button(ctx.state.probing===c.id?'Checkingâ€¦':'Check account and models',()=>ctx.probeConnection(c.id),{disabled:!!ctx.state.probing}):null,button('Inspect models',()=>{ctx.state.page='Selector lab';ctx.state.variant='rail';ctx.state.provider=c.providerId;ctx.state.query=c.displayName;ctx.render();}))));
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
 const controls=section('Session playground','Exercise the public SDK, option validation, event stream, approvals, and cancellation.',
  el('div',{class:'notice'},live?'A live turn uses your selected account allowance. The host controls allowed workspaces and permissions; this demo requests read-only execution.':'Simulation only. The fixture service returns predetermined text; it does not call an AI provider.'),
  el('label',{class:'field'},el('span',{},'Example request'),el('textarea',{rows:4,'aria-label':'Example request',value:state.prompt,disabled:busy||!executable,onInput:e=>{state.prompt=e.target.value;}})),
  checkbox(live?'Allow this live request to use the selected account':'Allow this simulated session',state.grantConsent,on=>{state.grantConsent=on;ctx.render();}),
  live?el('label',{class:'field'},'Host workspace name',el('input',{'aria-label':'Host workspace name',value:state.workspaceId,onInput:e=>{state.workspaceId=e.target.value;}})):checkbox('Simulate a tool approval',state.simulateApproval,on=>{state.simulateApproval=on;ctx.render();}),
  el('div',{class:'actions'},
   button(live?'Run live turn':'Simulate turn',ctx.startTurn,{class:'primary',disabled:busy||!executable||!state.grantConsent||!state.selection}),
   button('Cancel turn',ctx.cancelTurn,{disabled:!busy||!state.turnId}),badge(state.sessionStatus,busy?'accent':'muted')));
 const approval=state.approval?el('div',{class:'approval'},el('h3',{},state.approval.action),el('p',{},state.approval.detail),
  el('div',{class:'actions'},button('Approve',()=>ctx.answerApproval(true),{class:'primary'}),button('Deny',()=>ctx.answerApproval(false),{class:'danger'}))):null;
 const output=section('Stream output',null,
  el('div',{id:'session-status',class:'session-status',role:'status','aria-live':'polite'},state.sessionStatus),
  el('pre',{id:'response-text',class:'response'},state.output||'Output appears only after you explicitly start a turn.'),approval);
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
 if(!ctx.state.snapshot)return empty('No connected catalog',ctx.state.connectionError??'Connect an authorized local runtime or explicitly open the synthetic showcase.');
 switch(ctx.state.page){
  case 'Overview':return overviewPage(ctx);
  case 'Connections':return connectionsPage(ctx);
  case 'Usage':return usagePage(ctx);
  case 'Playground':return playgroundPage(ctx);
  case 'Diagnostics':return diagnosticsPage(ctx);
  default:return selectorsPage(ctx);
 }
}
