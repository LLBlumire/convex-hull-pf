//! Provides the Hull struct.

use shape::segment::Segment;

/// Represents a Convex Hull
#[derive(Serialize, Deserialize, Debug)]
pub struct Hull {
    /// The segments that constitute a hull.
    pub segment_set: Vec<Segment>,
}

impl Hull {
    /// Constructs a hull from it's segements.
    pub fn from_segment_set(segment_set: Vec<Segment>) -> Hull {
        Hull { segment_set }
    }
}
