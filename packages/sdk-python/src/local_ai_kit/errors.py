"""Sanitized error codes: native logs and provider response bodies never enter errors."""
import re


class LocalAIError(Exception):
    def __init__(self, code: str):
        self.code = code if re.fullmatch(r'[A-Z][A-Z0-9_]{0,63}', code) else 'PROTOCOL_INVALID'
        super().__init__(f'Local AI request failed ({self.code})')
