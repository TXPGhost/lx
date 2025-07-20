use crate::ast::{self, Binop};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub fields: HashMap<Ident, Expr>,
    pub parent: Option<Rc<RefCell<Struct>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ident {
    VIdent(Rc<str>),
    TIdent(Rc<str>),
    Binop(Binop),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    I32(i32),
    Struct(Rc<RefCell<Struct>>),
    Block(Block),
    Call(Call),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Bind(Ident, Expr),
    BindMut(Ident, Expr, Expr),
    Expr(Expr),
}

impl Struct {
    pub fn lookup(&self, ident: Ident) -> Option<Expr> {
        match self.fields.get(&ident) {
            Some(expr) => Some(expr.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(ident).clone(),
                None => None,
            },
        }
    }

    pub fn lookup_opt(opt: Option<Rc<RefCell<Struct>>>, ident: Ident) -> Option<Expr> {
        match opt {
            Some(parent) => parent.borrow().lookup(ident).clone(),
            None => None,
        }
    }
}

impl Ident {
    pub fn as_str(&self) -> &str {
        match self {
            Ident::VIdent(ident) | Ident::TIdent(ident) => ident.as_ref(),
            Ident::Binop(Binop::Add) => "(+)",
        }
    }
}

pub trait IntoIr {
    type Ir;
    fn into_ir(self, ctxt: Option<Rc<RefCell<Struct>>>) -> Result<Self::Ir, String>;
}

impl IntoIr for ast::Struct {
    type Ir = Rc<RefCell<Struct>>;

    fn into_ir(self, ctxt: Option<Rc<RefCell<Struct>>>) -> Result<Self::Ir, String> {
        let mut fields = HashMap::new();
        for field in self.fields {
            match field {
                ast::Field::Field(ident, expr) => {
                    let name = ident.name.clone();
                    if fields
                        .insert(ident.into_ir(ctxt.clone())?, expr.into_ir(ctxt.clone())?)
                        .is_some()
                    {
                        return Err(format!("duplicate field {name}"));
                    }
                }
                ast::Field::Inline(expr) => {
                    let struct_ = match expr {
                        ast::Expr::Ident(ident) => {
                            let name = ident.name.clone();
                            let expr =
                                Struct::lookup_opt(ctxt.clone(), ident.into_ir(ctxt.clone())?);
                            match expr {
                                Some(Expr::Struct(struct_)) => return Ok(struct_),
                                Some(_) => return Err("illegal inline expr".to_string()),
                                None => {
                                    return Err(format!("lookup failed for ident {name}"));
                                }
                            }
                        }
                        ast::Expr::Struct(struct_) => struct_.into_ir(ctxt.clone())?,
                        _ => panic!("illegal inline expr"),
                    };
                    for (ident, expr) in struct_.borrow().fields.iter() {
                        if fields.insert(ident.clone(), expr.clone()).is_some() {
                            return Err(format!("duplicate field {}", &ident.as_str()));
                        }
                    }
                }
            }
        }
        Ok(Rc::new(RefCell::new(Struct {
            fields,
            parent: ctxt.clone(),
        })))
    }
}

impl IntoIr for ast::Ident {
    type Ir = Ident;

    fn into_ir(self, _: Option<Rc<RefCell<Struct>>>) -> Result<Self::Ir, String> {
        match self.is_type {
            true => Ok(Ident::TIdent(self.name)),
            false => Ok(Ident::VIdent(self.name)),
        }
    }
}

impl IntoIr for ast::Expr {
    type Ir = Expr;

    fn into_ir(self, ctxt: Option<Rc<RefCell<Struct>>>) -> Result<Self::Ir, String> {
        match self {
            ast::Expr::Ident(ident) => Ok(Expr::Ident(ident.into_ir(ctxt.clone())?)),
            ast::Expr::I32(n) => Ok(Expr::I32(n)),
            ast::Expr::Struct(struct_) => Ok(Expr::Struct(struct_.into_ir(ctxt.clone())?)),
            ast::Expr::Block(block) => Ok(Expr::Block(block.into_ir(ctxt.clone())?)),
            ast::Expr::Binop(lhs, binop, rhs) => Ok(Expr::Call(Call {
                func: Box::new(Expr::Ident(Ident::Binop(binop))),
                args: vec![lhs.into_ir(ctxt.clone())?, rhs.into_ir(ctxt.clone())?],
            })),
        }
    }
}

impl IntoIr for ast::Block {
    type Ir = Block;

    fn into_ir(self, ctxt: Option<Rc<RefCell<Struct>>>) -> Result<Self::Ir, String> {
        let mut stmts = Vec::new();
        for stmt in self.stmts {
            stmts.push(match stmt {
                ast::Stmt::Bind(ident, expr) => {
                    Stmt::Bind(ident.into_ir(ctxt.clone())?, expr.into_ir(ctxt.clone())?)
                }
                ast::Stmt::BindMut(ident, ty, expr) => Stmt::BindMut(
                    ident.into_ir(ctxt.clone())?,
                    ty.into_ir(ctxt.clone())?,
                    expr.into_ir(ctxt.clone())?,
                ),
                ast::Stmt::Write(_, _) => todo!(),
                ast::Stmt::Update(_, _, _) => todo!(),
                ast::Stmt::Expr(expr) => Stmt::Expr(expr.into_ir(ctxt.clone())?),
            })
        }
        Ok(Block { stmts })
    }
}
