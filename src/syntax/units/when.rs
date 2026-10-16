use crate::syntax::Id;

#[derive(Debug)]
enum Subject {
    Unit(Id, State),
    Tool(Id),
}

#[derive(Debug)]
pub(super) enum State {
    Start,
    Finish,
    Fail,
    Watch,
}

#[derive(Debug)]
pub(super) enum Expression {
    Subject(Subject),

    NOT(Box<Expression>),

    AND([Box<Expression>; 2]),
    OR([Box<Expression>; 2]),
    XOR([Box<Expression>; 2]),
}

impl Expression {
    pub(super) fn parser(when: &str) -> Result<Expression, String> {
        let id = Id::new(when)?;
        Ok(Expression::Subject(Subject::Unit(id, State::Start))) // TODO: To work
    }
}

impl State {
    pub(super) fn parse(state: &str) -> Result<Self, String> {
        match state {
            "start" => Ok(Self::Start),
            "finish" => Ok(Self::Finish),
            "fail" => Ok(Self::Fail),
            "watch" => Ok(Self::Watch),
            _ => Err(format!(
                "State expected 'start' 'fìnish' 'fail' 'watch', got '{state}'"
            )),
        }
    }
}
