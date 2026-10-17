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

fn is_operator(operator: &str) -> bool {
    return match operator {
        "NOT" | "AND" | "OR" => true,
        _ => false,
    };
}

impl Expression {
    pub(crate) fn parser(when: &str) -> Result<Expression, String> {
        let when = when.trim();
        let cutted: Vec<&str> = when.split(" ").collect();

        if cutted.len() == 0 {
            if when.chars().all(|c| c.is_uppercase()) {
                return Err(format!("unknow {when} operator"));
            }
            if is_operator(when) {
                return Err(format!("{when} missing arguments"));
            }

            return Ok(Expression::Subject(Subject::parser(when)?));
        }

        todo!()
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
