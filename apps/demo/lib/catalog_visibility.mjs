const localRuntimeProviders=new Set(['lmstudio','ollama']);
export const isLocalRuntimeProvider=connection=>localRuntimeProviders.has(connection?.providerId);
const freshAuthentication=connection=>connection?.auth?.state==='known'
 &&connection.auth.value==='authenticated'
 &&connection.auth.freshness==='fresh';
const localRuntime= (connection,route)=>isLocalRuntimeProvider(connection)
 &&route?.billing?.state==='known'
 &&route.billing.value==='local'
 &&route.billing.freshness==='fresh';

/** A client published a model catalog and explicitly does not use account sign-in. */
export function catalogWithoutAccountAuth(connection){
 const auth=connection?.auth;
 return auth?.state==='known'
  &&auth.value==='unchecked'
  &&auth.freshness==='fresh'
  &&String(auth.source?.reference??'').includes('account sign-in is not required');
}

/** Keep real subscription/API routes behind fresh provider authentication evidence. */
export function filterLiveCatalog(snapshot){
 if(snapshot?.mode!=='live')return snapshot;
 const connections=new Map((snapshot.connections??[]).map(connection=>[connection.id,connection]));
 const routes=new Map((snapshot.routes??[]).map(route=>[route.id,route]));
 const models=(snapshot.models??[]).filter(model=>{
  const route=routes.get(model.routeId),connection=connections.get(route?.connectionId);
  return freshAuthentication(connection)||localRuntime(connection,route)||catalogWithoutAccountAuth(connection);
 });
 const visibleRoutes=new Set(models.map(model=>model.routeId));
 const visibleConnectionIds=new Set([...visibleRoutes].map(routeId=>routes.get(routeId)?.connectionId).filter(Boolean));
 return {
  ...snapshot,
  models,
  routes:(snapshot.routes??[]).filter(route=>visibleRoutes.has(route.id)),
  quotaPools:(snapshot.quotaPools??[]).map(pool=>({
   ...pool,
   connectionIds:pool.connectionIds.filter(id=>visibleConnectionIds.has(id)),
   routeIds:pool.routeIds.filter(id=>visibleRoutes.has(id)),
  })).filter(pool=>pool.connectionIds.length&&pool.routeIds.length),
 };
}
