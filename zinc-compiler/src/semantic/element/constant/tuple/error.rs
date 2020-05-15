//!
//! The semantic analyzer constant tuple element error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldDoesNotExist {
        location: Location,
        type_identifier: String,
        field_index: usize,
    },
}
