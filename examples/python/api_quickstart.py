"""Isolated passive-inventory walkthrough. Importing this module does nothing."""
from __future__ import annotations

import argparse
import json
import pathlib
import queue
import subprocess
import tempfile
import threading

MAX_FRAME = 2 * 1024 * 1024


class WireClient:
    """Small sequential example client, not a replacement for the public SDK."""

    def __init__(self, executable: pathlib.Path, home: pathlib.Path):
        self.frames = queue.Queue(maxsize=2)
        self.next_id = 0
        self.process = subprocess.Popen(
            [str(executable), 'serve', '--stdio', '--home', str(home),
             '--no-metadata-cache', '--allow-passive-discovery'],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
            creationflags=subprocess.CREATE_NO_WINDOW if hasattr(subprocess, 'CREATE_NO_WINDOW') else 0,
        )
        self.reader = threading.Thread(target=self._read, daemon=True)
        self.reader.start()

    def _read(self):
        try:
            while True:
                frame = self.process.stdout.readline(MAX_FRAME + 1)
                if not frame:
                    raise RuntimeError('Helper closed its protocol stream')
                if len(frame) > MAX_FRAME or not frame.endswith(b'\n'):
                    raise RuntimeError('Helper frame exceeded the example limit')
                self.frames.put(json.loads(frame), timeout=1)
        except Exception as error:
            try:
                self.frames.put(error, timeout=1)
            except queue.Full:
                pass

    def request(self, method, params):
        self.next_id += 1
        packet = {'jsonrpc': '2.0', 'id': self.next_id, 'method': method, 'params': params}
        self.process.stdin.write((json.dumps(packet) + '\n').encode())
        self.process.stdin.flush()
        try:
            response = self.frames.get(timeout=15)
        except queue.Empty as error:
            raise RuntimeError('Helper response deadline exceeded') from error
        if isinstance(response, Exception):
            raise response
        if response.get('id') != self.next_id or response.get('jsonrpc') != '2.0':
            raise RuntimeError('Unexpected response envelope')
        if 'error' in response:
            # Do not print unredacted provider payloads or error text.
            code = response['error'].get('data', {}).get('code', 'RPC_ERROR')
            raise RuntimeError('Helper rejected request: ' + code)
        return response['result']

    def close(self):
        self.process.stdin.close()
        try:
            self.process.wait(timeout=3)
        except subprocess.TimeoutExpired:
            self.process.kill()  # Only the exact child this instance started.
            self.process.wait(timeout=3)
        self.reader.join(timeout=2)
        self.process.stdout.close()


def walkthrough(executable: pathlib.Path):
    if not executable.is_absolute() or not executable.is_file():
        raise ValueError('Pass an absolute path to an approved local helper executable')
    with tempfile.TemporaryDirectory(prefix='lai-api-quickstart-') as directory:
        client = WireClient(executable, pathlib.Path(directory))
        try:
            handshake = client.request('runtime.handshake', {
                'protocolVersion': '1.0',
                'clientName': 'api-reference-quickstart',
            })
            if handshake['protocolVersion'] != '1.0':
                raise RuntimeError('This walkthrough requires protocol 1.0')
            required = {'discovery.refresh', 'snapshot.get', 'models.page'}
            if not required.issubset(handshake['methods']):
                raise RuntimeError('Helper does not advertise the walkthrough methods')
            client.request('discovery.refresh', {'level': 'passive'})
            snapshot = client.request('snapshot.get', {})
            page = client.request('models.page', {'limit': 16})
            return {'protocolVersion': handshake['protocolVersion'],
                    'runtimeVersion': handshake['runtimeVersion'], 'mode': snapshot['mode'],
                    'connections': len(snapshot['connections']), 'modelsOnPage': len(page['models'])}
        finally:
            client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('executable', type=pathlib.Path)
    args = parser.parse_args()
    print(json.dumps(walkthrough(args.executable), indent=2))


if __name__ == '__main__':
    main()
