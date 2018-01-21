//! Provides the Output struct.

use shape::hull::Hull;
use io::input::Input;

/// The Output of computation.
#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    /// The input that generated this output, if known
    pub input: Input,

    /// The point to point hulls that make up the outputs along the path.
    pub hulls: Vec<Hull>,
}
