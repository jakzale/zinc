//!
//! The semantic analyzer standard library `std::crypto::schnorr::Signature::verify` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::BuiltinIdentifier;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::scope::builtin::BuiltInTypeId;

#[derive(Debug, Clone)]
pub struct Function {
    pub location: Option<Location>,
    pub builtin_identifier: BuiltinIdentifier,
    pub identifier: &'static str,
    pub return_type: Box<Type>,
}

impl Function {
    pub const ARGUMENT_INDEX_SIGNATURE: usize = 0;
    pub const ARGUMENT_INDEX_MESSAGE: usize = 1;
    pub const ARGUMENT_COUNT: usize = 2;

    pub fn new(builtin_identifier: BuiltinIdentifier) -> Self {
        Self {
            location: None,
            builtin_identifier,
            identifier: "verify",
            return_type: Box::new(Type::boolean(None)),
        }
    }

    pub fn call(
        self,
        location: Option<Location>,
        actual_elements: Vec<Element>,
    ) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let location = element.location();

            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::ArgumentNotEvaluable {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        match actual_params.get(Self::ARGUMENT_INDEX_SIGNATURE) {
            Some((Type::Structure(structure), _location))
                if structure.type_id == BuiltInTypeId::StdCryptoSchnorrSignature as usize => {}
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "signature".to_owned(),
                    position: Self::ARGUMENT_INDEX_SIGNATURE + 1,
                    expected: "std::crypto::schnorr::Signature { r: std::crypto::ecc::Point, s: field, pk: std::crypto::ecc::Point }".to_owned(),
                    found: r#type.to_string(),
                })
            },
            None => {
                return Err(Error::ArgumentCount {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                })
            }
        }

        match actual_params.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some((Type::Array(array), location)) => match (array.r#type.deref(), array.size) {
                (Type::Boolean(_), size)
                    if size % crate::BITLENGTH_BYTE == 0
                        && size > 0
                        && size <= crate::LIMIT_SCHNORR_MESSAGE_BITS => {}
                (r#type, size) => {
                    return Err(Error::ArgumentType {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        name: "message".to_owned(),
                        position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                        expected: format!(
                            "[bool; N], 0 < N <= {}, N % {} == 0",
                            crate::BITLENGTH_INTEGER_MAX,
                            crate::BITLENGTH_BYTE
                        ),
                        found: format!("array [{}; {}]", r#type, size),
                    });
                }
            },
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "message".to_owned(),
                    position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                    expected: format!(
                        "[bool; N], 0 < N <= {}, N % {} == 0",
                        crate::BITLENGTH_INTEGER_MAX,
                        crate::BITLENGTH_BYTE
                    ),
                    found: r#type.to_string(),
                });
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                });
            }
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount {
                location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT,
                found: actual_params.len(),
            });
        }

        Ok(*self.return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "crypto::schnorr::{}(signature: std::crypto::schnorr::Signature, message: [bool; N]) -> bool", self.identifier)
    }
}