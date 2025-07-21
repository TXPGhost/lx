pub mod helpers;
pub mod pretty_print;

use std::rc::Rc;

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
pub struct Named {
    pub is_mut: bool,
    pub name: Ident,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgIdent {
    pub is_mut: bool,
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Named(Named),
    Ident(ArgIdent),
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
    Binop(Binop),
    Func(Func),
    Call(Call),
    Constructor(Constructor),
    Project(Project),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binop {
    pub left: Box<Expr>,
    pub op: BinopKind,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub name: Ident,
    pub fields: Struct,
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
pub struct Bind {
    pub name: Ident,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindMut {
    pub name: Ident,
    pub initial: Expr,
    pub update: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Write {
    pub target: Expr,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub target: Expr,
    pub op: BinopKind,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prim {
    I32(i32),
    String(String),
    Char(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Bind(Bind),
    BindMut(BindMut),
    Write(Write),
    Update(Update),
    Expr(Expr),
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
