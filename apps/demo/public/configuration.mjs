import {el,button,badge,code,known,empty} from './ui.mjs';
import {validateOption} from '/modules/selector/index.js';
import {reasonLabel} from './selectors.mjs';

export function configuration(ctx){
 const selected=ctx.all.find(c=>c.model.id===ctx.state.selection?.modelId);
 if(!selected)return el('aside',{class:'configuration card',id:'configuration-panel'},empty(ctx.all.length?'Choose a model':'No model route available',ctx.all.length?'Inspect a route to see its native settings and execution identity.':'Check account and model metadata on an authorized connection to load a catalog.'),button('Open Connections',()=>{ctx.state.page='Connections';ctx.render();}));
 const {model,route,connection}=selected,selection=ctx.state.selection;
 const setOption=(descriptor,value)=>{
  if(value!==undefined){
   const error=validateOption(descriptor,value);
   if(error){ctx.state.notice=error;ctx.render();return;}
   if((descriptor.billingSensitive||descriptor.permissionSensitive)&&!ctx.state.sensitiveConsent){ctx.state.notice='Review and allow sensitive configuration changes before changing this option.';ctx.render();return;}
  }
  const options={...selection.options};value===undefined?delete options[descriptor.id]:options[descriptor.id]=value;
  ctx.state.selection={...selection,options};ctx.state.notice='';ctx.render();
 };
 const optionNodes=model.options.map(descriptor=>{
  const id='native-'+descriptor.id,control=descriptor.control,value=selection.options[descriptor.id];let input;
  const common={id,'data-focus':id,'aria-label':descriptor.label};
  if(control.type==='select'){
   input=el('select',{...common,onChange:e=>setOption(descriptor,e.target.value===''?undefined:JSON.parse(e.target.value))},
    el('option',{value:''},control.defaultValue===undefined?'Use provider default':`Provider default (${control.choices.find(c=>c.value===control.defaultValue)?.label??control.defaultValue})`),
    control.choices.map(choice=>el('option',{value:JSON.stringify(choice.value),disabled:!!choice.disabledReason},choice.label+(choice.disabledReason?` — ${choice.disabledReason}`:''))));
   input.value=value===undefined?'':JSON.stringify(value);
  }else if(control.type==='boolean'){
   input=el('select',{...common,onChange:e=>setOption(descriptor,e.target.value===''?undefined:e.target.value==='true')},el('option',{value:''},'Use provider default'),el('option',{value:'true'},'Enabled'),el('option',{value:'false'},'Disabled'));
   input.value=value===undefined?'':String(value);
  }else if(control.type==='number')input=el('input',{...common,type:'number',min:control.min,max:control.max,step:control.step,value:value??'',placeholder:'Provider default',onChange:e=>setOption(descriptor,e.target.value===''?undefined:e.target.valueAsNumber)});
  else if(control.type==='text')input=el('input',{...common,type:'text',maxLength:control.maxLength,value:value??'',placeholder:'Provider default',onChange:e=>setOption(descriptor,e.target.value===''?undefined:e.target.value)});
  else input=el('div',{class:'unavailable'},`Inspectable, not editable: ${control.nativeType}`);
  const note=[`${descriptor.scope} option`,descriptor.billingSensitive?'May affect billing':null,descriptor.permissionSensitive?'Changes permissions':null].filter(Boolean).join(' · ');
  return el('div',{class:'field'},el('label',{for:id},descriptor.label),input,el('small',{class:'muted'},note));
 });
 const context=known(model.contextWindow);
 return el('aside',{class:'configuration card',id:'configuration-panel'},
  el('div',{class:'config-heading'},el('span',{class:'eyebrow'},'Selected route'),button(ctx.state.favorites.has(selected.key)?'★ Saved':'☆ Save',()=>{ctx.state.favorites.has(selected.key)?ctx.state.favorites.delete(selected.key):ctx.state.favorites.add(selected.key);ctx.render();},{class:'favorite','data-testid':'favorite-current','aria-label':ctx.state.favorites.has(selected.key)?'Remove current route from favorites':'Save current route to favorites'})),
  el('h2',{},model.name),el('p',{class:'muted'},connection.displayName),
  el('div',{class:'config-badges'},badge(selected.billingLabel,'accent'),badge(selected.eligible?'Eligible in snapshot':'Needs attention',selected.eligible?'supported':'warning')),
  el('dl',{class:'facts'},el('dt',{},'Provider'),el('dd',{},ctx.providers.find(p=>p.id===route.providerId)?.name??route.providerId),el('dt',{},'Native model'),el('dd',{class:'mono'},model.nativeId),el('dt',{},'Context'),el('dd',{},context?`${Number(context).toLocaleString()} · reported`:'Not reported'),el('dt',{},'Source'),el('dd',{},ctx.state.mode==='synthetic'?'Synthetic example':'Connected runtime')),
  el('div',{class:'config-options'},el('h3',{},'Native options'),optionNodes.length?optionNodes:el('p',{class:'muted'},'This model advertises no editable options. Nothing is assumed.')),
  model.options.some(o=>o.billingSensitive||o.permissionSensitive)?el('label',{class:'check'},el('input',{type:'checkbox',checked:ctx.state.sensitiveConsent,onChange:e=>{ctx.state.sensitiveConsent=e.target.checked;ctx.render();}}),'Allow sensitive configuration changes'):null,
  selected.reasons.length?el('div',{class:'notice warning'},selected.reasons.map(reasonLabel).join(' · ')):null,
  el('details',{},el('summary',{},'Exact selection payload'),el('pre',{id:'selection-json',class:'code'},JSON.stringify(selection,null,2))),
  el('p',{class:'fine-print'},'Selection is not authorization. The runtime must recheck access, settings, and permissions before execution.'));
}
