use ruff_formatter::write;
use ruff_python_ast::{ModModule, Node};
use ruff_python_trivia::lines_after;

use crate::prelude::*;
use crate::statement::suite::SuiteKind;
use crate::FormatNodeRule;

#[derive(Default)]
pub struct FormatModModule;

impl<'a> FormatNodeRule<'a, Node<'a, &'a ModModule>> for FormatModModule {
    fn fmt_fields(&self, item: Node<'a, &'a ModModule>, f: &mut PyFormatter) -> FormatResult<()> {
        let ModModule { range, body } = item.as_ref();

        if body.is_empty() {
            // Only preserve an empty line if the source contains an empty line too.
            if !f.context().comments().has_leading(item)
                && lines_after(range.start(), f.context().source()) != 0
            {
                empty_line().fmt(f)
            } else {
                Ok(())
            }
        } else {
            write!(
                f,
                [
                    body.format().with_options(SuiteKind::TopLevel),
                    // Trailing newline at the end of the file
                    hard_line_break()
                ]
            )
        }
    }
}
