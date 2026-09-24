import {createHttpTransport,createLocalAI} from '/modules/sdk/index.js';
import {mountDemo} from './app.mjs';
async function connect(endpoint,token){return createLocalAI({clientName:'local-ai-kit-showcase',transport:createHttpTransport({baseUrl:endpoint,token})});}
async function fixture(){
 const response=await fetch('/fixture/bootstrap',{credentials:'omit',cache:'no-store',redirect:'error'});
 if(!response.ok)throw Error('FIXTURE_BOOTSTRAP_FAILED');
 const bootstrap=await response.json();
 if(bootstrap.mode!=='synthetic'||typeof bootstrap.sessionToken!=='string')throw Error('INVALID_FIXTURE_BOOTSTRAP');
 return connect(window.location.origin,bootstrap.sessionToken);
}
mountDemo({connectFixture:fixture,connectLive:connect}).catch(()=>{
 const status=document.createElement('p');status.textContent='The showcase could not start. Build the SDK packages and run the explicit fixture server.';
 document.querySelector('#app').replaceChildren(status);
});
