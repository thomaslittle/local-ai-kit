import {spawn,execFile} from 'node:child_process';
import {promisify} from 'node:util';
import {randomBytes} from 'node:crypto';
import {access,copyFile,mkdir,mkdtemp,rm,writeFile} from 'node:fs/promises';
import {createServer} from 'node:net';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {buildOpenCodeRegistration,verifyOpenCodeAuthentication} from './live_demo_opencode.mjs';

const assets=resolve('apps/demo/dist');
const metadataProviders=['codex','claude_code','grok','gemini_cli','antigravity','opencode','ollama','lmstudio','cursor'];
const demoFlags=process.argv.slice(2);
const allowCodexExecution=demoFlags.length===1&&demoFlags[0]==='--allow-codex-execution';
if(demoFlags.length&&!allowCodexExecution){
 console.error('Usage: npm run demo [-- --allow-codex-execution]');
 process.exit(2);
}
const temp=await mkdtemp(join(tmpdir(),'local-ai-kit-live-'));
const workspace=allowCodexExecution?join(temp,'workspace'):undefined;
if(workspace)await mkdir(workspace);
const endpointFile=join(temp,'endpoints.json');
let opencode,helper,closing=false;

function allocatePort(){
 return new Promise((resolvePort,reject)=>{
  const server=createServer();
  server.once('error',reject);
  server.listen(0,'127.0.0.1',()=>{const port=server.address().port;server.close(error=>error?reject(error):resolvePort(port));});
 });
}
function executable(){
 if(process.env.OPENCODE_BIN)return process.env.OPENCODE_BIN;
 if(process.platform==='win32')return join(process.env.APPDATA??'','npm','node_modules','opencode-ai','bin','opencode.exe');
 return 'opencode';
}
async function startProtectedOpenCode(){
 const bin=executable();
 if(process.platform==='win32'&&!process.env.OPENCODE_BIN){
  try{await access(bin);}catch{return null;}
 }
 const port=await allocatePort();
 const password=randomBytes(32).toString('base64url');
 const registration=buildOpenCodeRegistration(port,password);
 await writeFile(endpointFile,JSON.stringify({endpoints:[registration.endpoint]}),{encoding:'utf8',mode:0o600});
 opencode=spawn(bin,registration.serverArgs,{windowsHide:true,stdio:'ignore',env:{...process.env,...registration.serverEnvironment}});
 let spawnError=null;
 opencode.once('error',error=>{spawnError=error;});
 let ready=false;
 for(let attempt=0;attempt<48;attempt++){
  if(spawnError||opencode.exitCode!==null)return null;
  try{
   if(await verifyOpenCodeAuthentication(registration.endpoint.baseUrl,password)){ready=true;break;}
  }catch{}
  await new Promise(resolveDelay=>setTimeout(resolveDelay,250));
 }
 if(!ready){opencode.kill();opencode=undefined;return null;}
 return registration;
}
async function close(){
 if(closing)return;
 closing=true;
 if(helper&&!helper.killed)helper.kill();
 if(opencode&&!opencode.killed)opencode.kill();
 await rm(temp,{recursive:true,force:true});
}
console.log('Building the native helper for the live demo...');
const build=await promisify(execFile)('cargo',['build','--locked','-p','lai-cli','--bin','local-ai-kit','--message-format=json'],{windowsHide:true,maxBuffer:16*1024*1024});
const artifact=build.stdout.split(/\r?\n/).filter(Boolean).map(line=>JSON.parse(line)).findLast(item=>item.reason==='compiler-artifact'&&item.target?.name==='local-ai-kit'&&item.executable);
if(!artifact)throw new Error('Native helper build did not report an executable');
// Running Cargo's output directly prevents Windows from rebuilding it while the demo is open.
const helperPath=join(temp,process.platform==='win32'?'local-ai-kit.exe':'local-ai-kit');
await copyFile(artifact.executable,helperPath);
const registration=await startProtectedOpenCode().catch(()=>null);
if(registration)console.log('OpenCode metadata server ready on loopback with temporary Basic Auth.');
else console.log('OpenCode metadata is unavailable; start a protected OpenCode server to inspect that connection.');
const env=registration?{...process.env,...registration.environment}:{...process.env};
const args=[
 'serve','--http','--port','4499','--assets',assets,'--allow-passive-discovery',
 ...metadataProviders.flatMap(provider=>['--allow-metadata',provider]),
 ...(allowCodexExecution?['--allow-codex-execution','--workspace',workspace]:[]),
 ...(registration?['--endpoints',endpointFile]:[]),
];
if(allowCodexExecution)console.log('Codex live turns are enabled in an isolated temporary workspace. No turn starts until you authorize and explicitly run one in the browser.');
helper=spawn(helperPath,args,{stdio:'inherit',windowsHide:true,env});
helper.once('error',error=>{console.error(`Unable to start the native helper: ${error.message}`);process.exitCode=1;void close();});
helper.once('exit',async(code,signal)=>{await close();process.exitCode=signal?1:code??1;});
for(const signal of ['SIGINT','SIGTERM'])process.once(signal,()=>{if(helper&&!helper.killed)helper.kill(signal);else void close();});
