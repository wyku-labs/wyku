use crate::syntax::Id;

#[derive(Debug, PartialEq)]
pub(crate) enum Subject {
    Unit(Id, UnitState),
    Tool(Id),
}

#[derive(Debug, PartialEq)]
pub(crate) enum UnitState {
    Start,
    Finish,
    Fail,
    Watch,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    Subject(Subject),
    NOT(Box<Expression>),
    AND((Box<Expression>, Option<Box<Expression>>)),
    OR((Box<Expression>, Option<Box<Expression>>)),
}

impl Expression {
    pub(crate) fn parser(when: &str) -> Result<Expression, String> {
        let mut rest: Vec<&str> = when.trim().split_whitespace().collect();
        if rest.is_empty() {
            return Err(format!("expression is empty"));
        }

        let mut expression: Option<Expression> = None;

        none_parse(&mut expression, &mut rest)
    }
}

fn none_parse(
    expression: &mut Option<Expression>,
    rest: &mut Vec<&str>,
) -> Result<Expression, String> {
    if rest[0] == "AND" || rest[0] == "OR" {
        return Err(format!("{} missing arguments", rest[0]));
    }

    let subject = subject_parse(expression, rest)?;
    *expression = Some(subject);
    operator_parse(expression, rest)
}

fn not_parse(
    expression: &mut Option<Expression>,
    rest: &mut Vec<&str>,
) -> Result<Expression, String> {
    rest.remove(0);

    if rest.is_empty() {
        return Err(format!("NOT missing arguments"));
    }

    Ok(Expression::NOT(Box::new(subject_parse(expression, rest)?)))
}

fn subject_parse(
    expression: &mut Option<Expression>,
    rest: &mut Vec<&str>,
) -> Result<Expression, String> {
    let Some(word) = rest.first().copied() else {
        return Err(format!("expected a Subject or NOT, got ''"));
    };

    if word == "NOT" {
        return not_parse(expression, rest);
    }

    rest.remove(0);
    Ok(Expression::Subject(Subject::parser(word)?))
}

fn operator_parse(
    expression: &mut Option<Expression>,
    rest: &mut Vec<&str>,
) -> Result<Expression, String> {
    let Some(word) = rest.first().copied() else {
        return expression
            .take()
            .ok_or("expected a subject, got ''".to_string());
    };

    let construct = match word {
        "AND" => Expression::AND,
        "OR" => Expression::OR,
        _ => return Err(format!("expected an operator, got '{word}'")),
    };

    let left = expression
        .take()
        .ok_or("operator missing arguments".to_string())?;

    rest.remove(0);

    let right = subject_parse(expression, rest)?;

    let built = construct((Box::new(left), Some(Box::new(right))));

    *expression = Some(built);
    operator_parse(expression, rest)
}

impl Subject {
    fn parser(subject: &str) -> Result<Subject, String> {
        if subject.starts_with(":") || !subject.contains(":") {
            return Ok(Subject::Tool(Id::new_tool(subject)?));
        }

        if subject.contains(":") {
            if let Some(split) = subject.split_once(":") {
                let (unit, state) = split;
                return Ok(Subject::Unit(Id::new(unit)?, UnitState::parser(state)?));
            }
        }

        Err(format!("expected tool or unit subject, got {subject}"))
    }
}

impl UnitState {
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
