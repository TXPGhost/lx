use std::{borrow::Cow, collections::HashSet};

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

#[derive(Clone)]
pub struct PrettyPrintContext<'parent> {
    pub indent_level: usize,
    pub argumets: Cow<'parent, HashSet<String>>,
    pub expand: bool,
    pub max_width: usize,
    pub colors: bool,
}

impl Default for PrettyPrintContext<'_> {
    fn default() -> Self {
        Self {
            indent_level: 0,
            argumets: Cow::Owned(HashSet::new()),
            expand: false,
            max_width: 80,
            colors: true,
        }
    }
}

impl<'parent> PrettyPrintContext<'parent> {
    pub fn indented(&self) -> PrettyPrintContext<'parent> {
        PrettyPrintContext {
            indent_level: self.indent_level + 1,
            ..self.clone()
        }
    }

    pub fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub fn add_argument(&mut self, argument: String) {
        self.argumets.to_mut().insert(argument);
    }

    pub fn remove_argument(&mut self, argument: &str) {
        self.argumets.to_mut().remove(argument);
    }

    pub fn is_argument(&self, argument: &str) -> bool {
        self.argumets.contains(argument)
    }

    pub fn with_expand(&self, expand: bool) -> PrettyPrintContext {
        PrettyPrintContext {
            expand,
            ..self.clone()
        }
    }

    pub fn with_colors(&self, colors: bool) -> PrettyPrintContext {
        PrettyPrintContext {
            colors,
            ..self.clone()
        }
    }

    pub fn color(&self, str: impl Into<String>, color: Color) -> String {
        if self.colors {
            format!("{}", str.into().color(color))
        } else {
            str.into()
        }
    }

    pub fn style(&self, str: impl Into<String>, color: Color, bold: bool, italic: bool) -> String {
        if self.colors {
            match (bold, italic) {
                (false, false) => format!("{}", str.into().color(color)),
                (false, true) => format!("{}", str.into().italic().color(color)),
                (true, false) => format!("{}", str.into().bold().color(color)),
                (true, true) => format!("{}", str.into().bold().italic().color(color)),
            }
        } else {
            str.into()
        }
    }
}

pub trait PrettyPrint {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String;
    fn pretty_print_string(&self) -> String {
        self.pretty_print(&mut PrettyPrintContext::default())
    }
}

impl PrettyPrint for Struct {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        if self.fields.is_empty() {
            return ctxt.color("()", PUNCTUATION);
        }
        if self.fields.len() == 1 {
            if let Field::Inline(expr) | Field::Field(_, expr) = &self.fields[0] {
                let mut is_large = false;
                if let Expr::Struct(_) | Expr::Block(_) = expr {
                    is_large = true;
                }
                if !is_large {
                    return ctxt.color("(", PUNCTUATION)
                        + &self.fields[0].pretty_print(ctxt)
                        + &ctxt.color(")", PUNCTUATION);
                }
            }
        }
        let mut s = String::new();
        s += &ctxt.color("(", PUNCTUATION);
        if ctxt.expand {
            s += "\n";
        }
        for i in 0..self.fields.len() {
            if ctxt.expand {
                s += &ctxt.indented().indent();
            }
            s += &self.fields[i].pretty_print(ctxt);
            if ctxt.expand {
                s += "\n";
            } else if i < self.fields.len() - 1 {
                s += &ctxt.color(",", PUNCTUATION);
                s += " ";
            }
        }
        if ctxt.expand {
            s += &ctxt.indent();
        }
        s += &ctxt.color(")", PUNCTUATION);
        s
    }
}

impl PrettyPrint for Args {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        if self.args.is_empty() {
            return ctxt.color("()", OPERATOR);
        }
        let mut s = String::new();
        s += &ctxt.color("(", OPERATOR);
        for i in 0..self.args.len() {
            s += &self.args[i].pretty_print(ctxt);
            if i < self.args.len() - 1 {
                s += &ctxt.color(",", PUNCTUATION);
                s += " ";
            }
        }
        s += &ctxt.color(")", OPERATOR);
        s
    }
}

impl PrettyPrint for Field {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Field::Field(ident, expr) => {
                if let Expr::Ident(other) = expr
                    && ident.is_type == other.is_type
                    && !ident.is_void
                    && !other.is_void
                    && ident.name == other.name
                    && ident.nhoist == 0
                    && other.nhoist == 1
                {
                    return match ident.is_type {
                        true => ctxt.style(&*ident.name, MEMBER, true, true),
                        false => ctxt.style(&*ident.name, MEMBER, false, true),
                    };
                }

