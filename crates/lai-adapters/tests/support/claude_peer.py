import sys,json
mode=sys.argv[1]
def send(value):print(json.dumps(value),flush=True)
def reply(r,result):send({'type':'control_response','response':{'subtype':'success','request_id':r['request_id'],'response':result}})
for line in sys.stdin:
 r=json.loads(line)
 if r.get('type')=='control_request':
  subtype=r['request']['subtype']
  if subtype=='initialize':reply(r,{'account':{'email':'person@example.test','subscriptionType':'Claude Test Plan','apiProvider':'firstParty'},'models':[{'value':'native','displayName':'Native','supportsEffort':True,'supportedEffortLevels':['future-effort']}],'current_permission_mode':'dontAsk'})
  elif subtype=='get_usage':reply(r,{'rate_limits_available':False})
  elif subtype=='interrupt':reply(r,{})
  elif subtype=='mcp_status':
   status={'mcp_failed':'failed'}.get(mode,'connected')
   reply(r,{'mcpServers':[{'name':'design','status':status,'tools':[{'name':'echo_color'}]}]})
  else:send({'type':'control_response','response':{'subtype':'error','request_id':r['request_id'],'error':'unsupported'}})
 elif r.get('type')=='user':
  assert r['client_composed'] is True
  if mode=='hang':continue
  if mode=='tool':send({'type':'control_request','request_id':'native-permission','request':{'subtype':'can_use_tool','tool_name':'Bash','input':{'command':'echo prohibited'}}});continue
  if mode=='host_tool':
   send({'type':'system','subtype':'init','session_id':'fixture-session','tools':['mcp__design__echo_color']})
   send({'type':'stream_event','session_id':'fixture-session','event':{'type':'content_block_start','index':0,'content_block':{'type':'tool_use','id':'tu-1','name':'mcp__design__echo_color','input':{}}}})
   send({'type':'assistant','session_id':'fixture-session','message':{'content':[{'type':'tool_use','id':'tu-1','name':'mcp__design__echo_color','input':{'color':'red'}}]}})
   send({'type':'user','session_id':'fixture-session','message':{'role':'user','content':[{'type':'tool_result','tool_use_id':'tu-1','content':'red'}]}})
   send({'type':'stream_event','session_id':'fixture-session','event':{'type':'content_block_delta','delta':{'type':'text_delta','text':'color is red'}}})
   send({'type':'result','session_id':'fixture-session','subtype':'success','is_error':False,'usage':{'input_tokens':10,'output_tokens':3}})
   continue
  if mode=='native_tool_use':
   send({'type':'system','subtype':'init','session_id':'fixture-session','tools':['mcp__design__echo_color']})
   send({'type':'assistant','session_id':'fixture-session','message':{'content':[{'type':'tool_use','id':'tu-2','name':'Read','input':{'file_path':'secret'}}]}})
   send({'type':'result','session_id':'fixture-session','subtype':'success','is_error':False})
   continue
  if mode=='advertise_native':
   send({'type':'system','subtype':'init','session_id':'fixture-session','tools':['mcp__design__echo_color','Bash']})
   send({'type':'result','session_id':'fixture-session','subtype':'success','is_error':False})
   continue
  send({'type':'system','subtype':'init','session_id':'fixture-session','tools':[]})
  send({'type':'stream_event','session_id':'fixture-session','event':{'type':'content_block_delta','delta':{'type':'text_delta','text':'native Claude reply'}}})
  send({'type':'assistant','session_id':'fixture-session','message':{'content':[{'type':'text','text':'native Claude reply'}]}})
  send({'type':'result','session_id':'fixture-session','subtype':'success','is_error':False,'usage':{'input_tokens':10,'output_tokens':3}})
 elif r.get('type')=='control_response':
  assert r['response']['response']['behavior']=='deny'
  send({'type':'result','session_id':'fixture-session','subtype':'error_during_execution','is_error':True})
