"""Fail on generated public contract drift; never edit files during verification."""
from generate_contracts import main
import sys
if __name__ == '__main__':
    sys.argv = [sys.argv[0], '--check']
    raise SystemExit(main())
