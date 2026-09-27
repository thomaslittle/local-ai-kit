import {el,button,badge,checkbox,empty} from './ui.mjs';
export const variants=[['rail','Provider rail','Providers first, models inside'],['compact','Compact','A small composer control'],['palette','Command palette','Keyboard-first global search'],['capability','Capability first','Choose what the task needs'],['compare','Compare','Inspect routes side by side'],['allowance','Allowance aware','Keep billing sources visible'],['preset','Task presets','Requirements, not model hype'],['mobile','Mobile drill-down','One decision at a time']];
const capNames={'generation.text':'Text generation','input.text':'Text input','input.image':'Image input','structured.output':'Structured output','tools':'Tool support','offline':'Offline operation'};
export function reasonLabel(reason){
 const [code,id]=reason.split(':',2),capability=capNames[id]??id;
 if(code==='CAPABILITY_UNKNOWN')return `${capability} is not confirmed`;
 if(code==='CAPABILITY_UNSUPPORTED')return `${capability} is not supported`;
 if(code==='CAPABILITY_CONDITIONAL')return `${capability} has unmet requirements`;
 return ({AUTH_UNVERIFIED:'Authentication has not been checked',AUTH_REQUIRED:'Authentication needs attention',BILLING_UNKNOWN:'Billing classification is not reported',BILLING_CONSENT_REQUIRED:'Metered access needs your consent',RATE_LIMITED:'The provider reports no ordinary usage allowance right now'})[code]??reason;
}
export function modelRow(candidate,ctx){
 const selected=ctx.state.selection?.modelId===candidate.model.id;
 return el('div',{'data-model-row':'',class:'model-row'+(selected?' selected':'')},button(el('div',{class:'model-row-inner'},
  el('div',{class:'model-line'},el('strong',{},candidate.model.name),selected?badge('Selected','accent'):null),
  el('div',{class:'model-meta'},el('span',{},candidate.connection.displayName),el('span',{},candidate.billingLabel)),
  el('div',{class:'capabilities'},candidate.model.capabilities.map(c=>badge(capNames[c.id]??c.id,c.support==='supported'?'supported':'muted'))),
  !candidate.eligible?el('div',{class:'reason'},candidate.reasons.map(reasonLabel).join(' · ')):null),()=>ctx.select(candidate),{
   class:'model-button','data-testid':candidate.model.id,'data-eligible':String(candidate.eligible),'aria-pressed':String(selected),
  }));
}
function list(candidates,ctx){
 if(!candidates.length)return emptyRoutes(ctx);
 return el('div',{class:'model-list'},candidates.slice(0,50).map(c=>modelRow(c,ctx)),candidates.length>50?el('p',{class:'list-footnote'},`Showing 50 of ${candidates.length.toLocaleString()} matches. Narrow the search to inspect the rest.`):null);
}
function emptyRoutes(ctx){
 if(!ctx.all.length)return empty('No routes match the current filters.','Check account and model metadata on an authorized connection to load routes.');
 if(ctx.state.required?.size)return empty('No routes have confirmed all selected capabilities.','Unknown support is not treated as supported.');
 return empty('No routes match the current filters.','Clear filters or broaden the search to see more routes.');
}
function providerRail(ctx){
 const groups=[['all','All connections'],['favorites','Favorites'],...ctx.providers.map(p=>[p.id,p.name])];
 return el('div',{class:'provider-rail','data-testid':'provider-rail'},groups.map(([id,name])=>button(el('span',{class:'provider-content'},el('span',{class:'provider-glyph','aria-hidden':'true'},id==='favorites'?'★':id==='all'?'≋':name.slice(0,1)),el('span',{},name),el('span',{class:'provider-count'},id==='favorites'?ctx.state.favorites.size:id==='all'?ctx.all.length:ctx.all.filter(c=>c.route.providerId===id).length)),()=>{ctx.state.provider=id;ctx.render();},{class:'provider'+(ctx.state.provider===id?' active':''),'data-provider':id,'aria-label':name,'aria-pressed':String(ctx.state.provider===id)})));
}
export function renderSelector(ctx){
 const {state}=ctx;let content;
 const groupFiltered=state.provider==='favorites'?ctx.candidates.filter(c=>state.favorites.has(c.key)):
  state.query.trim()||state.provider==='all'?ctx.candidates:ctx.candidates.filter(c=>c.route.providerId===state.provider);
 if(state.variant==='rail')content=el('div',{class:'rail-layout'},providerRail(ctx),el('div',{class:'rail-models'},el('div',{class:'list-heading'},el('strong',{},state.query?'Search all connections':state.provider==='favorites'?'Favorite routes':ctx.providers.find(p=>p.id===state.provider)?.name??'All connections'),badge(String(groupFiltered.length))),list(groupFiltered,ctx)));
 else if(state.variant==='compact'){
  const select=el('select',{'aria-label':'Provider and model',class:'compact-select',onChange:e=>{const c=ctx.all.find(c=>c.model.id===e.target.value);if(c)ctx.select(c);}},ctx.candidates.length?ctx.providers.map(p=>el('optgroup',{label:p.name},ctx.candidates.filter(c=>c.route.providerId===p.id).map(c=>el('option',{value:c.model.id},`${c.model.name} · ${c.connection.displayName}${c.eligible?'':' · needs attention'}`)))):el('option',{value:'',disabled:true},'No model routes reported'));
  select.value=state.selection?.modelId??'';
  content=el('div',{class:'variant-stage'},el('div',{class:'mini-composer'},el('h3',{},'A model picker that fits anywhere'),el('p',{},'Provider, account, and model stay together—even in one compact control.'),select,el('textarea',{placeholder:'Your application’s prompt goes here. This preview does not send it.','aria-label':'Prompt preview',rows:4})),el('p',{class:'muted'},'The configuration panel uses the same native option descriptors as every other selector.'));
 }else if(state.variant==='palette')content=el('div',{class:'variant-stage'},el('div',{class:'palette-promo'},badge('Keyboard first','accent'),el('h3',{},'Find a route without leaving the keyboard.'),el('p',{},'Search model names, provider names, accounts, and native identifiers.'),button('Open command palette',ctx.openPalette,{class:'primary'}),el('kbd',{},'Ctrl / ⌘ + K')),list(ctx.candidates.slice(0,4),ctx));
 else if(state.variant==='capability'){
  const requirements=el('div',{class:'requirements'},[['generation.text','Text generation'],['input.text','Text input'],['input.image','Image input'],['structured.output','Structured output'],['tools','Tool support'],['offline','Offline operation']].map(([id,label])=>checkbox(label,state.required.has(id),on=>{on?state.required.add(id):state.required.delete(id);ctx.render();})));
  content=el('div',{},requirements,list(ctx.candidates,ctx));
 }else if(state.variant==='compare'){
  content=el('div',{class:'table-scroll'},el('table',{class:'compare-table'},el('thead',{},el('tr',{},['Model / route','Billing','Generation','Text input','Image input','Structure','Tools','Select'].map(t=>el('th',{scope:'col'},t)))),el('tbody',{},ctx.candidates.length?ctx.candidates.slice(0,50).map(c=>el('tr',{},el('td',{},el('strong',{},c.model.name),el('small',{},c.connection.displayName)),el('td',{},c.billingLabel),['generation.text','input.text','input.image','structured.output','tools'].map(id=>el('td',{},c.model.capabilities.find(cap=>cap.id===id)?.support??'unknown')),el('td',{},button('Inspect',()=>ctx.select(c),{'data-testid':c.model.id,'aria-label':`Inspect ${c.model.name} through ${c.connection.displayName}`})))):el('tr',{class:'compare-empty'},el('td',{colSpan:8},emptyRoutes(ctx))))));
 }else if(state.variant==='allowance'){
  const labels=[...new Set(ctx.candidates.map(c=>c.billingLabel))];
  content=labels.length?el('div',{class:'allowance-groups'},labels.map(label=>el('section',{class:'allowance-group'},el('h3',{},label),el('p',{class:'muted'},label==='Included allowance'?'Existing allowance, not unlimited or free.':label==='Metered access'?'A separate billing decision is required.':'Use the evidence reported by this connection.'),list(ctx.candidates.filter(c=>c.billingLabel===label),ctx)))):el('div',{class:'variant-stage'},emptyRoutes(ctx));
 }else if(state.variant==='preset'){
  const presets=[['Text task',['generation.text']],['Structured result',['generation.text','structured.output']],['Offline task',['generation.text','offline']],['Tool-assisted task',['generation.text','tools']]];
  content=el('div',{},el('div',{class:'preset-grid'},presets.map(([name,required])=>button(el('span',{},el('strong',{},name),el('small',{},required.map(id=>capNames[id]??id).join(' + '))),()=>{state.required=new Set(required);state.onlyEligible=true;ctx.render();},{class:'preset'}))),el('p',{class:'list-footnote'},'Presets filter requirements. They do not automatically select an account, start a turn, or claim a model is best.'),list(ctx.candidates,ctx));
 }else{
  const back=()=>{state.mobileStep=Math.max(0,state.mobileStep-1);ctx.render();};
  let body;
  if(state.mobileStep===0)body=ctx.providers.map(p=>button(p.name,()=>{state.mobileProvider=p.id;state.mobileStep=1;ctx.render();},{class:'drill-row'}));
  else if(state.mobileStep===1)body=ctx.connections.filter(c=>c.providerId===state.mobileProvider).map(c=>button(c.displayName,()=>{state.mobileConnection=c.id;state.mobileStep=2;ctx.render();},{class:'drill-row'}));
  else if(state.mobileStep===2)body=list(ctx.candidates.filter(c=>c.connection.id===state.mobileConnection),{...ctx,select:c=>{state.mobileStep=3;ctx.select(c);}});
  else body=el('div',{class:'drill-finished'},badge('Route selected','accent'),el('h3',{},'Now choose its native options.'),el('p',{},'The configuration panel below retains the exact account, model identifier, and supported settings.'),button('Review configuration',()=>document.querySelector('#configuration-panel')?.scrollIntoView({behavior:'smooth'})));
  content=el('div',{class:'phone-stage'},el('div',{class:'phone'},el('div',{class:'phone-header'},state.mobileStep?button('← Back',back):el('span',{},'Choose a provider'),badge(`${state.mobileStep+1} / 4`)),el('div',{class:'phone-body'},body)));
 }
 return el('div',{id:'variant-view','data-current-variant':state.variant,class:'selector-surface'},content);
}
