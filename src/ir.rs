use crate::ast::{self, BinopKind};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ctxt {
    Struct(Rc<RefCell<Struct>>),
    Args(Rc<RefCell<Args>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub fields: HashMap<Ident, Expr>,
    pub parent: Option<Ctxt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub args: HashMap<Ident, (bool, Expr)>,
    pub parent: Option<Ctxt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ident {
    VIdent(Rc<str>),
    TIdent(Rc<str>),
    Binop(BinopKind),
    Void,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prim {
    I32(i32),
    String(String),
    Char(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    Prim(Prim),
    Struct(Rc<RefCell<Struct>>),
    Block(Block),
    Func(Func),
    Call(Call),
    Constructor(Ident, Rc<RefCell<Struct>>),
    Project(Project),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub expr: Box<Expr>,
    pub field: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub args: Rc<RefCell<Args>>,
    pub body: Box<Expr>,
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

impl Ctxt {
    pub fn lookup(&self, ident: Ident) -> Option<Expr> {
        match self {
            Ctxt::Struct(struct_) => match struct_.borrow().fields.get(&ident) {
                Some(expr) => Some(expr.clone()),
                None => match &struct_.borrow().parent {
                    Some(parent) => parent.lookup(ident).clone(),
                    None => None,
                },
            },
            Ctxt::Args(args) => match args.borrow().args.get(&ident) {
                Some((_, expr)) => Some(expr.clone()),
                None => match &args.borrow().parent {
                    Some(parent) => parent.lookup(ident).clone(),
                    None => None,
                },
            },
        }
    }

    pub fn lookup_opt(opt: Option<Ctxt>, ident: Ident) -> Option<Expr> {
        match opt {
            Some(ctxt) => ctxt.lookup(ident).clone(),
            None => None,
        }
    }
}

impl Ident {
    pub fn as_str(&self) -> &str {
        match self {
            Ident::VIdent(ident) | Ident::TIdent(ident) => ident.as_ref(),
            Ident::Binop(BinopKind::Add) => "(+)",
            Ident::Binop(BinopKind::Sub) => "(-)",
            Ident::Binop(BinopKind::Mul) => "(*)",
            Ident::Binop(BinopKind::Div) => "(/)",
            Ident::Binop(BinopKind::Pow) => "(^)",
            Ident::Binop(BinopKind::Concat) => "(++)",
            Ident::Void => "_",
        }
    }
}

pub trait IntoIr {
    type Ir;
    fn into_ir(self, ctxt: Option<Ctxt>) -> Result<Self::Ir, String>;
}

impl IntoIr for ast::Struct {
    type Ir = Rc<RefCell<Struct>>;

    fn into_ir(self, ctxt: Option<Ctxt>) -> Result<Self::Ir, String> {
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
                            let expr = Ctxt::lookup_opt(ctxt.clone(), ident.into_ir(ctxt.clone())?);
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
                ast::Field::Spacer => {}
            }
        }
        Ok(Rc::new(RefCell::new(Struct {
            fields,
            parent: ctxt,
        })))
    }
}

impl IntoIr for ast::Args {
    type Ir = Rc<RefCell<Args>>;

    fn into_ir(self, ctxt: Option<Ctxt>) -> Result<Self::Ir, String> {
        let mut args = HashMap::new();
        for arg in self.args {
            match arg {
                ast::Arg::Named(is_mut, ident, expr) => {
                    let name = ident.name.clone();
                    if args
                        .insert(
                            ident.into_ir(ctxt.clone())?,
                            (is_mut, expr.into_ir(ctxt.clone())?),
                        )
                        .is_some()
                    {
                        return Err(format!("duplicate argument {name}"));
                    }
                }
                ast::Arg::Ident(is_mut, ident) => {
                    if ident.is_type {
                        args.insert(
                            Ident::Void,
                            (is_mut, Expr::Ident(ident.into_ir(ctxt.clone())?)),
                        );
                    } else {
                        panic!("not yet implemented")
                    }
                }
            }
        }
        Ok(Rc::new(RefCell::new(Args { args, parent: ctxt })))
    }
}

impl IntoIr for ast::Ident {
    type Ir = Ident;

    fn into_ir(self, _: Option<Ctxt>) -> Result<Self::Ir, String> {
        match self.is_type {
            true => Ok(Ident::TIdent(self.name)),
            false => Ok(Ident::VIdent(self.name)),
        }
    }
}

impl IntoIr for ast::Expr {
    type Ir = Expr;

    fn into_ir(self, ctxt: Option<Ctxt>) -> Result<Self::Ir, String> {
        match self {
            ast::Expr::Ident(ident) => Ok(Expr::Ident(ident.into_ir(ctxt.clone())?)),
            ast::Expr::Prim(value) => Ok(Expr::Prim(value)),
            ast::Expr::Struct(struct_) => Ok(Expr::Struct(struct_.into_ir(ctxt.clone())?)),
            ast::Expr::Block(block) => Ok(Expr::Block(block.into_ir(ctxt.clone())?)),
            ast::Expr::Binop(lhs, binop, rhs) => Ok(Expr::Call(Call {
                func: Box::new(Expr::Ident(Ident::Binop(binop))),
                args: vec![lhs.into_ir(ctxt.clone())?, rhs.into_ir(ctxt.clone())?],
            })),
            ast::Expr::Func(func) => Ok(Expr::Func({
                let args = func.args.into_ir(ctxt.clone())?;
                args.borrow_mut().parent = ctxt.clone();
                let body = Box::new(func.body.into_ir(Some(Ctxt::Args(args.clone())))?);
                Func { args, body }
            })),
            ast::Expr::Call(call) => Ok(Expr::Call(Call {
                func: Box::new(call.func.into_ir(ctxt.clone())?),
                args: {
                    let mut args = Vec::new();
                    for arg in call.args {
                        args.push(arg.into_ir(ctxt.clone())?);
                    }
                    args
                },
            })),
            ast::Expr::Constructor(ident, constructor) => Ok(Expr::Constructor(
                ident.into_ir(ctxt.clone())?,
                constructor.into_ir(ctxt)?,
            )),
            ast::Expr::Project(project) => Ok(Expr::Project(Project {
                expr: Box::new(project.expr.into_ir(ctxt.clone())?),
                field: project.field.into_ir(ctxt)?,
            })),
        }
    }
}

impl IntoIr for ast::Block {
    type Ir = Block;

    fn into_ir(self, ctxt: Option<Ctxt>) -> Result<Self::Ir, String> {
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
