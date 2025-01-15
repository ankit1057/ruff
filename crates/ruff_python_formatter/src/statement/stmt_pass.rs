use ruff_python_ast::StmtPass;

use crate::comments::SourceComment;
use crate::{has_skip_comment, prelude::*};

#[derive(Default)]
pub struct FormatStmtPass;

impl<'a> FormatNodeRule<'a, &'a StmtPass> for FormatStmtPass {
    fn fmt_fields(&self, _item: &'a StmtPass, f: &mut PyFormatter) -> FormatResult<()> {
        token("pass").fmt(f)
    }

    fn is_suppressed(
        &self,
        trailing_comments: &[SourceComment],
        context: &PyFormatContext,
    ) -> bool {
        has_skip_comment(trailing_comments, context.source())
    }
}
