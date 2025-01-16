#![allow(clippy::derive_partial_eq_without_eq)]

use std::ops::Deref;

use crate as ast;

#[derive(Clone, Default, PartialEq)]
pub struct Ast {
    pub(crate) mod_storage: ast::ModStorage,
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
