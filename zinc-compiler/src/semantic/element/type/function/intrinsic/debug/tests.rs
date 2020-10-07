//!
//! The `dbg!` intrinsic function tests.
//!

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::intrinsic::debug::error::Error as DebugFunctionError;
use crate::semantic::element::r#type::function::intrinsic::debug::Function as DebugFunction;
use crate::semantic::element::r#type::function::intrinsic::error::Error as IntrinsicFunctionError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_argument_count_lesser() {
    let input = r#"
fn main() {
    dbg!("{} {}", 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::Debug(
            DebugFunctionError::ArgumentCount {
                location: Location::test(3, 5),
                expected: 3,
                found: 2,
            },
        ))),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_count_greater() {
    let input = r#"
fn main() {
    dbg!("{}", 42, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::Debug(
            DebugFunctionError::ArgumentCount {
                location: Location::test(3, 5),
                expected: 2,
                found: 3,
            },
        ))),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_1_format_expected_string() {
    let input = r#"
fn main() {
    dbg!(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 10),
            function: DebugFunction::IDENTIFIER.to_owned(),
            name: "format".to_owned(),
            position: DebugFunction::ARGUMENT_INDEX_FORMAT + 1,
            expected: Type::string(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
