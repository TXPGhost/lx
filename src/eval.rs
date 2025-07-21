use std::collections::HashMap;

use crate::ir::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Prim(Prim),
}

pub trait Eval {
    fn eval(&self, ctxt: &mut HashMap<Ident, Value>) -> Result<Option<Value>, &'static str>;
}

impl Eval for Block {
    fn eval(&self, ctxt: &mut HashMap<Ident, Value>) -> Result<Option<Value>, &'static str> {
        let mut return_value = None;
        for stmt in &self.stmts {
            return_value = stmt.eval(ctxt)?;
        }
        Ok(return_value)
    }
}

impl Eval for Stmt {
    fn eval(&self, ctxt: &mut HashMap<Ident, Value>) -> Result<Option<Value>, &'static str> {
        match self {
            Stmt::Bind(Ident::Void, _) => Ok(None),
            Stmt::Bind(ident, expr) => {
                let expr = expr.eval(ctxt)?.ok_or("expr did not eval")?;
                ctxt.insert(ident.clone(), expr);
                Ok(None)
            }
            Stmt::BindMut(ident, ty, expr) => {
                let expr = expr.eval(ctxt)?.ok_or("expr did not eval")?;
                ctxt.insert(ident.clone(), expr);
                Ok(None)
            }
            Stmt::Expr(expr) => todo!(),
        }
    }
}

impl Eval for Expr {
    fn eval(&self, ctxt: &mut HashMap<Ident, Value>) -> Result<Option<Value>, &'static str> {
        match self {
            Expr::Ident(ident) => panic!("not yet implemented"),
            Expr::Prim(prim) => todo!(),
            Expr::Struct(ref_cell) => todo!(),
            Expr::Block(block) => todo!(),
            Expr::Func(func) => todo!(),
            Expr::Call(call) => todo!(),
            Expr::Constructor(ident, ref_cell) => todo!(),
            Expr::Project(project) => todo!(),
        }
    }
}
