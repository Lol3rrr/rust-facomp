use std::collections::BTreeMap;

// Structure
//
// Local-Variables:
// Local variables are stored on the stack
// reference: https://bob.cs.sonoma.edu/IntroCompOrg-RPi/sec-varstack.html

pub mod archs;
pub mod traits;

/// Stores the Stack-Offset for every variable
/// in the current Scope
pub type VariableOffsets = BTreeMap<String, i64>;
