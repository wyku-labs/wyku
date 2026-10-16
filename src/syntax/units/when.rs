use crate::syntax::Id;

#[derive(Debug)]
pub(crate) enum Subject {
    Unit(Id, State),
    Tool(Id),
}

#[derive(Debug)]
pub(crate) enum State {
    Start,
    Finish,
    Fail,
    Watch,
}

#[derive(Debug)]
pub(crate) enum Expression {
    Subject(Subject),

    NOT(Box<Expression>),

    AND([Box<Expression>; 2]),
    OR([Box<Expression>; 2]),
}

impl Expression {
    pub(super) fn parser(when: &str) -> Result<Expression, String> {
        let id = Id::new(when)?;
        Ok(Expression::Subject(Subject::Unit(id, State::Start))) // TODO: To work
    }
}

impl Subject {
    fn parser(subject: &str) -> Result<Subject, String> {
        if subject.starts_with(":") || !subject.contains(":") {
            return Ok(Subject::Tool(Id::new_tool(subject)?));
        }

        if subject.contains(":") {
            if let Some(split) = subject.split_once(":") {
                let (unit, state) = split;
                return Ok(Subject::Unit(Id::new(unit)?, State::parser(state)?));
            }
        }

        Err(format!("expected tool or unit subject, got {subject}"))
    }
}

impl State {
    pub(super) fn parser(state: &str) -> Result<Self, String> {
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
