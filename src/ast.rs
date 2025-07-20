pub mod pretty_print;

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
    Call(Call),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Bind(Ident, Expr),
    BindMut(Ident, Expr, Expr),
    Write(Ident, Expr),
    Update(Ident, Binop, Expr),
    Expr(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binop {
    Add,
}

pub fn estruct(fields: impl Into<Vec<Field>>) -> Expr {
    Expr::Struct(Struct {
        fields: fields.into(),
    })
}

pub fn inline_struct(fields: impl Into<Vec<Field>>) -> Field {
    Field::Inline(Expr::Struct(Struct {
        fields: fields.into(),
    }))
}

pub fn field(ident: Ident, expr: Expr) -> Field {
    Field::Field(ident, expr)
}

pub fn inline(struct_: Struct) -> Field {
    Field::Inline(Expr::Struct(struct_))
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

pub fn evid(name: &'static str) -> Expr {
    Expr::Ident(vid(name))
}

pub fn etid(name: &'static str) -> Expr {
    Expr::Ident(tid(name))
}

pub fn ei32(n: i32) -> Expr {
    Expr::I32(n)
}

pub fn add(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Add, Box::new(rhs))
}

pub fn block(stmts: Vec<Stmt>) -> Block {
    Block { stmts }
}

pub fn eblock(stmts: Vec<Stmt>) -> Expr {
    Expr::Block(Block { stmts })
}

pub fn call(func: Expr, args: Vec<Expr>) -> Call {
    Call {
        func: Box::new(func),
        args,
    }
}

pub fn ecall(func: Expr, args: Vec<Expr>) -> Expr {
    Expr::Call(Call {
        func: Box::new(func),
        args,
    })
}

pub fn bind(ident: Ident, expr: Expr) -> Stmt {
    Stmt::Bind(ident, expr)
}

pub fn bind_mut(ident: Ident, ty: Expr, expr: Expr) -> Stmt {
    Stmt::BindMut(ident, ty, expr)
}

pub fn write(ident: Ident, expr: Expr) -> Stmt {
    Stmt::Write(ident, expr)
}

pub fn add_assn(ident: Ident, expr: Expr) -> Stmt {
    Stmt::Update(ident, Binop::Add, expr)
}

pub fn stmt_expr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}