                (match ident.is_type {
                    true => ctxt.style(&*ident.name, MEMBER, true, false),
                    false => ctxt.color(&*ident.name, MEMBER),
                } + " "
                    + &expr.pretty_print(&mut ctxt.indented()))
            }
            Field::Inline(expr) => {
                ctxt.color("..", PUNCTUATION) + &expr.pretty_print(&mut ctxt.indented())
            }
            Field::Spacer => "".to_string(),
        }
    }
}

impl PrettyPrint for Arg {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Arg::Named(is_mut, ident, expr) => {
                ctxt.add_argument(ident.name.as_ref().to_owned());
                ctxt.color(if *is_mut { "&" } else { "" }, PUNCTUATION)
                    + &match ident.is_type {
                        true => ctxt.style(&*ident.name, NORMAL, true, true),
                        false => ctxt.style(&*ident.name, NORMAL, false, true),
                    }
                    + " "
                    + &expr.pretty_print(&mut ctxt.indented())
            }
            Arg::Ident(is_mut, ident) => {
                ctxt.add_argument(ident.name.as_ref().to_owned());
                ctxt.color(if *is_mut { "&" } else { "" }, PUNCTUATION)
                    + &match ident.is_type {
                        true => ctxt.style(&*ident.name, TYPE, true, false),
                        false => ctxt.style(&*ident.name, NORMAL, false, true),
                    }
            }
        }
    }
}

impl PrettyPrint for Ident {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        ctxt.color("&".repeat(self.nhoist), PUNCTUATION)
            + &match (self.is_type, ctxt.is_argument(&self.name)) {
                (true, true) => ctxt.style(&*self.name, TYPE, true, true),
                (true, false) => ctxt.style(&*self.name, TYPE, true, false),
                (false, true) => ctxt.style(&*self.name, NORMAL, false, true),
                (false, false) => ctxt.style(&*self.name, NORMAL, false, false),
            }
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Expr::Ident(ident) => ident.pretty_print(ctxt),
            Expr::Value(value) => value.pretty_print(ctxt),
            Expr::Struct(struct_) => {
                let res = struct_.pretty_print(&mut ctxt.with_expand(false).with_colors(false));
                if res.lines().map(|s| s.chars().count()).max().unwrap_or(0) > ctxt.max_width {
                    struct_.pretty_print(&mut ctxt.with_expand(true))
                } else {
                    struct_.pretty_print(&mut ctxt.with_expand(false))
                }
            }
            Expr::Block(block) => block.pretty_print(ctxt),
            Expr::Binop(lhs, op, rhs) => format!(
                "{} {} {}",
                lhs.pretty_print(ctxt),
                op.pretty_print(ctxt),
                rhs.pretty_print(ctxt)
            ),
            Expr::Func(func) => func.pretty_print(ctxt),
            Expr::Call(call) => call.pretty_print(ctxt),
            Expr::Constructor(ident, constructor) => {
                let res = constructor.pretty_print(&mut ctxt.with_expand(false).with_colors(false));
                let res =
                    if res.lines().map(|s| s.chars().count()).max().unwrap_or(0) > ctxt.max_width {
                        constructor.pretty_print(&mut ctxt.with_expand(true))
                    } else {
                        constructor.pretty_print(&mut ctxt.with_expand(false))
                    };
                format!("{}{}", ident.pretty_print(ctxt), res)
            }
            Expr::Project(project) => project.pretty_print(ctxt),
        }
    }
}

impl PrettyPrint for Value {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Value::I32(n) => ctxt.color(n.to_string(), CONSTANT),
            Value::String(s) => {
                let mut res = String::new();
                res += &ctxt.color("\"", STRING);
                let parts: Vec<&str> = s.split("\"").collect();
                for i in 0..parts.len() {
                    res += &ctxt.color(parts[i], STRING);
                    if i < parts.len() - 1 {
                        res += &ctxt.color("\\\"", FUNCTION);
                    }
                }
                res += &ctxt.color("\"", STRING);
                res
            }
            Value::Char(c) => {
                let c = *c as char;
                ctxt.color("'", STRING)
                    + &match c {
                        '\'' => ctxt.color("\\\'", FUNCTION),
                        _ => ctxt.color(c.to_string(), STRING),
                    }
                    + &ctxt.color("'", STRING)
            }
        }
    }
}

