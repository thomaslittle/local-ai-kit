import test from 'node:test';
import assert from 'node:assert/strict';
import {chmodSync, mkdtempSync, rmSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {defaultAddress, defaultTokenFile, platformPackage, resolveHelper} from '../node/auto.mjs';

function helper() {
  const directory = mkdtempSync(join(tmpdir(), 'lai-helper-'));
  const binary = join(directory, 'local-ai-kit');
  writeFileSync(binary, '#!/bin/sh\n');
  chmodSync(binary, 0o755);
  return {directory, binary};
}

test('platform packages mirror esbuild and Biome', () => {
  assert.equal(platformPackage('linux', 'x64'), '@local-ai-kit/cli-linux-x64');
  assert.equal(platformPackage('linux', 'arm64'), '@local-ai-kit/cli-linux-arm64');
  assert.equal(platformPackage('darwin', 'arm64'), '@local-ai-kit/cli-darwin-arm64');
  assert.equal(platformPackage('darwin', 'x64'), '@local-ai-kit/cli-darwin-x64');
  assert.equal(platformPackage('win32', 'x64'), '@local-ai-kit/cli-win32-x64');
  assert.equal(platformPackage('freebsd', 'x64'), undefined);
});

test('LAI_HELPER_PATH overrides packaged resolution', () => {
  const {directory, binary} = helper();
  const previous = process.env.LAI_HELPER_PATH;
  process.env.LAI_HELPER_PATH = binary;
  try {
    assert.equal(resolveHelper(), binary);
  } finally {
    if (previous === undefined) delete process.env.LAI_HELPER_PATH;
    else process.env.LAI_HELPER_PATH = previous;
    rmSync(directory, {recursive: true, force: true});
  }
});

test('a bad override fails loudly instead of falling back', () => {
  const previous = process.env.LAI_HELPER_PATH;
  process.env.LAI_HELPER_PATH = '/nonexistent/local-ai-kit';
  try {
    assert.throws(() => resolveHelper(), /INVALID_EXECUTABLE/);
  } finally {
    if (previous === undefined) delete process.env.LAI_HELPER_PATH;
    else process.env.LAI_HELPER_PATH = previous;
  }
  assert.throws(() => resolveHelper({executable: 'relative/helper'}), /INVALID_EXECUTABLE/);
});

test('no binary and no override is a runtime-missing error', () => {
  const previous = process.env.LAI_HELPER_PATH;
  delete process.env.LAI_HELPER_PATH;
  try {
    assert.throws(() => resolveHelper(), /RUNTIME_MISSING/);
  } finally {
    if (previous !== undefined) process.env.LAI_HELPER_PATH = previous;
  }
});

test('shared broker defaults follow the environment', () => {
  const previous = {
    address: process.env.LAI_BROKER_ADDRESS,
    token: process.env.LAI_BROKER_TOKEN_FILE,
  };
  process.env.LAI_BROKER_ADDRESS = '/tmp/custom.sock';
  process.env.LAI_BROKER_TOKEN_FILE = '/tmp/custom-token.json';
  try {
    assert.equal(defaultAddress(), '/tmp/custom.sock');
    assert.equal(defaultTokenFile(), '/tmp/custom-token.json');
  } finally {
    if (previous.address === undefined) delete process.env.LAI_BROKER_ADDRESS;
    else process.env.LAI_BROKER_ADDRESS = previous.address;
    if (previous.token === undefined) delete process.env.LAI_BROKER_TOKEN_FILE;
    else process.env.LAI_BROKER_TOKEN_FILE = previous.token;
  }
  assert.match(defaultTokenFile(), /broker-token\.json$/);
});
