import sys,json,os
mode=sys.argv[1]
model={"id":"native-id","model":"test-native","displayName":"Test native","description":"Fixture","hidden":False,"isDefault":True,"supportedReasoningEfforts":[{"reasoningEffort":"future","description":"Future"}],"defaultReasoningEffort":"future","inputModalities":["text"]}
def send(v): print(json.dumps(v),flush=True)
def emit(method,params): send({"method":method,"params":params})
def reply(id,value): send({"id":id,"result":value})
turn=''
account_reads=0
for line in sys.stdin:
 r=json.loads(line);method=r.get('method');p=r.get('params',{});id=r.get('id')
 if method=='initialize':reply(id,{"userAgent":"codex/0.156.1"})
 elif method=='initialized':pass
 elif method=='account/read':
  account_reads+=1
  reply(id,{"account":{"type":"chatgpt","email":"changed@example.test" if mode=='account-changed-before-turn' and account_reads>1 else "test@example.test","planType":"test"},"requiresOpenaiAuth":True})
 elif method=='model/list':reply(id,{"data":[model],"nextCursor":None})
 elif method=='account/rateLimits/read':reply(id,{"rateLimits":{}})
 elif method=='thread/start':
  assert p['ephemeral'] is True and p['sandbox']=='read-only' and p['serviceTier']=='default'
  assert p['config']['features.shell_tool'] is False and p['config']['features.unified_exec'] is False
  reply(id,{"thread":{"id":"native-thread"},"model":"test-native","sandbox":{"type":"dangerFullAccess" if mode=='unsafe' else 'readOnly'},"approvalPolicy":"onRequest","cwd":os.getcwd()})
 elif method=='turn/start':
  assert p['model']=='test-native' and p['effort']=='future'
  assert p['sandboxPolicy']=={'type':'readOnly'}
  turn='turn-'+str(id)
  if mode=='early':emit('item/agentMessage/delta',{"threadId":"native-thread","turnId":turn,"delta":"early "})
  reply(id,{"turn":{"id":turn,"status":"inProgress"}})
  if mode=='approval':send({"id":900,"method":"item/commandExecution/requestApproval","params":{"threadId":"native-thread","turnId":turn,"command":"echo fixture","cwd":os.getcwd()}})
  elif mode=='hang':pass
  elif mode=='account-switch':emit('account/updated',{"authMode":"chatgpt"})
  else:
   emit('item/agentMessage/delta',{"threadId":"native-thread","turnId":turn,"delta":"native reply"})
   emit('turn/completed',{"threadId":"native-thread","turn":{"id":turn,"status":"completed"}})
 elif method=='turn/interrupt':reply(id,{});emit('turn/completed',{"threadId":"native-thread","turn":{"id":turn,"status":"interrupted"}})
 elif id==900:
  assert r['result']['decision']=='decline'
  emit('turn/completed',{"threadId":"native-thread","turn":{"id":turn,"status":"completed"}})
 else:send({"id":id,"error":{"code":-32601,"message":"Unsupported fixture operation"}})
