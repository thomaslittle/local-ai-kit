import {test} from 'node:test';
import assert from 'node:assert/strict';
import {userFacingError} from '../apps/demo/public/errors.mjs';

test('provider capability failures are readable and never expose raw codes',()=>{
 assert.equal(userFacingError({code:'CAPABILITY_UNSUPPORTED'}),'This provider cannot perform that metadata check in the current runtime.');
 assert.equal(userFacingError({code:'RUNTIME_MISSING'}),'A compatible native provider runtime is not installed or registered.');
 assert.equal(userFacingError({code:'LOCAL_ACCESS_DENIED'}),'The host has not allowed metadata checks for this provider.');
 assert.equal(userFacingError({code:'SOME_NEW_INTERNAL_CODE'}),'The request could not be completed.');
});
