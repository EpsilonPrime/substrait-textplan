#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""
Substrait TextPlan Python bindings.

This package provides Python bindings for the Substrait TextPlan library,
which allows parsing and converting between Substrait textplan format and
binary protobuf format.
"""

from substrait_textplan._lib import TextPlan, load_from_text, load_from_binary

__version__ = "0.1.0"
__all__ = ["TextPlan", "load_from_text", "load_from_binary"]
