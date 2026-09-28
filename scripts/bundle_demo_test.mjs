/** Offline test bundling for the React demo using the pinned esbuild compiler. */
import {build} from 'esbuild';
import {fileURLToPath} from 'node:url';
import {resolve,dirname,basename,extname} from 'node:path';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
if(!process.argv[2])throw Error('Pass an explicit output path');
const output=resolve(root,process.argv[2]);
const name=basename(output,extname(output));
// Styles come from the Vite/Tailwind build (`apps/demo/dist/assets/*.css`);
// esbuild only needs the application logic for the offline test host.
const emptyCss={name:'empty-css',setup(api){
 api.onResolve({filter:/\.css$/},args=>({path:args.path,namespace:'empty-css'}));
 api.onLoad({filter:/.*/,namespace:'empty-css'},()=>({contents:'export default "";',loader:'js'}));
}};
await build({
 entryPoints:[resolve(root,'apps/demo/tests/fixture-entry.tsx')],
 bundle:true,
 write:true,
 outdir:resolve(output,'..'),
 entryNames:name,
 assetNames:name+'-assets/[name]',
 format:'iife',
 platform:'browser',
 target:'es2022',
 jsx:'automatic',
 loader:{'.mjs':'js'},
 plugins:[emptyCss],
 resolveExtensions:['.tsx','.ts','.jsx','.js','.mjs','.json'],
});
console.log('Bundled demo test host: '+output);
