// SPDX-License-Identifier: Apache-2.0

//! The location module provides a way to track source positions in the parsed text.

use std::fmt;

/// Represents a position in the source text.
/// 
/// This is used to provide context for errors and to reference entities in the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    /// The position in the source text, 0-indexed.
    position: i32,
    /// The length of the text at this location.
    length: i32,
}

impl Location {
    /// A special location used to indicate an unknown position.
    pub const UNKNOWN_LOCATION: Location = Location { position: -1, length: 0 };

    /// Creates a new location from a position and length.
    pub fn new(position: i32, length: i32) -> Self {
        Self { position, length }
    }

    /// Returns true if this is the unknown location.
    pub fn is_unknown(&self) -> bool {
        self.position < 0
    }

    /// Returns the position in the source text.
    pub fn position(&self) -> i32 {
        self.position
    }

    /// Returns the length of the text at this location.
    pub fn length(&self) -> i32 {
        self.length
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_unknown() {
            write!(f, "unknown")
        } else {
            write!(f, "pos {} len {}", self.position, self.length)
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::UNKNOWN_LOCATION
    }
}