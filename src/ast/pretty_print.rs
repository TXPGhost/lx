use super::*;

#[derive(Clone, Copy, Default)]
pub struct PrettyPrintContext {
    pub indent_level: usize,
}

impl PrettyPrintContext {
    pub fn indented(self) -> PrettyPrintContext {
        PrettyPrintContext {
            indent_level: self.indent_level + 1,
        }
    }

    pub fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }
}

pub trait PrettyPrint {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String;
    fn pretty_print_string(&self) -> String {
        self.pretty_print(PrettyPrintContext::default())
    }
}

impl PrettyPrint for Struct {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        let mut s = String::new();
        s += "(\n";
        for field in &self.fields {
            s += &ctxt.indented().indent();
            s += &field.pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += ")";
        s
    }
}

impl PrettyPrint for Field {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Field::Field(ident, expr) => {
                format!(
                    "{} {}",
                    ident.pretty_print(ctxt),
                    expr.pretty_print(ctxt.indented())
                )
            }
            Field::Inline(expr) => format!("..{}", expr.pretty_print(ctxt.indented())),
        }
    }
}

impl PrettyPrint for Ident {
    fn pretty_print(&self, _: PrettyPrintContext) -> String {
        self.name.as_ref().to_owned()
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Expr::Ident(ident) => ident.pretty_print(ctxt),
            Expr::I32(n) => n.to_string(),
            Expr::Struct(struct_) => struct_.pretty_print(ctxt),
            Expr::Block(block) => block.pretty_print(ctxt),
            Expr::Binop(lhs, op, rhs) => format!(
                "{} {} {}",
                lhs.pretty_print(ctxt),
                op.pretty_print(ctxt),
                rhs.pretty_print(ctxt)
            ),
        }
    }
}

impl PrettyPrint for Block {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        let mut s = String::new();
        s += "{\n";
        for stmt in &self.stmts {
            s += &ctxt.indented().indent();
            s += &stmt.pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += "}";
        s
    }
}

impl PrettyPrint for Binop {
    fn pretty_print(&self, _: PrettyPrintContext) -> String {
        match self {
            Binop::Add => "+".to_owned(),
        }
    }
}

impl PrettyPrint for Stmt {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Stmt::Bind(ident, expr) => {
                format!("{} = {}", ident.pretty_print(ctxt), expr.pretty_print(ctxt))
            }
            Stmt::BindMut(ident, ty, expr) => format!(
                "{} {} = {}",
                ident.pretty_print(ctxt),
                ty.pretty_print(ctxt),
                expr.pretty_print(ctxt)
            ),
            Stmt::Write(ident, expr) => {
                format!(
                    "{} := {}",
                    ident.pretty_print(ctxt),
                    expr.pretty_print(ctxt)
                )
            }
            Stmt::Update(ident, binop, expr) => format!(
                "{} {}= {}",
                ident.pretty_print(ctxt),
                binop.pretty_print(ctxt),
                expr.pretty_print(ctxt)
            ),
            Stmt::Expr(expr) => expr.pretty_print(ctxt).to_string(),
        }
    }
}
