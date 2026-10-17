use crate::syntax::when::{Expression, UnitState};

use crate::syntax::Id;
use crate::syntax::Tool;

#[derive(Debug)]
pub(crate) struct Unit {
    id: Id,
    task: Option<String>,
    exec: Option<Tool>,
    when: Option<Expression>,
    tools: Option<Vec<Id>>,

    state: UnitState,
    ok: Option<bool>,
}
