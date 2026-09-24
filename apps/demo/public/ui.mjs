/** Small DOM helpers. Runtime-supplied labels are always text nodes, never HTML. */
export function el(tag,attributes={},...children){
 const node=document.createElement(tag);
 for(const child of children.flat(Infinity))if(child!==null&&child!==undefined&&child!==false)node.append(child instanceof Node?child:document.createTextNode(String(child)));
 for(const [key,value]of Object.entries(attributes)){
  if(value===undefined||value===null)continue;
  if(key.startsWith('on')&&typeof value==='function')node.addEventListener(key.slice(2).toLowerCase(),value);
  else if(key==='value'||key==='checked'||key==='disabled')node[key]=value;
  else node.setAttribute(key=== 'className'?'class':key,String(value));
 }
 return node;
}
export const button=(text,action,attributes={})=>el('button',{type:'button',onClick:action,...attributes},text);
export const badge=(text,kind='muted')=>el('span',{class:'badge '+kind},text);
export const code=value=>el('pre',{class:'code'},JSON.stringify(value,null,2));
export const known=observation=>observation?.state==='known'?observation.value:undefined;
export const titleCase=text=>String(text).replaceAll('_',' ').replace(/\b\w/g,c=>c.toUpperCase());
export function checkbox(label,checked,onChange){
 return el('label',{class:'check'},el('input',{type:'checkbox',checked,onChange:event=>onChange(event.target.checked)}),el('span',{},label));
}
export function empty(title,description){return el('div',{class:'empty'},el('h3',{},title),el('p',{},description));}
export function section(title,description,...children){return el('section',{class:'card'},el('div',{class:'section-heading'},el('h2',{},title),description?el('p',{},description):null),...children);}
