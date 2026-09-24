import {el,button,badge} from './ui.mjs';
import {resolveCandidates} from '/modules/selector/index.js';
export function openPalette(ctx){
 const dialog=el('dialog',{id:'palette-dialog',class:'palette-dialog','aria-label':'Find a provider model'});
 const results=el('div',{class:'palette-results'});
 const input=el('input',{type:'search','aria-label':'Search every model',placeholder:'Search every provider, model, account…',class:'palette-search'});
 const refresh=()=>{
  const candidates=ctx.catalog?resolveCandidates(ctx.catalog,{query:input.value,allowPaidFallback:ctx.state.allowPaid}).slice(0,40):[];
  results.replaceChildren(...candidates.map(c=>button(el('span',{},el('strong',{},c.model.name),el('small',{},`${c.connection.displayName} · ${c.billingLabel}`)),()=>{ctx.select(c);dialog.close();},{class:'palette-result'})));
  if(!candidates.length)results.append(el('p',{class:'muted'},'No matching routes.'));
 };
 input.addEventListener('input',refresh);
 input.addEventListener('keydown',event=>{if(event.key==='ArrowDown'){event.preventDefault();results.querySelector('button')?.focus();}});
 results.addEventListener('keydown',event=>{
  if(!['ArrowDown','ArrowUp'].includes(event.key))return;
  event.preventDefault();const buttons=[...results.querySelectorAll('button')],index=buttons.indexOf(document.activeElement);
  const next=index+(event.key==='ArrowDown'?1:-1);
  if(next<0)input.focus();else buttons[Math.min(buttons.length-1,next)]?.focus();
 });
 dialog.append(el('div',{class:'dialog-heading'},el('h2',{},'Find a route'),button('Close',()=>dialog.close())),input,results,el('div',{class:'palette-footer'},el('kbd',{},'↑ ↓'),el('span',{},'Navigate'),el('kbd',{},'Enter'),el('span',{},'Inspect'),el('kbd',{},'Esc'),el('span',{},'Close')));
 dialog.addEventListener('close',()=>dialog.remove(),{once:true});document.body.append(dialog);refresh();dialog.showModal();input.focus();
}
export function openConnectionDialog(ctx){
 const dialog=el('dialog',{class:'connect-dialog','aria-label':'Connect an authorized local runtime'});
 const endpoint=el('input',{id:'runtime-endpoint',type:'url',value:'http://127.0.0.1:4499',required:true});
 const token=el('input',{id:'app-token',type:'password',autocomplete:'off',required:true});
 const form=el('form',{},el('div',{class:'dialog-heading'},el('h2',{},'Connect local runtime'),button('Close',()=>dialog.close())),
  el('p',{class:'muted'},'Connect only to a runtime you started and explicitly authorized. Use an app-session token—not an AI provider key.'),
  el('label',{class:'field',for:'runtime-endpoint'},'Runtime endpoint',endpoint),el('label',{class:'field',for:'app-token'},'App session token',token),
  el('p',{class:'fine-print'},'Live execution is disabled in this preview. Failed live connection attempts never fall back to example accounts.'),
  el('button',{type:'submit',class:'primary'},'Connect runtime'));
 form.addEventListener('submit',event=>{event.preventDefault();const address=endpoint.value,sessionToken=token.value;token.value='';dialog.close();ctx.connectLive(address,sessionToken);});
 dialog.append(form);dialog.addEventListener('close',()=>dialog.remove(),{once:true});document.body.append(dialog);dialog.showModal();endpoint.focus();
}
