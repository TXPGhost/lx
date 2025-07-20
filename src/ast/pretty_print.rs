use colored::{Color, Colorize};

use super::*;

pub const NORMAL: Color = Color::TrueColor {
    r: 210,
    g: 201,
    b: 187,
};
pub const MEMBER: Color = Color::TrueColor {
    r: 155,
    g: 155,
    b: 173,
};
pub const TYPE: Color = Color::TrueColor {
    r: 143,
    g: 175,
    b: 167,
};
pub const FUNCTION: Color = Color::TrueColor {
    r: 149,
    g: 174,
    b: 167,
};
pub const CONSTANT: Color = Color::TrueColor {
    r: 204,
    g: 139,
    b: 102,
};
pub const OPERATOR: Color = Color::TrueColor {
    r: 158,
    g: 151,
    b: 140,
};
pub const PUNCTUATION: Color = Color::TrueColor {
    r: 138,
    g: 132,
    b: 122,
};
pub const STRING: Color = Color::TrueColor {
    r: 175,
    g: 175,
    b: 135,
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
        if self.fields.is_empty() {
            return format!("{}", "()".color(PUNCTUATION));
        }
        if self.fields.len() == 1 {
            if let Field::Inline(expr) | Field::Field(_, expr) = &self.fields[0] {
                let mut is_large = false;
                if let Expr::Struct(_) | Expr::Block(_) = expr {
                    is_large = true;
                }
                if !is_large {
                    return format!(
                        "{}{}{}",
                        "(".color(PUNCTUATION),
                        self.fields[0].pretty_print(ctxt),
                        ")".color(PUNCTUATION)
                    );
                }
            }
        }
        let mut s = String::new();
        s += &format!("{}\n", "(".color(PUNCTUATION));
        for field in &self.fields {
            s += &ctxt.indented().indent();
            s += &field.pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += &format!("{}", ")".color(PUNCTUATION));
        s
    }
}

impl PrettyPrint for Args {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        if self.args.is_empty() {
            return format!("{}", "()".color(OPERATOR));
        }
        let mut s = String::new();
        s += &format!("{}", "(".color(OPERATOR));
        for i in 0..self.args.len() {
            s += &self.args[i].pretty_print(ctxt);
            if i < self.args.len() - 1 {
                s += &format!("{} ", ",".color(PUNCTUATION));
            }
        }
        s += &format!("{}", ")".color(OPERATOR));
        s
    }
}

impl PrettyPrint for Field {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Field::Field(ident, expr) => {
                format!(
                    "{} {}",
                    match ident.is_type {
                        true => ident.name.bold().color(MEMBER),
                        false => ident.name.color(MEMBER),
                    },
                    expr.pretty_print(ctxt.indented())
                )
            }
            Field::Inline(expr) => format!(
                "{}{}",
                "..".color(PUNCTUATION),
                expr.pretty_print(ctxt.indented())
            ),
            Field::Spacer => "".to_string(),
        }
    }
}

impl PrettyPrint for Arg {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Arg::Named(is_mut, ident, expr) => {
                format!(
                    "{}{} {}",
                    (if *is_mut { "&" } else { "" }).color(PUNCTUATION),
                    match ident.is_type {
                        true => ident.name.italic().bold().color(NORMAL),
                        false => ident.name.italic().color(NORMAL),
                    },
                    expr.pretty_print(ctxt.indented())
                )
            }
            Arg::Ident(is_mut, ident) => {
                format!(
                    "{}{}",
                    (if *is_mut { "&" } else { "" }).color(PUNCTUATION),
                    match ident.is_type {
                        true => ident.name.bold().color(TYPE),
                        false => ident.name.italic().color(NORMAL),
                    },
                )
            }
        }
    }
}

impl PrettyPrint for Ident {
    fn pretty_print(&self, _: PrettyPrintContext) -> String {
        if self.is_type {
            format!("{}", self.name.as_ref().bold().color(TYPE))
        } else {
            format!("{}", self.name.as_ref().color(NORMAL))
        }
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        match self {
            Expr::Ident(ident) => ident.pretty_print(ctxt),
            Expr::Value(value) => value.pretty_print(ctxt),
            Expr::Struct(struct_) => struct_.pretty_print(ctxt),
            Expr::Block(block) => block.pretty_print(ctxt),
            Expr::Binop(lhs, op, rhs) => format!(
                "{} {} {}",
                lhs.pretty_print(ctxt),
                op.pretty_print(ctxt),
                rhs.pretty_print(ctxt)
            ),
            Expr::Func(func) => func.pretty_print(ctxt),
            Expr::Call(call) => call.pretty_print(ctxt),
            Expr::Project(project) => project.pretty_print(ctxt),
        }
    }
}

