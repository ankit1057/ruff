use ruff_python_ast::StmtIpyEscapeCommand;
use ruff_text_size::Ranged;

use crate::comments::SourceComment;
use crate::{has_skip_comment, prelude::*};

#[derive(Default)]
pub struct FormatStmtIpyEscapeCommand;

impl<'a> FormatNodeRule<'a, &'a StmtIpyEscapeCommand> for FormatStmtIpyEscapeCommand {
    fn fmt_fields(&self, item: &'a StmtIpyEscapeCommand, f: &mut PyFormatter) -> FormatResult<()> {
        source_text_slice(item.range()).fmt(f)
    }

    fn is_suppressed(
        &self,
        trailing_comments: &[SourceComment],
        context: &PyFormatContext,
    ) -> bool {
        has_skip_comment(trailing_comments, context.source())
    }
}
