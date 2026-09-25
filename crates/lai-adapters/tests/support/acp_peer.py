import sys,json,time
mode=sys.argv[1] if len(sys.argv)>1 else 'normal'
session='session-native'
options=[{'id':'model','name':'Model','category':'model','type':'select','currentValue':'native-x','options':[{'value':'native-x','name':'Native X'},{'value':'native-y','name':'Native Y'}]}, {'id':'thought','name':'Thought','category':'thought_level','type':'select','currentValue':'future','options':[{'value':'future','name':'Future'},{'value':'tiny','name':'Tiny'}]}]
pending=None
turns=0
authenticated=False
def out(value):print(json.dumps({'jsonrpc':'2.0',**value}),flush=True)
def answer(i,result):out({'id':i,'result':result})
def update(kind,**data):out({'method':'session/update','params':{'sessionId':'foreign' if mode=='foreign' else session,'update':{'sessionUpdate':kind,**data}}})
for line in sys.stdin:
 r=json.loads(line);m=r.get('method');i=r.get('id');p=r.get('params',{})
 if r.get('jsonrpc')!='2.0':sys.exit(7)
 if m=='initialize':
  if p.get('clientCapabilities',{}).get('terminal') or p.get('clientCapabilities',{}).get('fs',{}).get('writeTextFile'):sys.exit(8)
  answer(i,{'protocolVersion':9 if mode=='version' else 1,'agentInfo':{'name':'Fixture','version':'1'},'agentCapabilities':{'loadSession':mode!='no_resume'},'authMethods':[{'id':'oauth-personal','name':'Personal login'}]})
 elif m=='authenticate':
  if mode=='auth_fail':out({'id':i,'error':{'code':-32000,'message':'SECRET_SENTINEL_AUTH_URL'}})
  else:authenticated=True;answer(i,None)
 elif m=='session/new':
  if mode=='resume':sys.exit(11)
  if mode in ('auth','auth_fail') and not authenticated:
   out({'id':i,'error':{'code':-32000,'message':'SECRET_SENTINEL_AUTH_REQUIRED'}});continue
  if p.get('mcpServers')!=[]:sys.exit(9)
  answer(i,{'sessionId':session,'configOptions':options})
 elif m=='session/load':
  if p.get('mcpServers')!=[]:sys.exit(12)
  if p.get('sessionId')!=session:out({'id':i,'error':{'code':-32004,'message':'session not found'}})
  else:answer(i,{'configOptions':options})
 elif m=='session/set_config_option':
  for o in options:
   if o['id']==p['configId']:o['currentValue']=p['value']
  if mode=='dependent' and p['configId']=='model':options[1]['options']=[{'value':'only-new','name':'Only new'}];options[1]['currentValue']='only-new'
  answer(i,{'configOptions':options})
 elif m=='session/prompt':
  turns+=1;pending=i
  if mode=='hang':continue
  if mode=='permission':out({'id':'permission-id','method':'session/request_permission','params':{'sessionId':session,'toolCall':{'toolCallId':'tool-1','title':'Read fixture'},'options':[{'optionId':'yes','name':'Allow once','kind':'allow_once'},{'optionId':'no','name':'Reject once','kind':'reject_once'}]}})
  elif mode=='fs':out({'id':'fs-id','method':'fs/read_text_file','params':{'sessionId':session,'path':'private-sentinel'}})
  elif mode=='changed':
   options[0]['currentValue']='native-y';update('config_option_update',configOptions=options);answer(i,{'stopReason':'end_turn'})
  else:
   update('agent_message_chunk',content={'type':'text','text':'native reply '+str(turns)});update('usage_update',used=12,size=1024);answer(i,{'stopReason':'end_turn'})
 elif m=='session/cancel':
  if pending is not None:answer(pending,{'stopReason':'cancelled'})
 elif m=='session/close':answer(i,{})
 elif i=='permission-id':
  result=r.get('result',{}).get('outcome',{});update('agent_message_chunk',content={'type':'text','text':'allowed' if result.get('optionId')=='yes' else 'denied'});answer(pending,{'stopReason':'end_turn'})
 elif i=='fs-id':
  if 'error' not in r:sys.exit(10)
  answer(pending,{'stopReason':'refusal'})
 else:out({'id':i,'error':{'code':-32601,'message':'fixture unsupported'}})
