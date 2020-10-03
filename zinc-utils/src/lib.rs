//!
//! The Zinc common utilities.
//!

pub(crate) mod array;
pub(crate) mod bigint;
pub(crate) mod euclidean;
pub(crate) mod inference;
pub(crate) mod logger;
pub(crate) mod math;

pub use crate::array::eth_address_from_vec;
pub use crate::array::eth_private_key_from_vec;
pub use crate::bigint::from_str as bigint_from_str;
pub use crate::euclidean::div_rem as euclidean_div_rem;
pub use crate::inference::error::Error as InferenceError;
pub use crate::inference::literal_types as infer_literal_types;
pub use crate::inference::minimal_bitlength as infer_minimal_bitlength;
pub use crate::inference::r#type::Type as InferredType;
pub use crate::inference::result::Binary as BinaryInferenceResult;
pub use crate::logger::initialize as initialize_logger;
pub use crate::math::floor_to_power_of_two;
pub use crate::math::log2ceil;
