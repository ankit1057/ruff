#![allow(clippy::derive_partial_eq_without_eq)]

use std::ops::Deref;

use ruff_index::IndexVec;

use crate as ast;

#[derive(Clone, PartialEq)]
pub struct Ast {
    pub(crate) mod_modules: IndexVec<ast::ModModuleId, ast::ModModule>,
    pub(crate) mod_expressions: IndexVec<ast::ModExpressionId, ast::ModExpression>,
}

impl Ast {
    #[inline]
    pub fn wrap<T>(&self, node: T) -> Node<T> {
        Node { ast: self, node }
    }
}

impl std::fmt::Debug for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ast").finish()
    }
}

#[derive(Clone, Copy)]
pub struct Node<'ast, T> {
    pub ast: &'ast Ast,
    pub node: T,
}

impl<T> Node<'_, T> {
    pub fn as_ref(&self) -> &T {
        &self.node
    }
}

impl<T> std::fmt::Debug for Node<'_, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Node").field(&self.node).finish()
    }
}

impl<T> Deref for Node<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<T> Eq for Node<'_, T> where T: Eq {}

impl<T> std::hash::Hash for Node<'_, T>
where
    T: std::hash::Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.node.hash(state);
    }
}

impl<T> PartialEq for Node<'_, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

#[derive(Default)]
pub struct AstBuilder {
    mod_modules: IndexVec<ast::ModModuleId, ast::ModModule>,
    mod_expressions: IndexVec<ast::ModExpressionId, ast::ModExpression>,
}

impl std::fmt::Debug for AstBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstBuilder").finish()
    }
}

impl AstBuilder {
    pub fn build(mut self) -> ast::Ast {
        self.mod_modules.shrink_to_fit();
        self.mod_expressions.shrink_to_fit();
        ast::Ast {
            mod_modules: self.mod_modules,
            mod_expressions: self.mod_expressions,
        }
    }

    pub fn add_mod_module(&mut self, payload: ast::ModModule) -> ast::ModId {
        ast::ModId::Module(self.mod_modules.push(payload))
    }

    pub fn add_mod_expression(&mut self, payload: ast::ModExpression) -> ast::ModId {
        ast::ModId::Expression(self.mod_expressions.push(payload))
    }
}
