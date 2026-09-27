// Test-only browser host; never used by the shipped live/HTTP entry point.
import {mountDemo} from '../public/app.mjs';
import {fixtureSnapshot} from '../fixture/snapshot.mjs';
import {createFixtureService} from '../fixture/service.mjs';
import {fixtureTransport} from '../fixture/transport.mjs';
import {createLocalAI,LocalAIError} from '../../../packages/sdk-typescript/dist/index.js';
globalThis.mountTestApp=async(options={})=>{
 const snapshot=fixtureSnapshot();
 if(options.maliciousLabel)snapshot.models[0].name='<img src=x onerror="globalThis.__unsafeExecuted=true">';
 if(options.largeCatalog){const models=snapshot.models;snapshot.models=Array.from({length:5000},(_,i)=>({...models[i%models.length],id:'huge-'+i,name:'Catalog Model '+i,nativeId:'fixture/huge-'+i}));}
 const connectFixture=async()=>createLocalAI({transport:fixtureTransport(createFixtureService({snapshot}))});
 let initialClient=null;
 if(options.nativeResumption){
  globalThis.testNativeCalls=[];
  const service=createFixtureService({snapshot}),transport=fixtureTransport(service),request=transport.request.bind(transport);
  let previousSession=null;
  transport.request=async(method,params={},call={})=>{
   if(['sessions.create','sessions.close','turns.start'].includes(method))globalThis.testNativeCalls.push(method);
   if(method==='runtime.handshake'){
    const handshake=await request(method,params,call);
    return {...handshake,methods:[...handshake.methods,'sessions.suspend','sessions.resume']};
   }
   if(method==='sessions.create'){
    previousSession={...await request(method,params,call),nativeResumption:true};
    return previousSession;
   }
   if(method==='sessions.suspend')return {resumeToken:'test-only-resume-token-sentinel'};
   if(method==='sessions.resume')return {...previousSession,state:'ready'};
   return request(method,params,call);
  };
  initialClient=await createLocalAI({transport});
 }
 return mountDemo({connectFixture,connectLive:async()=>{throw new LocalAIError('TRANSPORT_FAILED');},initialClient,initialMode:'synthetic'});
};
