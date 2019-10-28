//!
//! The tuple expression.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<syntax::Expression>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<syntax::Expression>) -> Self {
        Self { location, elements }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.elements
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
