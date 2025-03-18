#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""
Python wrapper for the Substrait TextPlan library.
"""

import ctypes
import os
import platform
from typing import Union, List, Optional


def _get_lib_path() -> str:
    """Get the path to the shared library."""
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    
    system = platform.system()
    if system == "Linux":
        lib_ext = "so"
    elif system == "Darwin":
        lib_ext = "dylib"
    elif system == "Windows":
        lib_ext = "dll"
    else:
        raise RuntimeError(f"Unsupported platform: {system}")
    
    # Check debug build first
    debug_path = os.path.join(base_dir, "target", "debug", f"libsubstrait_textplan.{lib_ext}")
    if os.path.exists(debug_path):
        return debug_path
    
    # Check release build
    release_path = os.path.join(base_dir, "target", "release", f"libsubstrait_textplan.{lib_ext}")
    if os.path.exists(release_path):
        return release_path
    
    raise RuntimeError("Could not find the Substrait TextPlan shared library")


class TextPlan:
    """Wrapper for the Substrait TextPlan library."""
    
    def __init__(self):
        """Initialize the TextPlan library."""
        lib_path = _get_lib_path()
        self._lib = ctypes.CDLL(lib_path)
        
        # Configure function signatures
        self._lib.load_from_text.argtypes = [ctypes.c_char_p]
        self._lib.load_from_text.restype = ctypes.POINTER(ctypes.c_uint8)
        
        self._lib.free_plan_bytes.argtypes = [ctypes.POINTER(ctypes.c_uint8)]
        self._lib.free_plan_bytes.restype = None
        
        self._lib.load_from_binary.argtypes = [ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t]
        self._lib.load_from_binary.restype = ctypes.c_char_p
        
        self._lib.free_text_plan.argtypes = [ctypes.c_char_p]
        self._lib.free_text_plan.restype = None
    
    def load_from_text(self, text: str) -> Optional[bytes]:
        """
        Load a textplan from a string and convert it to binary protobuf.
        
        Args:
            text: The textplan to load.
            
        Returns:
            The binary protobuf representation of the plan, or None if an error occurred.
        """
        text_bytes = text.encode('utf-8')
        ptr = self._lib.load_from_text(text_bytes)
        
        if not ptr:
            return None
        
        # First sizeof(size_t) bytes contain the length
        len_ptr = ctypes.cast(ptr, ctypes.POINTER(ctypes.c_size_t))
        length = len_ptr.contents.value
        
        # Rest is the data
        data_ptr = ctypes.cast(ctypes.addressof(ptr.contents) + ctypes.sizeof(ctypes.c_size_t), 
                             ctypes.POINTER(ctypes.c_uint8 * length))
        
        # Copy the data
        result = bytes(data_ptr.contents)
        
        # Free the memory
        self._lib.free_plan_bytes(ptr)
        
        return result
    
    def load_from_binary(self, data: bytes) -> Optional[str]:
        """
        Load a binary plan and convert it to textplan format.
        
        Args:
            data: The binary plan to load.
            
        Returns:
            The textplan representation of the plan, or None if an error occurred.
        """
        data_array = (ctypes.c_uint8 * len(data))(*data)
        ptr = self._lib.load_from_binary(data_array, len(data))
        
        if not ptr:
            return None
        
        # Copy the string
        result = ctypes.string_at(ptr).decode('utf-8')
        
        # Free the memory
        self._lib.free_text_plan(ptr)
        
        return result


def load_from_text(text: str) -> Optional[bytes]:
    """
    Load a textplan from a string and convert it to binary protobuf.
    
    Args:
        text: The textplan to load.
        
    Returns:
        The binary protobuf representation of the plan, or None if an error occurred.
    """
    tp = TextPlan()
    return tp.load_from_text(text)


def load_from_binary(data: bytes) -> Optional[str]:
    """
    Load a binary plan and convert it to textplan format.
    
    Args:
        data: The binary plan to load.
        
    Returns:
        The textplan representation of the plan, or None if an error occurred.
    """
    tp = TextPlan()
    return tp.load_from_binary(data)