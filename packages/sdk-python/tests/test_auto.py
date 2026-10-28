import os
import pathlib
import stat
import sys
import tempfile
import unittest
from unittest.mock import patch

import importlib.util

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'src'))
from local_ai_kit import LocalAIError, default_address, default_token_file, resolve_helper  # noqa: E402
import local_ai_kit.client as client_module  # noqa: E402


class HelperResolutionTests(unittest.TestCase):
    def test_explicit_helper_wins(self):
        with tempfile.TemporaryDirectory() as directory:
            binary = pathlib.Path(directory) / 'local-ai-kit'
            binary.write_text('#!/bin/sh\n')
            binary.chmod(binary.stat().st_mode | stat.S_IXUSR)
            self.assertEqual(resolve_helper(str(binary)), str(binary))

    def test_invalid_explicit_helper_is_rejected(self):
        with self.assertRaises(LocalAIError) as caught:
            resolve_helper('relative/helper')
        self.assertEqual(caught.exception.code, 'INVALID_EXECUTABLE')
        with self.assertRaises(LocalAIError) as caught:
            resolve_helper('/bin/sh.cmd')
        self.assertEqual(caught.exception.code, 'INVALID_EXECUTABLE')

    def test_environment_override(self):
        with tempfile.TemporaryDirectory() as directory:
            binary = pathlib.Path(directory) / 'local-ai-kit'
            binary.write_text('#!/bin/sh\n')
            binary.chmod(binary.stat().st_mode | stat.S_IXUSR)
            with patch.dict(os.environ, {'LAI_HELPER_PATH': str(binary)}):
                self.assertEqual(resolve_helper(), str(binary))
            with patch.dict(os.environ, {'LAI_HELPER_PATH': str(binary) + '.missing'}):
                with self.assertRaises(LocalAIError) as caught:
                    resolve_helper()
                self.assertEqual(caught.exception.code, 'INVALID_EXECUTABLE')

    def test_missing_bundled_binary_is_runtime_missing(self):
        with patch.dict(os.environ, {}, clear=True):
            with self.assertRaises(LocalAIError) as caught:
                resolve_helper()
            self.assertEqual(caught.exception.code, 'RUNTIME_MISSING')

    def test_defaults_follow_environment(self):
        with patch.dict(os.environ, {'LAI_BROKER_ADDRESS': '/tmp/x.sock', 'LAI_BROKER_TOKEN_FILE': '/tmp/t.json'}):
            self.assertEqual(default_address(), '/tmp/x.sock')
            self.assertEqual(default_token_file(), pathlib.Path('/tmp/t.json'))
        with patch.dict(os.environ, {}, clear=True):
            self.assertTrue(default_address().endswith('user.sock') or default_address() == 'local-ai-kit-user')
            self.assertTrue(str(default_token_file()).endswith('broker-token.json'))

    def test_shared_token_rejects_group_readable_descriptor(self):
        with tempfile.TemporaryDirectory() as directory:
            path = pathlib.Path(directory) / 'broker-token.json'
            path.write_text('{"address":"/tmp/x.sock","token":"' + 'a' * 64 + '"}')
            path.chmod(0o644)
            self.assertIsNone(client_module._shared_token('/tmp/x.sock', path))
            if os.name != 'nt':
                path.chmod(0o600)
                self.assertEqual(client_module._shared_token('/tmp/x.sock', path), 'a' * 64)
                self.assertIsNone(client_module._shared_token('/tmp/other.sock', path))


if __name__ == '__main__':
    unittest.main()
