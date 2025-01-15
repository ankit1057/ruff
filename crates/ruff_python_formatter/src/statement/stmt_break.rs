use ruff_python_ast::StmtBreak;

use crate::comments::SourceComment;
use crate::{has_skip_comment, prelude::*};

#[derive(Default)]
pub struct FormatStmtBreak;

impl<'a> FormatNodeRule<'a, &'a StmtBreak> for FormatStmtBreak {
    fn fmt_fields(&self, _item: &'a StmtBreak, f: &mut PyFormatter) -> FormatResult<()> {
        token("break").fmt(f)
    }

    fn is_suppressed(
        &self,
        trailing_comments: &[SourceComment],
        context: &PyFormatContext,
    ) -> bool {
        has_skip_comment(trailing_comments, context.source())
    }
}