impl PrettyPrint for Value {
    fn pretty_print(&self, _: PrettyPrintContext) -> String {
        match self {
            Value::I32(n) => format!("{}", n.to_string().color(CONSTANT)),
            Value::String(s) => {
                let mut res = String::new();
                res += &format!("{}", "\"".color(STRING));
                let parts: Vec<&str> = s.split("\"").collect();
                for i in 0..parts.len() {
                    res += &format!("{}", parts[i].color(STRING));
                    if i < parts.len() - 1 {
                        res += &format!("{}", "\\\"".color(FUNCTION));
                    }
                }
                res += &format!("{}", "\"".color(STRING));
                res
            }
            Value::Char(c) => {
                let c = *c as char;
                format!(
                    "{}{}{}",
                    "'".color(STRING),
                    match c {
                        '\'' => "\\\'".color(FUNCTION),
                        _ => c.to_string().color(STRING),
                    },
                    "'".color(STRING)
                )
            }
        }
    }
}

impl PrettyPrint for Block {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        if self.stmts.is_empty() {
            return format!("{}", "{}".color(PUNCTUATION));
        }
        if self.stmts.len() == 1 {
            return format!(
                "{} {} {}",
                "{".color(PUNCTUATION),
                self.stmts[0].pretty_print(ctxt),
                "}".color(PUNCTUATION)
            );
        }
        let mut s = String::new();
        s += &format!("{}\n", "{".color(PUNCTUATION));
        for stmt in &self.stmts {
            s += &ctxt.indented().indent();
            s += &stmt.pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += &format!("{}", "}".color(PUNCTUATION));
        s
    }
}

impl PrettyPrint for Binop {
    fn pretty_print(&self, _: PrettyPrintContext) -> String {
        match self {
            Binop::Add => format!("{}", "+".color(OPERATOR)),
            Binop::Sub => format!("{}", "-".color(OPERATOR)),
            Binop::Mul => format!("{}", "*".color(OPERATOR)),
            Binop::Div => format!("{}", "/".color(OPERATOR)),
            Binop::Pow => format!("{}", "^".color(OPERATOR)),
            Binop::Concat => format!("{}", "++".color(OPERATOR)),
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
                    "=".color(OPERATOR),
                    expr.pretty_print(ctxt)
                )
            }
            Stmt::BindMut(ident, ty, expr) => format!(
                "{} {} {} {}",
                ident.pretty_print(ctxt),
                ty.pretty_print(ctxt),
                "=".color(OPERATOR),
                expr.pretty_print(ctxt)
            ),
            Stmt::Write(ident, expr) => {
                format!(
                    "{} {} {}",
                    ident.pretty_print(ctxt),
                    ":=".color(OPERATOR),
                    expr.pretty_print(ctxt)
                )
            }
            Stmt::Update(ident, binop, expr) => format!(
                "{} {}{} {}",
                ident.pretty_print(ctxt),
                binop.pretty_print(ctxt),
                "=".color(OPERATOR),
                expr.pretty_print(ctxt)
            ),
            Stmt::Expr(expr) => expr.pretty_print(ctxt).to_string(),
        }
    }
}

impl PrettyPrint for Func {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        format!(
            "{} {}",
            self.args.pretty_print(ctxt),
            self.body.pretty_print(ctxt)
        )
    }
}

impl PrettyPrint for Call {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        let mut s = String::new();
        if self.method_syntax {
            assert!(!self.args.is_empty());
            s += &self.args[0].pretty_print(ctxt);
            s += &format!("{}", ":".color(PUNCTUATION));
        }
        match &*self.func {
            Expr::Ident(ident) => {
                s += &format!(
                    "{}",
                    if ident.is_type {
                        ident.name.bold().color(TYPE)
                    } else {
                        ident.name.color(FUNCTION)
                    }
                )
            }
            _ => s += &self.func.pretty_print(ctxt),
        }
        // if !(self.method_syntax && self.args.len() == 1) {
        s += &format!("{}", "(".color(OPERATOR));
        let start_idx = if self.method_syntax { 1 } else { 0 };
        for i in start_idx..self.args.len() {
            s += &self.args[i].pretty_print(ctxt);
            if i < self.args.len() - 1 {
                s += &format!("{}", ", ".color(PUNCTUATION));
            }
        }
        s += &format!("{}", ")".color(OPERATOR));
        // }
        s
    }
}

impl PrettyPrint for Project {
    fn pretty_print(&self, ctxt: PrettyPrintContext) -> String {
        format!(
            "{}{}{}",
            self.expr.pretty_print(ctxt),
            ".".color(PUNCTUATION),
            if self.field.is_type {
                self.field.name.bold().color(TYPE)
            } else {
                self.field.name.color(NORMAL)
            }
        )
    }
}
