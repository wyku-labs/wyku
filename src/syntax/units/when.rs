use crate::syntax::Tool;
use crate::syntax::Unit;

#[derive(Debug)]
enum Subject {
    Unit(Box<Unit>),
    Tool(Tool),
}

#[derive(Debug)]
pub(super) enum Expression {
    Subject(Subject),

    NOT(Box<Expression>),

    AND([Box<Expression>; 2]),
    OR([Box<Expression>; 2]),
    XOR([Box<Expression>; 2]),
}
