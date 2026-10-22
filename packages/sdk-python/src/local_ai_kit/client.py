"""Async-first client and a small synchronous facade over the same implementation."""
import asyncio
import re
from typing import Any
from .errors import LocalAIError
from .stdio import AsyncStdioTransport

FAMILIES = {
    'runtime': ('health','closeClient'), 'snapshot': ('get',), 'discovery': ('refresh','watch','cancel'),
    'connections': ('list','setupInstructions','authenticate','probe'), 'models': ('list','page'),
    'usage': ('read',), 'capabilities': ('read',), 'selection': ('resolve','validate'),
    'grants': ('request','list','revoke'), 'sessions': ('create','get','suspend','resume','updateOptions','close'),
    'health': ('get',), 'issues': ('list','acknowledge','snooze'), 'notifications': ('poll','configure','ack'), 'apps': ('list','revoke'),
    'inputs': ('put',), 'turns': ('start','cancel'), 'approvals': ('respond',), 'artifacts': ('read',),
    'diagnostics': ('report',), 'adapters': ('list',), 'events': ('subscribe',), 'inventory': ('subscribe',),
}


class _Namespace:
    def __init__(self, client, name):
        self._client, self._name = client, name

    def __getattr__(self, name):
        if name not in FAMILIES[self._name]:
            raise AttributeError(name)
        if self._name == 'events' and name == 'subscribe':
            return self._client.subscribe
        if self._name == 'inventory' and name == 'subscribe':
            return self._client.subscribe_inventory
        def call(params=None, **options):
            return self._client.request(f'{self._name}.{name}', params, **options)
        return call


class _ChatgptNamespace:
    def __init__(self, client):
        self._client = client

    def __getattr__(self, name):
        if name not in ('status', 'start', 'cancel', 'disconnect'):
            raise AttributeError(name)
        def call(params=None, **options):
            return self._client.request(f'auth.chatgpt.{name}', params, **options)
        return call


class _AuthNamespace:
    def __init__(self, client):
        self.chatgpt = _ChatgptNamespace(client)


class AsyncClient:
    @classmethod
    async def connect(cls, transport):
        try:
            handshake = await transport.request('runtime.handshake', {'protocolVersion':'1.0','clientName':'local-ai-kit-python'})
            if not isinstance(handshake, dict) or not isinstance(handshake.get('protocolVersion'), str) or not re.fullmatch(r'1\.\d+', handshake['protocolVersion']):
                raise LocalAIError('PROTOCOL_INCOMPATIBLE')
            methods = handshake.get('methods')
            if not isinstance(methods, list) or len(methods) > 512 or any(not isinstance(m, str) for m in methods):
                raise LocalAIError('PROTOCOL_INCOMPATIBLE')
        except BaseException:
            await transport.close()
            raise
        self = cls()
        self._transport, self._methods, self._closed = transport, frozenset(methods), False
        self.handshake = {**handshake, 'methods':tuple(methods)}
        return self

    def __getattr__(self, name):
        if name == 'auth':
            return _AuthNamespace(self)
        if name not in FAMILIES:
            raise AttributeError(name)
        return _Namespace(self, name)

    def _check(self, method):
        if self._closed:
            raise LocalAIError('CLOSED')
        if method not in self._methods:
            raise LocalAIError('CAPABILITY_UNSUPPORTED')

    async def request(self, method: str, params: Any = None, **options) -> Any:
        self._check(method)
        return await self._transport.request(method, params, **options)

    async def subscribe(self, query=None):
        self._check('events.subscribe')
        async for event in self._transport.subscribe(query):
            yield event

    async def subscribe_inventory(self, after=None):
        self._check('events.subscribe')
        query = {'scope': 'inventory'}
        if after is not None:
            query['after'] = after
        async for event in self._transport.subscribe(query):
            yield event

    async def close(self):
        if not self._closed:
            self._closed = True
            try:
                if 'runtime.closeClient' in self._methods:
                    await self._transport.request('runtime.closeClient', {}, timeout=1)
            except Exception:
                pass  # Remote cleanup cannot prevent local shutdown or expose error bodies.
            finally:
                await self._transport.close()

    async def __aenter__(self):
        return self

    async def __aexit__(self, *_args):
        await self.close()


class Client:
    @classmethod
    def from_stdio(cls, **options):
        try:
            asyncio.get_running_loop()
        except RuntimeError:
            pass
        else:
            raise LocalAIError('USE_ASYNC_CLIENT')
        runner = asyncio.Runner()
        async def connect():
            return await AsyncClient.connect(await AsyncStdioTransport.open(**options))
        try:
            client = runner.run(connect())
        except BaseException:
            runner.close()
            raise
        self = cls()
        self._runner, self._client, self._closed = runner, client, False
        self.handshake = client.handshake
        return self

    def __getattr__(self, name):
        if name == 'auth':
            return _AuthNamespace(self)
        if name not in FAMILIES:
            raise AttributeError(name)
        return _Namespace(self, name)

    def request(self, method, params=None, **options):
        if self._closed:
            raise LocalAIError('CLOSED')
        return self._runner.run(self._client.request(method, params, **options))

    def subscribe(self, query=None):
        if self._closed:
            raise LocalAIError('CLOSED')
        iterator = self._client.subscribe(query)
        try:
            while not self._closed:
                try:
                    yield self._runner.run(anext(iterator))
                except StopAsyncIteration:
                    break
        finally:
            if not self._closed:
                self._runner.run(iterator.aclose())

    def subscribe_inventory(self, after=None):
        if self._closed:
            raise LocalAIError('CLOSED')
        iterator = self._client.subscribe_inventory(after)
        try:
            while not self._closed:
                try:
                    yield self._runner.run(anext(iterator))
                except StopAsyncIteration:
                    break
        finally:
            if not self._closed:
                self._runner.run(iterator.aclose())

    def close(self):
        if not self._closed:
            self._closed = True
            try:
                self._runner.run(self._client.close())
            finally:
                self._runner.close()

    def __enter__(self):
        return self

    def __exit__(self, *_args):
        self.close()
