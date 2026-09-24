import {cp,mkdir,rm} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
import {resolve,dirname} from 'node:path';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const destination=resolve(root,'apps/demo/dist');
// Only remove the generated, project-owned distribution directory.
await rm(destination,{recursive:true,force:true});
await mkdir(destination,{recursive:true});
await cp(resolve(root,'apps/demo/public'),destination,{recursive:true});
for(const [source,name]of [['sdk-typescript','sdk'],['selector-core','selector']]){
 await cp(resolve(root,'packages',source,'dist'),resolve(destination,'modules',name),{recursive:true});
}
console.log('Built native-served demo assets: apps/demo/dist');
