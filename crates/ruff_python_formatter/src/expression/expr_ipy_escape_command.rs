use ruff_python_ast::ExprIpyEscapeCommand;
use ruff_text_size::Ranged;

use crate::expression::parentheses::{NeedsParentheses, OptionalParentheses};
use crate::prelude::*;

#[derive(Default)]
pub struct FormatExprIpyEscapeCommand;

impl<'a> FormatNodeRule<'a, &'a ExprIpyEscapeCommand> for FormatExprIpyEscapeCommand {
    fn fmt_fields(&self, item: &'a ExprIpyEscapeCommand, f: &mut PyFormatter) -> FormatResult<()> {
        source_text_slice(item.range()).fmt(f)
    }
}

impl NeedsParentheses for ExprIpyEscapeCommand {
    fn needs_parentheses(
        &self,
        _parent: ruff_python_ast::AnyNodeRef,
        _context: &PyFormatContext,
    ) -> OptionalParentheses {
        OptionalParentheses::Never
    }
}
