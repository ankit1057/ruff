use ruff_formatter::write;
use ruff_python_ast::TypeParamTypeVarTuple;

use crate::prelude::*;

#[derive(Default)]
pub struct FormatTypeParamTypeVarTuple;

impl<'a> FormatNodeRule<'a, &'a TypeParamTypeVarTuple> for FormatTypeParamTypeVarTuple {
    fn fmt_fields(&self, item: &'a TypeParamTypeVarTuple, f: &mut PyFormatter) -> FormatResult<()> {
        let TypeParamTypeVarTuple {
            range: _,
            name,
            default,
        } = item;
        write!(f, [token("*"), name.format()])?;
        if let Some(default) = default {
            write!(f, [space(), token("="), space(), default.format()])?;
        }
        Ok(())
    }
}
