use ruff_python_ast::ModExpression;

use crate::prelude::*;

#[derive(Default)]
pub struct FormatModExpression;

impl<'a> FormatNodeRule<'a, &'a ModExpression> for FormatModExpression {
    fn fmt_fields(&self, item: &'a ModExpression, f: &mut PyFormatter) -> FormatResult<()> {
        let ModExpression { body, range: _ } = item;
        body.format().fmt(f)
    }
}
