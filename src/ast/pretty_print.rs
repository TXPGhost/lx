use colored::{Color, Colorize};

use super::*;

pub const MEMBER: Color = Color::TrueColor {
    r: 209,
    g: 175,
    b: 121,
};
pub const CONST: Color = Color::TrueColor {
    r: 198,
    g: 70,
    b: 64,
};
pub const PUNCT: Color = Color::TrueColor {
    r: 158,
    g: 151,
    b: 140,
};

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
        s += &format!("{}\n", "(".color(PUNCT));
        for field in &self.fields {
            s += &ctxt.indented().indent();
            s += &field.pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += &format!("{}", ")".color(PUNCT));
        s
    }
}

impl PrettyPrint for Field {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Field::Field(ident, expr) => {
                format!(
                    "{} {}",
                    ident.pretty_print(ctxt).color(MEMBER),
                    expr.pretty_print(ctxt.indented())
                )
            }
            Field::Inline(expr) => format!(
                "{}{}",
                "..".color(PUNCT),
                expr.pretty_print(ctxt.indented())
            ),
        }
    }
}

impl PrettyPrint for Ident {
    fn pretty_print(&self, _: PrettyPrintContext) -> String {
        if self.is_type {
            format!("{}", self.name.as_ref().bold())
        } else {
            self.name.as_ref().to_owned()
        }
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Expr::Ident(ident) => ident.pretty_print(ctxt),
            Expr::I32(n) => format!("{}", n.to_string().color(CONST)),
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
        s += &format!("{}\n", "{".color(PUNCT));
        for stmt in &self.stmts {
            s += &ctxt.indented().indent();
            s += &stmt.pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += &format!("{}", "}".color(PUNCT));
        s
    }
}

impl PrettyPrint for Binop {
    fn pretty_print(&self, _: PrettyPrintContext) -> String {
        match self {
            Binop::Add => format!("{}", "+".color(PUNCT)),
        }
    }
}

impl PrettyPrint for Stmt {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Stmt::Bind(ident, expr) => {
                format!(
                    "{} {} {}",
                    ident.pretty_print(ctxt),
                    "=".color(PUNCT),
                    expr.pretty_print(ctxt)
                )
            }
            Stmt::BindMut(ident, ty, expr) => format!(
                "{} {} {} {}",
                ident.pretty_print(ctxt),
                ty.pretty_print(ctxt),
                "=".color(PUNCT),
                expr.pretty_print(ctxt)
            ),
            Stmt::Write(ident, expr) => {
                format!(
                    "{} {} {}",
                    ident.pretty_print(ctxt),
                    ":=".color(PUNCT),
                    expr.pretty_print(ctxt)
                )
            }
            Stmt::Update(ident, binop, expr) => format!(
                "{} {}{} {}",
                ident.pretty_print(ctxt),
                binop.pretty_print(ctxt),
                "=".color(PUNCT),
                expr.pretty_print(ctxt)
            ),
            Stmt::Expr(expr) => expr.pretty_print(ctxt).to_string(),
        }
    }
}
