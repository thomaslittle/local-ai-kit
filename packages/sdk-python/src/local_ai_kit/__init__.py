"""Local AI Kit protocol SDK. Importing performs no discovery or execution."""
from .errors import LocalAIError
from .stdio import AsyncStdioTransport
from .client import AsyncClient, Client
__all__ = ['LocalAIError', 'AsyncStdioTransport', 'AsyncClient', 'Client']

from .bundle import resolve_bundle_binary
__all__.append("resolve_bundle_binary")
