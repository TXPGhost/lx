use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Field(Ident, Expr),
    Inline(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: Rc<str>,
    pub is_type: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    I32(i32),
    Struct(Struct),
    Block(Block),
    Binop(Box<Expr>, Binop, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Bind(Ident, Expr),
    BindMut(Ident, Expr, Expr),
    Write(Ident, Box<Expr>),
    Update(Ident, Binop, Box<Expr>),
    Expr(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
}

pub fn vid(name: &'static str) -> Ident {
    Ident {
        name: name.into(),
        is_type: false,
    }
}

pub fn tid(name: &'static str) -> Ident {
    Ident {
        name: name.into(),
        is_type: true,
    }
}
