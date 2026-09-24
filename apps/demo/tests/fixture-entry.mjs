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
 return mountDemo({connectFixture:async()=>createLocalAI({transport:fixtureTransport(createFixtureService({snapshot}))}),connectLive:async()=>{throw new LocalAIError('TRANSPORT_FAILED');}});
};
