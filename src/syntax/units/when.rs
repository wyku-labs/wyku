use crate::syntax::Id;

#[derive(Debug)]
enum Subject {
    Unit(Id),
    Tool(Id),
}

#[derive(Debug)]
pub(super) enum Expression {
    Subject(Subject),

    NOT(Box<Expression>),

    AND([Box<Expression>; 2]),
    OR([Box<Expression>; 2]),
    XOR([Box<Expression>; 2]),
}
