pub mod helpers;
pub mod pretty_print;

use std::rc::Rc;

use crate::ir::Prim;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Field(Ident, Expr),
    Inline(Expr),
    Spacer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Named(bool, Ident, Expr),
    Ident(bool, Ident),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: Rc<str>,
    pub is_type: bool,
    pub is_void: bool,
    pub nhoist: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    Prim(Prim),
    Struct(Struct),
    Block(Block),
    Binop(Box<Expr>, Binop, Box<Expr>),
    Func(Func),
    Call(Call),
    Constructor(Ident, Struct),
    Project(Project),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub expr: Box<Expr>,
    pub field: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub args: Args,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
    pub method_syntax: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Bind(Ident, Expr),
    BindMut(Ident, Expr, Expr),
    Write(Expr, Expr),
    Update(Expr, Binop, Expr),
    Expr(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Concat,
}
