import {test} from 'node:test';
import assert from 'node:assert/strict';
import {filterLiveCatalog} from '../apps/demo/lib/catalog_visibility.mjs';

const source={kind:'native',adapterId:'test',adapterVersion:'1',reference:'fixture'};
const observed=value=>({state:'known',value,source,observedAt:'2026-09-25T12:00:00Z',freshness:'fresh',verification:'observed'});
const unknown={state:'unknown',reason:'NOT_REPORTED'};
const snapshot={mode:'live',connections:[
 {id:'verified',providerId:'claude',auth:observed('authenticated')},
 {id:'unverified',providerId:'claude',auth:unknown},
 {id:'stale',providerId:'grok',auth:{...observed('authenticated'),freshness:'stale'}},
 {id:'local',providerId:'lmstudio',auth:unknown},
 {id:'freebuff',providerId:'freebuff',auth:{...observed('unchecked'),source:{...source,reference:'Freebuff local model catalog; account sign-in is not required'}}},
 {id:'unchecked',providerId:'cursor',auth:observed('unchecked')},
],routes:[
 {id:'r-verified',connectionId:'verified',billing:unknown},
 {id:'r-unverified',connectionId:'unverified',billing:unknown},
 {id:'r-stale',connectionId:'stale',billing:unknown},
 {id:'r-local',connectionId:'local',billing:observed('local')},
 {id:'r-freebuff',connectionId:'freebuff',billing:unknown},
 {id:'r-unchecked',connectionId:'unchecked',billing:unknown},
],models:['r-verified','r-unverified','r-stale','r-local','r-freebuff','r-unchecked'].map((routeId,index)=>({id:`m-${index}`,routeId}))};

test('live catalog excludes unauthenticated and stale subscription profiles but keeps local runtime models',()=>{
 const result=filterLiveCatalog(snapshot);
 assert.deepEqual(result.models.map(model=>model.routeId),['r-verified','r-local','r-freebuff']);
 assert.deepEqual(result.routes.map(route=>route.id),['r-verified','r-local','r-freebuff']);
 assert.equal(result.connections.length,6);
});

test('synthetic example records are left untouched by the live catalog filter',()=>{
 const synthetic={...snapshot,mode:'synthetic'};
 assert.equal(filterLiveCatalog(synthetic),synthetic);
});
