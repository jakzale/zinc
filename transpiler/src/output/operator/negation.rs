//!
//! Transpiler output negation operator.
//!

use crate::element::Element;

pub struct Output {}

impl Output {
    pub fn output(identifier: String, namespace: String, operand: Element) -> String {
        format!(
            r#"let {0} = r1cs::negate(system.namespace(|| {1}), &{2}, 254)?.0;"#,
            identifier, namespace, operand,
        )
    }
}
