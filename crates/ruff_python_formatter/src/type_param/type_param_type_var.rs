use ruff_formatter::write;
use ruff_python_ast::TypeParamTypeVar;

use crate::prelude::*;

#[derive(Default)]
pub struct FormatTypeParamTypeVar;

impl<'a> FormatNodeRule<'a, &'a TypeParamTypeVar> for FormatTypeParamTypeVar {
    fn fmt_fields(&self, item: &'a TypeParamTypeVar, f: &mut PyFormatter) -> FormatResult<()> {
        let TypeParamTypeVar {
            range: _,
            name,
            bound,
            default,
        } = item;
        name.format().fmt(f)?;
        if let Some(bound) = bound {
            write!(f, [token(":"), space(), bound.format()])?;
        }
        if let Some(default) = default {
            write!(f, [space(), token("="), space(), default.format()])?;
        }
        Ok(())
    }
}
