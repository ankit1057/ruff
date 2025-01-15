use ruff_python_ast::AnyNodeRef;
use ruff_python_ast::ExprNoneLiteral;

use crate::expression::parentheses::{NeedsParentheses, OptionalParentheses};
use crate::prelude::*;

#[derive(Default)]
pub struct FormatExprNoneLiteral;

impl<'a> FormatNodeRule<'a, &'a ExprNoneLiteral> for FormatExprNoneLiteral {
    fn fmt_fields(&self, _item: &'a ExprNoneLiteral, f: &mut PyFormatter) -> FormatResult<()> {
        token("None").fmt(f)
    }
}

impl NeedsParentheses for ExprNoneLiteral {
    fn needs_parentheses(
        &self,
        _parent: AnyNodeRef,
        _context: &PyFormatContext,
    ) -> OptionalParentheses {
        OptionalParentheses::BestFit
    }
}
