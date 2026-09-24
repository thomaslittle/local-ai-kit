import test from 'node:test';
import assert from 'node:assert/strict';
import { createHttpTransport, createLocalAI, parseSSE } from '../src/index.ts';
const bytes=text=>new TextEncoder().encode(text);
const stream=(parts)=>new ReadableStream({start(c){for(const p of parts)c.enqueue(typeof p==='string'?bytes(p):p);c.close();}});
const response=(id,result)=>new Response(JSON.stringify({jsonrpc:'2.0',id,result}),{headers:{'content-type':'application/json'}});
const collect=async iterable=>{const out=[];for await(const item of iterable)out.push(item);return out;};
const config=(fetch)=>({baseUrl:'http://127.0.0.1:4499',token:'synthetic-token',fetch});
test('constructing a transport has no I/O',()=>{
 let calls=0;const t=createHttpTransport(config(()=>{calls++;throw Error('unexpected');}));assert.equal(calls,0);assert.equal(JSON.stringify(t).includes('synthetic-token'),false);
});
test('request uses header auth, exact method/params and no cookies or redirects',async()=>{
 const t=createHttpTransport(config(async(url,init)=>{
  assert.equal(url,'http://127.0.0.1:4499/v1/rpc');assert.equal(init.method,'POST');assert.equal(init.redirect,'error');assert.equal(init.credentials,'omit');
  assert.equal(new Headers(init.headers).get('authorization'),'Bearer synthetic-token');
  const b=JSON.parse(init.body);assert.equal(b.method,'snapshot.get');assert.deepEqual(b.params,{fields:['connections']});return response(b.id,{revision:'7'});
 }));assert.deepEqual(await t.request('snapshot.get',{fields:['connections']}),{revision:'7'});await t.close();
});
test('nonlocal endpoints, URL credentials, query secrets and header injection are rejected',()=>{
 for(const baseUrl of ['https://example.com','http://127.0.0.1.evil.test','http://user:secret@localhost','http://localhost?token=secret','file:///tmp/x','http://localhost/#secret'])
  assert.throws(()=>createHttpTransport({...config(fetch),baseUrl}),/LOCAL_ENDPOINT_REQUIRED/);
 assert.throws(()=>createHttpTransport({...config(fetch),token:'bad\r\nInjected: yes'}),/INVALID_AUTH/);
});
test('malformed or mismatched envelopes are rejected',async()=>{
 for(const body of [r=>({id:r.id,result:1}),r=>({jsonrpc:'2.0',id:r.id+1,result:1}),r=>({jsonrpc:'2.0',id:r.id,result:1,error:{code:1}})]){
  const t=createHttpTransport(config(async(_u,init)=>new Response(JSON.stringify(body(JSON.parse(init.body))),{headers:{'content-type':'application/json'}})));
  await assert.rejects(t.request('snapshot.get'),/PROTOCOL_INVALID/);await t.close();
 }
});
test('provider error details are not reflected into SDK errors',async()=>{
 const t=createHttpTransport(config(async(_u,init)=>new Response(JSON.stringify({jsonrpc:'2.0',id:JSON.parse(init.body).id,error:{code:-32000,message:'secret synthetic-token',data:{code:'AUTH_REQUIRED',message:'private path',retryable:false}}}),{headers:{'content-type':'application/json'}})));
 await assert.rejects(t.request('snapshot.get'),e=>e.code==='AUTH_REQUIRED'&&!String(e).includes('synthetic-token')&&!String(e).includes('private path'));
});
test('no automatic retry on HTTP or ambiguous transport failure',async()=>{
 let calls=0;const t=createHttpTransport(config(async()=>{calls++;throw Error('request may have executed: secret');}));
 await assert.rejects(t.request('turns.start'),/TRANSPORT_FAILED/);assert.equal(calls,1);
});
test('response bytes are bounded before JSON parsing',async()=>{
 let cancelled=false;const body=new ReadableStream({start(c){c.enqueue(bytes('x'.repeat(1025)));},cancel(){cancelled=true;}});
 const t=createHttpTransport({...config(async()=>new Response(body,{headers:{'content-type':'application/json'}})),maxFrameBytes:1024});
 await assert.rejects(t.request('snapshot.get'),/FRAME_TOO_LARGE/);assert.equal(cancelled,true);
});
test('outgoing request bytes are bounded without sending',async()=>{
 let calls=0;const t=createHttpTransport({...config(async()=>{calls++;return response(1,{});}),maxFrameBytes:1024});
 await assert.rejects(t.request('turns.start',{prompt:'x'.repeat(2000)}),/FRAME_TOO_LARGE/);assert.equal(calls,0);
});
test('aborted request sends nothing',async()=>{
 let calls=0;const abort=new AbortController();abort.abort();const t=createHttpTransport(config(async()=>{calls++;return response(1,{});}));
 await assert.rejects(t.request('snapshot.get',{}, {signal:abort.signal}),/CANCELLED/);assert.equal(calls,0);
});
test('timeout aborts a hung request and never retries',async()=>{
 let calls=0;const t=createHttpTransport({...config((_u,init)=>{calls++;return new Promise((_r,reject)=>init.signal.addEventListener('abort',()=>reject(init.signal.reason),{once:true}));}),timeoutMs:20});
 await assert.rejects(t.request('snapshot.get'),/TIMEOUT/);assert.equal(calls,1);
});
test('close aborts active requests and rejects future requests',async()=>{
 const t=createHttpTransport(config((_u,init)=>new Promise((_r,reject)=>init.signal.addEventListener('abort',()=>reject(init.signal.reason),{once:true}))));
 const pending=t.request('snapshot.get');const check=assert.rejects(pending,/CLOSED/);await t.close();await check;await assert.rejects(t.request('snapshot.get'),/CLOSED/);
});
test('client handshake rejects a mismatched protocol major and closes transport',async()=>{
 let closed=false;await assert.rejects(createLocalAI({transport:{request:async()=>({protocolVersion:'2.0',methods:[]}),close:async()=>{closed=true;}}}),/PROTOCOL_INCOMPATIBLE/);assert.equal(closed,true);
});
test('client methods call exact RPC names, without auto discovery or execution',async()=>{
 const calls=[];const t={request:async(method,params)=>{calls.push({method,params});return method==='runtime.handshake'?{protocolVersion:'1.0',methods:['snapshot.get','turns.cancel']}:{revision:'9'};},close:async()=>{}};
 const ai=await createLocalAI({transport:t});assert.deepEqual(calls.map(c=>c.method),['runtime.handshake']);assert.deepEqual(await ai.snapshot.get(),{revision:'9'});
 await ai.turns.cancel({turnId:'t'});assert.equal(calls[2].method,'turns.cancel');await assert.rejects(ai.turns.start({prompt:'x'}),/CAPABILITY_UNSUPPORTED/);assert.equal(calls.length,3);
});
test('SSE handles split UTF8, comments, CRLF and multiline data',async()=>{
 const b=bytes(': heartbeat\r\nid: 4\r\nevent: update\r\ndata: {"text":"🌎",\r\ndata: "x":1}\r\n\r\n');
 const frames=await collect(parseSSE(stream([...b].map(v=>new Uint8Array([v])))));
 assert.equal(frames.length,1);assert.deepEqual(frames[0],{id:'4',event:'update',data:'{"text":"🌎",\n"x":1}'});
});
test('SSE never emits an incomplete event at EOF',async()=>{
 assert.deepEqual(await collect(parseSSE(stream(['data: not-dispatched']))),[]);
});
test('SSE frame limits include unterminated lines',async()=>{
 await assert.rejects(collect(parseSSE(stream(['data: '+ 'x'.repeat(1100)]),{maxEventBytes:1024})),/FRAME_TOO_LARGE/);
});
test('authenticated event stream uses safe cursor and explicit gap error',async()=>{
 const t=createHttpTransport(config(async(url,init)=>{
  assert.equal(new URL(url).searchParams.has('token'),false);assert.equal(new URL(url).searchParams.get('after'),'7');
  assert.equal(new Headers(init.headers).get('authorization'),'Bearer synthetic-token');
  return new Response(stream(['event: gap\ndata: {}\n\n']),{headers:{'content-type':'text/event-stream'}});
 }));await assert.rejects(collect(t.subscribe({after:'7'})),/EVENT_GAP/);
});
test('plain text HTTP failures never disclose body content',async()=>{
 const t=createHttpTransport(config(async()=>new Response('secret synthetic-token',{status:403})));
 await assert.rejects(t.request('snapshot.get'),e=>e.code==='HTTP_403'&&!String(e).includes('synthetic-token'));
});
