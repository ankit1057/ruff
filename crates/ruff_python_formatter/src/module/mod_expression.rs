use ruff_python_ast::{ModExpression, Node};

use crate::prelude::*;

#[derive(Default)]
pub struct FormatModExpression;

impl<'a> FormatNodeRule<'a, Node<'a, &'a ModExpression>> for FormatModExpression {
    fn fmt_fields(
        &self,
        item: Node<'a, &'a ModExpression>,
        f: &mut PyFormatter,
    ) -> FormatResult<()> {
        let ModExpression { body, range: _ } = item.as_ref();
        body.format().fmt(f)
    }
}
