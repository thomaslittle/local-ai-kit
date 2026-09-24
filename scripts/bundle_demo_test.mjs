/** Dependency-free offline test bundling using the already pinned TypeScript compiler. */
import {createRequire} from 'node:module';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve,dirname,relative,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
const require=createRequire(import.meta.url),ts=require('typescript');
const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const modules=new Map(),mappings={};
function dependency(from,name){
 let path;if(name.startsWith('/modules/sdk/'))path=resolve(root,'packages/sdk-typescript/dist',name.slice(13));
 else if(name.startsWith('/modules/selector/'))path=resolve(root,'packages/selector-core/dist',name.slice(18));
 else if(name.startsWith('.'))path=resolve(dirname(from),name);
 else throw Error('External test-bundle import is unsupported: '+name);
 if(relative(root,path).startsWith('..'))throw Error('Import outside project');return path;
}
function add(path){
 const id=relative(root,path).split(sep).join('/');if(modules.has(id))return id;modules.set(id,'');
 const source=readFileSync(path,'utf8'),imports={};
 for(const item of ts.preProcessFile(source,true,true).importedFiles)imports[item.fileName]=add(dependency(path,item.fileName));
 mappings[id]=imports;
 modules.set(id,ts.transpileModule(source,{fileName:path+".ts",compilerOptions:{module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2022,allowJs:true}}).outputText);return id;
}
const entry=add(resolve(root,'apps/demo/tests/fixture-entry.mjs'));
const body=[`(()=>{const modules={},cache={},map=${JSON.stringify(mappings)};`];
for(const [id,code]of modules)body.push(`modules[${JSON.stringify(id)}]=(module,exports,require)=>{\n${code}\n};`);
body.push(`function load(id){if(cache[id])return cache[id].exports;const module={exports:{}};cache[id]=module;modules[id](module,module.exports,name=>load(map[id][name]));return module.exports;}load(${JSON.stringify(entry)});})();`);
if(!process.argv[2])throw Error('Pass an explicit output path');writeFileSync(process.argv[2],body.join('\n'));
