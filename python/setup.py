#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""
Setup script for the substrait-textplan Python package.
"""

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    rust_extensions=[
        RustExtension(
            "substrait_textplan._lib",
            path="../Cargo.toml",
            binding=Binding.NoBinding,  # We use ctypes, not PyO3
            strip=False,  # Keep symbols for ctypes to find functions
        )
    ],
    # rust-extensions are built in-place, so the package will include the .so/.dylib/.dll
    zip_safe=False,
)
