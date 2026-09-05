//! Where in the configuration something went wrong.
//!
//! A host configuration is a nested object assembled from manifests, so
//! "unknown attribute type" is not an actionable message and
//! `tags.callout.attributes.type.type` is. Every error this crate reports about
//! a configuration is prefixed with the path to the value that caused it.

use std::fmt;

/// A dotted path into the configuration object.
#[derive(Clone)]
pub(crate) struct Path(String);

impl Path {
    /// The root of the configuration.
    pub(crate) fn root() -> Path {
        Path(String::from("config"))
    }

    /// The path to a named property of this one.
    pub(crate) fn child(&self, key: &str) -> Path {
        Path(format!("{}.{key}", self.0))
    }

    /// The path to an element of this one.
    pub(crate) fn index(&self, index: u32) -> Path {
        Path(format!("{}[{index}]", self.0))
    }
}

impl fmt::Display for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
