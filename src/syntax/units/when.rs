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
    if rest.is_empty() {
        return Err(format!("expected a Subject or NOT, got ''"));
    }

    if rest[0] == "NOT" {
        return not_parse(expression, rest);
    }

    let word = rest.remove(0);
    Ok(Expression::Subject(Subject::parser(word)?))
}

fn operator_parse(
    expression: &mut Option<Expression>,
    rest: &mut Vec<&str>,
) -> Result<Expression, String> {
    if rest.is_empty() {
        return expression
            .take()
            .ok_or(format!("expected a subject, got ''"));
    }

    let construct = match rest[0] {
        "AND" => Expression::AND,
        "OR" => Expression::OR,
        _ => return Err(format!("expected an operator, got '{}'", rest[0])),
    };

    let left = expression
        .take()
        .ok_or(format!("operator {} missing arguments", rest[0]))?;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        assert!(Expression::parser("").is_err());
    }

    #[test]
    fn not_alone() {
        let alones = vec!["OR", "AND", "NOT"];
        for i in alones {
            assert!(Expression::parser(i).is_err());
        }
    }

    #[test]
    fn unit() {
        let units = vec!["main:watch", "build:start", "build:fail"];
        for i in units {
            assert_eq!(
                Expression::parser(i).unwrap(),
                Expression::Subject(Subject::parser(i).unwrap()),
            );
        }
    }

    #[test]
    fn tool() {
        let tools = vec![":bash", "curl", "safe_command"];
        for i in tools {
            assert_eq!(
                Expression::parser(i).unwrap(),
                Expression::Subject(Subject::parser(i).unwrap())
            );
        }
    }

    #[test]
    fn not() {
        let subjects = vec!["build:start", "my_command", ":bash", "main:watch"];
        for i in subjects {
            assert_eq!(
                Expression::parser(&format!("NOT {i}")).unwrap(),
                Expression::NOT(Box::new(Expression::Subject(Subject::parser(i).unwrap())))
            );
        }
    }

    #[test]
    fn not_nested() {
        assert_eq!(
            Expression::parser("NOT NOT dev:start").unwrap(),
            Expression::NOT(Box::new(Expression::NOT(Box::new(Expression::Subject(
                Subject::parser("dev:start").unwrap(),
            )))))
        );
    }

    #[test]
    fn chain() {
        let a = Expression::Subject(Subject::parser("a:start").unwrap());
        let b = Expression::Subject(Subject::parser("b:finish").unwrap());
        let c = Expression::Subject(Subject::parser("c:fail").unwrap());
        let d = Expression::Subject(Subject::parser("d:watch").unwrap());

        assert_eq!(
            Expression::parser("a:start OR b:finish AND c:fail OR d:watch").unwrap(),
            Expression::OR((
                Box::new(Expression::AND((
                    Box::new(Expression::OR((Box::new(a), Some(Box::new(b))))),
                    Some(Box::new(c)),
                ))),
                Some(Box::new(d)),
            ))
        );
    }

    #[test]
    fn not_and() {
        let a = Expression::Subject(Subject::parser("a:start").unwrap());
        let b = Expression::Subject(Subject::parser("b:finish").unwrap());

        assert_eq!(
            Expression::parser("NOT a:start AND b:finish").unwrap(),
            Expression::AND((Box::new(Expression::NOT(Box::new(a))), Some(Box::new(b)),))
        );
    }

    #[test]
    fn not_on_the_right() {
        let a = Expression::Subject(Subject::parser("a:start").unwrap());
        let b = Expression::Subject(Subject::parser("b:finish").unwrap());
        let c = Expression::Subject(Subject::parser("c:fail").unwrap());

        assert_eq!(
            Expression::parser("a:start AND NOT b:finish OR c:fail").unwrap(),
            Expression::OR((
                Box::new(Expression::AND((
                    Box::new(a),
                    Some(Box::new(Expression::NOT(Box::new(b)))),
                ))),
                Some(Box::new(c)),
            ))
        );
    }
}
