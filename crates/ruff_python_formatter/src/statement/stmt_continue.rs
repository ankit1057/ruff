use ruff_python_ast::StmtContinue;

use crate::comments::SourceComment;
use crate::{has_skip_comment, prelude::*};

#[derive(Default)]
pub struct FormatStmtContinue;

impl<'a> FormatNodeRule<'a, &'a StmtContinue> for FormatStmtContinue {
    fn fmt_fields(&self, _item: &'a StmtContinue, f: &mut PyFormatter) -> FormatResult<()> {
        token("continue").fmt(f)
    }

    fn is_suppressed(
        &self,
        trailing_comments: &[SourceComment],
        context: &PyFormatContext,
    ) -> bool {
        has_skip_comment(trailing_comments, context.source())
    }
}
