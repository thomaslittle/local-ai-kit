"""API documentation must reject contract drift and invalid examples, without Rust builds."""
import copy
import ast
import importlib.util
import pathlib
import tempfile
import unittest
from unittest import mock

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('api_reference', ROOT / 'scripts/generate_api_reference.py')
api = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(api)

SCHEMA = {'$defs': {
    'Params': {'type': 'object', 'additionalProperties': False,
               'properties': {'limit': {'type': 'integer', 'minimum': 1}}, 'required': ['limit']},
    'Result': {'type': 'object', 'additionalProperties': False,
               'properties': {'count': {'type': 'string'}}, 'required': ['count']},
    'ErrorCode': {'enum': ['INVALID_REQUEST']},
    'MethodInputs': {'properties': {'models.example': {'$ref': '#/$defs/Params'}}},
    'MethodOutputs': {'properties': {'models.example': {'$ref': '#/$defs/Result'}}},
}}
MANIFEST = {'protocolVersion': '1.0', '$defs': SCHEMA['$defs'], 'methods': [
    {'name': 'models.example', 'paramsSchema': {'$ref': '#/$defs/Params'},
     'resultSchema': {'$ref': '#/$defs/Result'}}]}
CONTENT = {'methods': {'models.example': {
    'summary': 'Read a fictional count.', 'authority': 'Scoped read.',
    'params': {'limit': 1}, 'result': {'count': '3'}, 'notes': [],
    'errors': [{'code': 'INVALID_REQUEST', 'when': 'Invalid limit.'}],
}}, 'schemaExamples': {}, 'runtimeExtensions': {}}


class ApiReferenceTests(unittest.TestCase):
    def test_valid_examples_use_wire_string_counts(self):
        api.validate_examples(SCHEMA, MANIFEST, CONTENT)

    def test_wrong_request_type_is_rejected(self):
        bad = copy.deepcopy(CONTENT)
        bad['methods']['models.example']['params']['limit'] = '1'
        with self.assertRaisesRegex(ValueError, 'models.example.*params'):
            api.validate_examples(SCHEMA, MANIFEST, bad)

    def test_unknown_request_fields_are_rejected(self):
        bad = copy.deepcopy(CONTENT)
        bad['methods']['models.example']['params']['clientId'] = 'cannot-self-authorize'
        with self.assertRaises(ValueError):
            api.validate_examples(SCHEMA, MANIFEST, bad)

    def test_wrong_result_type_is_rejected(self):
        bad = copy.deepcopy(CONTENT)
        bad['methods']['models.example']['result']['count'] = 3
        with self.assertRaisesRegex(ValueError, 'models.example.*result'):
            api.validate_examples(SCHEMA, MANIFEST, bad)

    def test_new_or_removed_methods_cannot_silently_lose_documentation(self):
        bad = copy.deepcopy(CONTENT)
        bad['methods']['imaginary.method'] = copy.deepcopy(bad['methods']['models.example'])
        with self.assertRaisesRegex(ValueError, 'coverage'):
            api.validate_examples(SCHEMA, MANIFEST, bad)
        bad = copy.deepcopy(CONTENT)
        del bad['methods']['models.example']
        with self.assertRaisesRegex(ValueError, 'coverage'):
            api.validate_examples(SCHEMA, MANIFEST, bad)

    def test_undocumented_error_code_is_rejected(self):
        bad = copy.deepcopy(CONTENT)
        bad['methods']['models.example']['errors'][0]['code'] = 'MADE_UP'
        with self.assertRaisesRegex(ValueError, 'error code'):
            api.validate_examples(SCHEMA, MANIFEST, bad)

    def test_external_schema_references_cannot_trigger_network_access(self):
        bad = copy.deepcopy(SCHEMA)
        bad['$defs']['Result'] = {'$ref': 'https://example.invalid/schema'}
        with self.assertRaisesRegex(ValueError, 'local'):
            api.validate_examples(bad, MANIFEST, CONTENT)

    def test_unrestricted_schema_is_not_documented_as_literal_true(self):
        self.assertIn('Any JSON value', api.type_text(True))

    def test_unrestricted_property_is_rendered_without_fabricating_a_type(self):
        rendered = '\n'.join(api.table({'properties': {'data': True}}))
        self.assertIn('Any JSON value', rendered)

    def test_http_responses_include_inline_fields_and_envelope_constraints(self):
        rendered = api.render_files(*api.load(ROOT))['docs/reference/http.md']
        for field in ('token', 'protocolVersion', 'expiresInSeconds', 'jsonrpc', 'result', 'error'):
            self.assertIn('| `' + field + '` |', rendered)
        self.assertIn('Must not satisfy', rendered)

    def test_composition_does_not_hide_shared_object_fields(self):
        envelope = {'type': 'object', 'properties': {'result': {}},
                    'oneOf': [{'required': ['result']}, {'required': ['error']}]}
        self.assertIn('`object`', api.type_text(envelope))
        self.assertNotIn('unrestricted', api.type_text(envelope))

    def test_guide_examples_cannot_invent_methods_or_request_fields(self):
        for request in ('{"jsonrpc":"2.0","id":1,"method":"imaginary.call","params":{}}',
                        '{"jsonrpc":"2.0","id":1,"method":"models.example","params":{"limit":"wrong"}}'):
            with self.assertRaises(ValueError):
                api.validate_guide_examples(SCHEMA, MANIFEST, '```json\n' + request + '\n```')

    def test_anchor_checker_finds_missing_fragment_not_just_missing_file(self):
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            (root / 'a.md').write_text('[valid](b.md#safe)\n[broken](b.md#missing)', encoding='utf-8')
            (root / 'b.md').write_text('<a id="safe"></a>\n# B', encoding='utf-8')
            failures = api.check_links(root, ['a.md', 'b.md'])
            self.assertEqual(len(failures), 1)
            self.assertIn('missing', failures[0])

    def test_current_reference_and_every_example_are_checked(self):
        self.assertEqual(api.check(ROOT), [])

    def test_quickstart_requests_match_the_canonical_contract(self):
        schema, manifest, _, _ = api.load(ROOT)
        methods = {item['name']: item for item in manifest['methods']}
        tree = ast.parse((ROOT / 'examples/python/api_quickstart.py').read_text(encoding='utf-8'))
        checked = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == 'request':
                method = ast.literal_eval(node.args[0])
                params = ast.literal_eval(node.args[1])
                api.validate_value(schema, methods[method]['paramsSchema'], params, 'Quickstart ' + method)
                checked.append(method)
        self.assertEqual(set(checked), {'runtime.handshake', 'discovery.refresh', 'snapshot.get', 'models.page'})

    def test_quickstart_import_does_not_start_processes_or_create_homes(self):
        spec = importlib.util.spec_from_file_location('quickstart', ROOT / 'examples/python/api_quickstart.py')
        module = importlib.util.module_from_spec(spec)
        with mock.patch('subprocess.Popen', side_effect=AssertionError('spawn during import')), \
                mock.patch('tempfile.TemporaryDirectory', side_effect=AssertionError('home during import')):
            spec.loader.exec_module(module)


if __name__ == '__main__':
    unittest.main()
