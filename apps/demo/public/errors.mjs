const messages={
 LOCAL_ACCESS_DENIED:'The host has not allowed metadata checks for this provider.',
 CAPABILITY_UNSUPPORTED:'This provider cannot perform that metadata check in the current runtime.',
 RUNTIME_MISSING:'A compatible native provider runtime is not installed or registered.',
 INTEGRATION_RESTRICTED:'This provider integration is not configured for this profile.',
 PROVIDER_UNAVAILABLE:'The provider runtime is unavailable. Try the metadata check again later.',
 AUTH_REQUIRED:'This profile needs authentication in its native provider client.',
};

export function userFacingError(error){
 return messages[error?.code]??'The request could not be completed.';
}
