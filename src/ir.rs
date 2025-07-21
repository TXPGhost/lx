use std::{collections::HashMap, rc::Rc};

use crate::ast::Prim;
use crate::node::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: Rc<str>,
    pub is_type: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a, M: NodeMeta> {
    Ident(Ident),
    Prim(Prim),
    Struct(Struct<'a, M>),
    Block(Block<'a, M>),
    Func(Func<'a, M>),
    Call(Call<'a, M>),
    Constructor(Constructor<'a, M>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'a, M: NodeMeta> {
    pub stmts: Vec<Node<Stmt<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<'a, M: NodeMeta> {
    Bind(Bind<'a, M>),
    Expr(Node<Expr<'a, M>, M>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind<'a, M: NodeMeta> {
    pub name: Ident,
    pub value: Node<Expr<'a, M>, M>,
    pub ty: Option<Node<Expr<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Write<'a, M: NodeMeta> {
    pub target: Node<Expr<'a, M>, M>,
    pub value: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct<'a, M: NodeMeta> {
    pub fields: HashMap<Ident, Node<Expr<'a, M>, M>>,
    pub parent: Option<Node<Expr<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'a, M: NodeMeta> {
    pub func: Node<Expr<'a, M>, M>,
    pub args: Vec<Node<Expr<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func<'a, M: NodeMeta> {
    pub args: Node<Args<'a, M>, M>,
    pub body: Node<Block<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args<'a, M: NodeMeta> {
    pub args: HashMap<Ident, Arg<'a, M>>,
    pub parent: Option<Node<Expr<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg<'a, M: NodeMeta> {
    pub ty: Node<Expr<'a, M>, M>,
    pub is_mut: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor<'a, M: NodeMeta> {
    pub ty: Node<Struct<'a, M>, M>,
    pub fields: Struct<'a, M>,
}
