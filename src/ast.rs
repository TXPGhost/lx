pub mod helpers;
pub mod pretty_print;

use crate::node::*;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct<'a, M: NodeMeta> {
    pub fields: Vec<Node<Field<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args<'a, M: NodeMeta> {
    pub args: Vec<Node<Arg<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field<'a, M: NodeMeta> {
    Field(Ident, Node<Expr<'a, M>, M>),
    Inline(Expr<'a, M>),
    Spacer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArg<'a, M: NodeMeta> {
    pub is_mut: bool,
    pub name: Ident,
    pub value: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentArg {
    pub is_mut: bool,
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg<'a, M: NodeMeta> {
    Named(NamedArg<'a, M>),
    Ident(IdentArg),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: Rc<str>,
    pub is_type: bool,
    pub is_void: bool,
    pub nhoist: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a, M: NodeMeta> {
    Ident(Ident),
    Prim(Prim),
    Struct(Struct<'a, M>),
    Block(Block<'a, M>),
    Binop(Binop<'a, M>),
    Func(Func<'a, M>),
    Call(Call<'a, M>),
    Constructor(Constructor<'a, M>),
    Project(Project<'a, M>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binop<'a, M: NodeMeta> {
    pub left: Node<Expr<'a, M>, M>,
    pub op: BinopKind,
    pub right: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor<'a, M: NodeMeta> {
    pub name: Ident,
    pub fields: Struct<'a, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project<'a, M: NodeMeta> {
    pub expr: Node<Expr<'a, M>, M>,
    pub field: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func<'a, M: NodeMeta> {
    pub args: Args<'a, M>,
    pub body: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'a, M: NodeMeta> {
    pub func: Node<Expr<'a, M>, M>,
    pub args: Vec<Node<Expr<'a, M>, M>>,
    pub method_syntax: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'a, M: NodeMeta> {
    pub stmts: Vec<Node<Stmt<'a, M>, M>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind<'a, M: NodeMeta> {
    pub name: Ident,
    pub value: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindMut<'a, M: NodeMeta> {
    pub name: Ident,
    pub initial: Node<Expr<'a, M>, M>,
    pub update: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Write<'a, M: NodeMeta> {
    pub target: Node<Expr<'a, M>, M>,
    pub value: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update<'a, M: NodeMeta> {
    pub target: Node<Expr<'a, M>, M>,
    pub op: BinopKind,
    pub value: Node<Expr<'a, M>, M>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prim {
    I32(i32),
    String(String),
    Char(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<'a, M: NodeMeta> {
    Bind(Bind<'a, M>),
    BindMut(BindMut<'a, M>),
    Write(Write<'a, M>),
    Update(Update<'a, M>),
    Expr(Node<Expr<'a, M>, M>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinopKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Concat,
}
