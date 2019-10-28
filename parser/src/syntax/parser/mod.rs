//!
//! The syntax parser.
//!

mod expression;
mod inputs;
mod statement;
mod r#type;
mod witnesses;

pub use self::expression::AccessOperandParser;
pub use self::expression::AddSubOperandParser;
pub use self::expression::AndOperandParser;
pub use self::expression::ArrayExpressionParser;
pub use self::expression::BlockExpressionParser;
pub use self::expression::CastingOperandParser;
pub use self::expression::ComparisonOperandParser;
pub use self::expression::ConditionalExpressionParser;
pub use self::expression::MulDivRemOperandParser;
pub use self::expression::OrOperandParser;
pub use self::expression::Parser as ExpressionParser;
pub use self::expression::PathExpressionParser;
pub use self::expression::StructureExpressionParser;
pub use self::expression::TupleExpressionParser;
pub use self::expression::MatchExpressionParser;
pub use self::expression::XorOperandParser;
pub use self::inputs::Parser as InputsParser;
pub use self::r#type::Parser as TypeParser;
pub use self::statement::DebugParser as DebugStatementParser;
pub use self::statement::LetParser as LetStatementParser;
pub use self::statement::LoopParser as LoopStatementParser;
pub use self::statement::Parser as StatementParser;
pub use self::statement::RequireParser as RequireStatementParser;
pub use self::witnesses::Parser as WitnessesParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::TokenStream;
use crate::lexical::Token;
use crate::lexical::Lexeme;
use crate::syntax::CircuitProgram;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::Error;

#[derive(Default)]
pub struct Parser {
    next: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, input: String) -> Result<CircuitProgram, Error> {
        let stream = TokenStream::new(input);
        let stream = Rc::new(RefCell::new(stream));

        let inputs = InputsParser::default().parse(stream.clone())?;
        let (witnesses, next) = WitnessesParser::default().parse(stream.clone())?;
        self.next = next;

        let mut statements = Vec::new();
        loop {
            match match self.next.take() {
                Some(token) => token,
                None => stream.borrow_mut().next()?,
            } {
                Token { lexeme: Lexeme::Eof, .. } => break,
                token => {
                    let (statement, next, is_unterminated) =
                        StatementParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    if let Statement::Expression(ref expression) = statement {
                        if is_unterminated {
                            return Err(Error::Syntax(SyntaxError::ExpressionAtRoot(
                                expression.location,
                            )));
                        }
                    }
                    log::trace!("Statement: {:?}", statement);
                    statements.push(statement);
                }
            }
        }

        Ok(CircuitProgram {
            inputs,
            witnesses,
            statements,
        })
    }
}
