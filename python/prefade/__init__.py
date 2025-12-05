"""High-level Python API for the `prefade` library.

This package re-exports classes and functions from the Rust extension module
`prefade._prefade_native`. Static type information is provided separately
in the accompanying `__init__.pyi` stub file.
"""

from ._prefade_native import Email, NonEmptyStr, send_email  # type: ignore[attr-defined]

__all__ = ["Email", "NonEmptyStr", "send_email"]
