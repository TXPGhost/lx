pub mod pretty_print;

use std::rc::Rc;

use crate::eval::Value;

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
    Named(Ident, Expr),
    Ident(Ident),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: Rc<str>,
    pub is_type: bool,
    pub is_void: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    Value(Value),
    Struct(Struct),
    Block(Block),
    Binop(Box<Expr>, Binop, Box<Expr>),
    Func(Func),
    Call(Call),
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

pub fn args(args: impl Into<Vec<Arg>>) -> Args {
    Args { args: args.into() }
}

pub fn estruct(fields: impl Into<Vec<Field>>) -> Expr {
    Expr::Struct(Struct {
        fields: fields.into(),
    })
}

pub fn estring(string: impl Into<String>) -> Expr {
    Expr::Value(Value::String(string.into()))
}

pub fn echar(char: char) -> Expr {
    Expr::Value(Value::Char(char as u8))
}

pub fn istruct(fields: impl Into<Vec<Field>>) -> Field {
    Field::Inline(Expr::Struct(Struct {
        fields: fields.into(),
    }))
}

pub fn field(ident: Ident, expr: Expr) -> Field {
    Field::Field(ident, expr)
}

pub fn arg(ident: Ident, expr: Expr) -> Arg {
    Arg::Named(ident, expr)
}

pub fn aident(ident: Ident) -> Arg {
    Arg::Ident(ident)
}

pub fn fspacer() -> Field {
    Field::Spacer
}

pub fn inline(expr: Expr) -> Field {
    Field::Inline(expr)
}

pub fn vid(name: &'static str) -> Ident {
    Ident {
        name: name.into(),
        is_type: false,
        is_void: false,
    }
}

pub fn tid(name: &'static str) -> Ident {
    Ident {
        name: name.into(),
        is_type: true,
        is_void: false,
    }
}

pub fn void() -> Ident {
    Ident {
        name: "_".into(),
        is_type: false,
        is_void: true,
    }
}

pub fn evid(name: &'static str) -> Expr {
    Expr::Ident(vid(name))
}

pub fn etid(name: &'static str) -> Expr {
    Expr::Ident(tid(name))
}

pub fn ei32(n: i32) -> Expr {
    Expr::Value(Value::I32(n))
}

pub fn add(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Add, Box::new(rhs))
}

pub fn block(stmts: impl Into<Vec<Stmt>>) -> Block {
    Block {
        stmts: stmts.into(),
    }
}

pub fn eblock(stmts: impl Into<Vec<Stmt>>) -> Expr {
    Expr::Block(Block {
        stmts: stmts.into(),
    })
}

pub fn efunc(args: Args, body: Expr) -> Expr {
    Expr::Func(Func {
        args,
        body: Box::new(body),
    })
}

pub fn eproject(expr: Expr, field: Ident) -> Expr {
    Expr::Project(Project {
        expr: Box::new(expr),
        field,
    })
}

pub fn call(func: Expr, args: impl Into<Vec<Expr>>) -> Call {
    Call {
        func: Box::new(func),
        args: args.into(),
    }
}

pub fn ecall(func: Expr, args: impl Into<Vec<Expr>>) -> Expr {
    Expr::Call(Call {
        func: Box::new(func),
        args: args.into(),
    })
}

pub fn sbind(ident: Ident, expr: Expr) -> Stmt {
    Stmt::Bind(ident, expr)
}

pub fn sbindmut(ident: Ident, ty: Expr, expr: Expr) -> Stmt {
    Stmt::BindMut(ident, ty, expr)
}

pub fn swrite(ident: Ident, expr: Expr) -> Stmt {
    Stmt::Write(ident, expr)
}

pub fn sadd(ident: Ident, expr: Expr) -> Stmt {
    Stmt::Update(ident, Binop::Add, expr)
}

pub fn sexpr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}