impl PrettyPrint for Block {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        if self.stmts.is_empty() {
            ctxt.color("{}", PUNCTUATION);
        }
        if self.stmts.len() == 1 {
            return ctxt.color("{", PUNCTUATION)
                + " "
                + &self.stmts[0].pretty_print(ctxt)
                + " "
                + &ctxt.color("}", PUNCTUATION);
        }
        let mut s = String::new();
        s += &ctxt.color("{", PUNCTUATION);
        s += "\n";
        for stmt in &self.stmts {
            s += &ctxt.indented().indent();
            s += &stmt.pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += &ctxt.color("}", PUNCTUATION);
        s
    }
}

impl PrettyPrint for Binop {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Binop::Add => ctxt.color("+", OPERATOR),
            Binop::Sub => ctxt.color("-", OPERATOR),
            Binop::Mul => ctxt.color("*", OPERATOR),
            Binop::Div => ctxt.color("/", OPERATOR),
            Binop::Pow => ctxt.color("^", OPERATOR),
            Binop::Concat => ctxt.color("++", OPERATOR),
        }
    }
}

impl PrettyPrint for Stmt {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Stmt::Bind(ident, expr) => {
                let expr = expr.pretty_print(ctxt);
                ctxt.remove_argument(&ident.name);
                ident.pretty_print(ctxt) + " " + &ctxt.color("=", OPERATOR) + " " + &expr
            }
            Stmt::BindMut(ident, ty, expr) => {
                let ty = ty.pretty_print(ctxt);
                let expr = expr.pretty_print(ctxt);
                ctxt.remove_argument(&ident.name);
                ident.pretty_print(ctxt)
                    + " "
                    + &ty
                    + " "
                    + &ctxt.color("=", OPERATOR)
                    + " "
                    + &expr
            }
            Stmt::Write(ident, expr) => {
                ident.pretty_print(ctxt)
                    + " "
                    + &ctxt.color(":=", OPERATOR)
                    + " "
                    + &expr.pretty_print(ctxt)
            }
            Stmt::Update(ident, binop, expr) => {
                ident.pretty_print(ctxt)
                    + " "
                    + &binop.pretty_print(ctxt)
                    + &ctxt.color("=", OPERATOR)
                    + " "
                    + &expr.pretty_print(ctxt)
            }
            Stmt::Expr(expr) => expr.pretty_print(ctxt).to_string(),
        }
    }
}

impl PrettyPrint for Func {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        self.args.pretty_print(ctxt) + " " + &self.body.pretty_print(ctxt)
    }
}

impl PrettyPrint for Call {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        let mut s = String::new();
        if self.method_syntax {
            assert!(!self.args.is_empty());
            s += &self.args[0].pretty_print(ctxt);
            s += &ctxt.color(":", PUNCTUATION);
        }
        let mut angle_brackets = false;
        let mut func = self.func.as_ref().clone();
        loop {
            match func {
                Expr::Ident(ident) => {
                    s += &if ident.is_type {
                        angle_brackets = true;
                        ctxt.style(&*ident.name, TYPE, true, false)
                    } else {
                        ctxt.color(&*ident.name, FUNCTION)
                    };
                    break;
                }
                Expr::Project(Project { expr, field }) => {
                    s += &expr.pretty_print(ctxt).to_string();
                    s += &ctxt.color(".", PUNCTUATION);
                    func = Expr::Ident(field.clone());
                    continue;
                }
                _ => {
                    s += &self.func.pretty_print(ctxt);
                    break;
                }
            }
        }
        s += &ctxt.color(if angle_brackets { "<" } else { "(" }, OPERATOR);
        let start_idx = if self.method_syntax { 1 } else { 0 };
        for i in start_idx..self.args.len() {
            s += &self.args[i].pretty_print(ctxt);
            if i < self.args.len() - 1 {
                s += &ctxt.color(",", PUNCTUATION);
                s += " ";
            }
        }
        s += &ctxt.color(if angle_brackets { ">" } else { ")" }, OPERATOR);
        s
    }
}

impl PrettyPrint for Project {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        self.expr.pretty_print(ctxt)
            + &ctxt.color(".", PUNCTUATION)
            + &if self.field.is_type {
                ctxt.style(&*self.field.name, TYPE, true, false)
            } else {
                ctxt.color(&*self.field.name, NORMAL)
            }
    }
}
