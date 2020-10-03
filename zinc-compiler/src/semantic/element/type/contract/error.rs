//!
//! The semantic analyzer contract type element error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer contract type element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// A field with the same name occurs for the second time.
    DuplicateField {
        /// The duplicate field location.
        location: Location,
        /// The contract type name.
        type_identifier: String,
        /// The duplicate field name.
        field_name: String,
    },
}
