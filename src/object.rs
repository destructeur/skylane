// Copyright 2016-2017 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Definitions of `Object` and `ObjectId`.

use std;

use defs::{Header, SkylaneError, Task};
use bundle::Bundle;

// -------------------------------------------------------------------------------------------------

/// Structure representing ID of protocol object.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId(u32);

impl ObjectId {
    /// Constructs new `ObjectId`.
    pub fn new(value: u32) -> Self {
        ObjectId(value)
    }

    /// Returns numerical value of objects ID.
    pub fn get_value(&self) -> u32 {
        self.0
    }

    /// Returns new `ObjectId` incremented by `1`.
    pub fn incremented(&self) -> ObjectId {
        ObjectId(self.0 + 1)
    }

    /// Checks if object ID is valid.
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

impl std::fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.get_value())
    }
}

/// Default ID of main global object.
pub const DISPLAY_ID: ObjectId = ObjectId(1);

// -------------------------------------------------------------------------------------------------

/// This trait has to be implemented by all objects to be registered as message handlers in
/// `Connection`.
pub trait Object {
    /// Informs implementation about incoming message.
    ///
    /// - `bundle` provides access to socket and registered objects.
    /// - `header` defines what method was called for what objects.
    /// - `bytes_buf` contains raw message without header.
    /// - `fds_buf` contains received file descriptors.
    fn dispatch(&mut self,
                bundle: &mut Bundle,
                header: &Header,
                bytes_buf: &mut std::io::Cursor<&[u8]>,
                fds_buf: &mut std::io::Cursor<&[u8]>)
                -> Result<Task, SkylaneError>;
}

// -------------------------------------------------------------------------------------------------
