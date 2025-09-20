use std::{borrow::Cow, collections::HashSet};

use colored::{Color, Colorize};

use super::*;
use crate::colorscheme::Colorscheme;

#[derive(Clone)]
pub struct PrettyPrintContext<'parent> {
    pub indent_level: usize,
    pub argumets: Cow<'parent, HashSet<String>>,
    pub expand: bool,
    pub max_width: usize,
    pub colors: bool,
    pub cs: Colorscheme,
}

impl Default for PrettyPrintContext<'_> {
    fn default() -> Self {
        Self {
            indent_level: 0,
            argumets: Cow::Owned(HashSet::new()),
            expand: false,
            max_width: 80,
            colors: true,
            cs: Colorscheme::default(),
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

impl<'a, M: NodeMeta> PrettyPrint for Struct<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        if self.fields.is_empty() {
            return ctxt.color("()", ctxt.cs.punctuation);
        }
        if self.fields.len() == 1 {
            let field = &self.fields[0].elt.borrow();
            let mut is_large = false;
            match &**field {
                Field::Inline(expr) => {
                    if let Expr::Struct(_) | Expr::Block(_) = expr {
                        is_large = true;
                    }
                }
                Field::Field(_, expr) => {
                    if let Expr::Struct(_) | Expr::Block(_) = &*expr.elt.borrow() {
                        is_large = true;
                    }
                }
                Field::Spacer => {}
            }
            if !is_large {
                return ctxt.color("(", ctxt.cs.punctuation)
                    + &field.pretty_print(ctxt)
                    + &ctxt.color(")", ctxt.cs.punctuation);
            }
        }
        let mut s = String::new();
        s += &ctxt.color("(", ctxt.cs.punctuation);
        if ctxt.expand {
            s += "\n";
        }
        for i in 0..self.fields.len() {
            if ctxt.expand {
                s += &ctxt.indented().indent();
            }
            s += &self.fields[i].elt.borrow().pretty_print(ctxt);
            if ctxt.expand {
                s += "\n";
            } else if i < self.fields.len() - 1 {
                s += &ctxt.color(",", ctxt.cs.punctuation);
                s += " ";
            }
        }
        if ctxt.expand {
            s += &ctxt.indent();
        }
        s += &ctxt.color(")", ctxt.cs.punctuation);
        s
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Params<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        if self.params.is_empty() {
            return ctxt.color("()", ctxt.cs.operator);
        }
        let mut s = String::new();
        s += &ctxt.color("(", ctxt.cs.operator);
        for i in 0..self.params.len() {
            s += &self.params[i].elt.borrow().pretty_print(ctxt);
            if i < self.params.len() - 1 {
                s += &ctxt.color(",", ctxt.cs.punctuation);
                s += " ";
            }
        }
        s += &ctxt.color(")", ctxt.cs.operator);
        s
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Field<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Field::Field(ident, expr) => {
                if let Expr::Ident(other) = &*expr.elt.borrow()
                    && ident.is_type == other.is_type
                    && !ident.is_void
                    && !other.is_void
                    && ident.name == other.name
                    && ident.nshadow == 0
                    && other.nshadow == 1
                {
                    return match ident.is_type {
                        true => ctxt.style(&*ident.name, ctxt.cs.member, true, true),
                        false => ctxt.style(&*ident.name, ctxt.cs.member, false, true),
                    };
                }

                (match ident.is_type {
                    true => ctxt.style(&*ident.name, ctxt.cs.member, true, false),
                    false => ctxt.color(&*ident.name, ctxt.cs.member),
                } + (if let Expr::Func(_) = *expr.elt.borrow() {
                    ""
                } else {
                    " "
                }) + &expr.elt.borrow().pretty_print(&mut ctxt.indented()))
            }
            Field::Inline(expr) => {
                ctxt.color("..", ctxt.cs.punctuation) + &expr.pretty_print(&mut ctxt.indented())
            }
            Field::Spacer => "".to_string(),
        }
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Param<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        ctxt.add_argument(self.ident.name.as_ref().to_owned());
        ctxt.color(if self.is_mut { "*" } else { "" }, ctxt.cs.punctuation)
            + &match self.ident.is_type {
                true => ctxt.style(&*self.ident.name, ctxt.cs.normal, true, true),
                false => ctxt.style(&*self.ident.name, ctxt.cs.normal, false, true),
            }
            + " "
            + &self.expr.elt.borrow().pretty_print(&mut ctxt.indented())
    }
}

impl PrettyPrint for Ident {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        ctxt.color("^".repeat(self.nshadow), ctxt.cs.punctuation)
            + &match (self.is_type, ctxt.is_argument(&self.name)) {
                (true, true) => ctxt.style(&*self.name, ctxt.cs.type_, true, true),
                (true, false) => ctxt.style(&*self.name, ctxt.cs.type_, true, false),
                (false, true) => ctxt.style(&*self.name, ctxt.cs.normal, false, true),
                (false, false) => ctxt.style(&*self.name, ctxt.cs.normal, false, false),
            }
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Expr<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Expr::Ident(ident) => ident.pretty_print(ctxt),
            Expr::Prim(value) => value.pretty_print(ctxt),
            Expr::Struct(struct_) => {
                let res = struct_.pretty_print(&mut ctxt.with_expand(false).with_colors(false));
                if res.lines().map(|s| s.chars().count()).max().unwrap_or(0) > ctxt.max_width {
                    struct_.pretty_print(&mut ctxt.with_expand(true))
                } else {
                    struct_.pretty_print(&mut ctxt.with_expand(false))
                }
            }
            Expr::Block(block) => block.pretty_print(ctxt),
            Expr::Unop(unop) => format!(
                "{}{}",
                unop.op.pretty_print(ctxt),
                unop.expr.elt.borrow().pretty_print(ctxt)
            ),
            Expr::Binop(binop) => format!(
                "{} {} {}",
                binop.lhs.elt.borrow().pretty_print(ctxt),
                binop.op.pretty_print(ctxt),
                binop.rhs.elt.borrow().pretty_print(ctxt)
            ),
            Expr::Func(func) => func.pretty_print(ctxt),
            Expr::Call(call) => call.pretty_print(ctxt),
            Expr::Constructor(constructor) => {
                let res = constructor
                    .fields
                    .pretty_print(&mut ctxt.with_expand(false).with_colors(false));
                let res =
                    if res.lines().map(|s| s.chars().count()).max().unwrap_or(0) > ctxt.max_width {
                        constructor.fields.pretty_print(&mut ctxt.with_expand(true))
                    } else {
                        constructor
                            .fields
                            .pretty_print(&mut ctxt.with_expand(false))
                    };
                ctxt.color("..", ctxt.cs.punctuation) + &constructor.name.pretty_print(ctxt) + &res
            }
            Expr::Project(project) => project.pretty_print(ctxt),
        }
    }
}

impl PrettyPrint for Prim {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Prim::I32(n) => ctxt.color(n.to_string(), ctxt.cs.constant),
            Prim::String(s) => {
                let mut res = String::new();
                res += &ctxt.color("\"", ctxt.cs.string);
                let parts: Vec<&str> = s.split("\"").collect();
                for i in 0..parts.len() {
                    res += &ctxt.color(parts[i], ctxt.cs.string);
                    if i < parts.len() - 1 {
                        res += &ctxt.color("\\\"", ctxt.cs.function);
                    }
                }
                res += &ctxt.color("\"", ctxt.cs.string);
                res
            }
            Prim::Char(c) => {
                let c = *c as char;
                ctxt.color("'", ctxt.cs.string)
                    + &match c {
                        '\'' => ctxt.color("\\\'", ctxt.cs.function),
                        _ => ctxt.color(c.to_string(), ctxt.cs.string),
                    }
                    + &ctxt.color("'", ctxt.cs.string)
            }
        }
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Block<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        if self.stmts.is_empty() {
            ctxt.color("{}", ctxt.cs.punctuation);
        }
        if self.stmts.len() == 1 {
            return ctxt.color("{", ctxt.cs.punctuation)
                + " "
                + &self.stmts[0].elt.borrow().pretty_print(ctxt)
                + " "
                + &ctxt.color("}", ctxt.cs.punctuation);
        }
        let mut s = String::new();
        s += &ctxt.color("{", ctxt.cs.punctuation);
        s += "\n";
        for stmt in &self.stmts {
            s += &ctxt.indented().indent();
            s += &stmt.elt.borrow().pretty_print(ctxt);
            s += "\n";
        }
        s += &ctxt.indent();
        s += &ctxt.color("}", ctxt.cs.punctuation);
        s
    }
}

impl PrettyPrint for UnopKind {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            UnopKind::Copy => ctxt.color("+", ctxt.cs.operator),
            UnopKind::Neg => ctxt.color("-", ctxt.cs.operator),
        }
    }
}

impl PrettyPrint for BinopKind {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            BinopKind::Add => ctxt.color("+", ctxt.cs.operator),
            BinopKind::Sub => ctxt.color("-", ctxt.cs.operator),
            BinopKind::Mul => ctxt.color("*", ctxt.cs.operator),
            BinopKind::Div => ctxt.color("/", ctxt.cs.operator),
            BinopKind::Pow => ctxt.color("^", ctxt.cs.operator),
            BinopKind::Concat => ctxt.color("++", ctxt.cs.operator),
        }
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Stmt<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        match self {
            Stmt::Bind(bind) => {
                let expr = bind.value.elt.borrow().pretty_print(ctxt);
                ctxt.remove_argument(&bind.name.name);
                bind.name.pretty_print(ctxt)
                    + " "
                    + &ctxt.color("=", ctxt.cs.operator)
                    + " "
                    + &expr
            }
            Stmt::BindMut(bind_mut) => {
                let ty = bind_mut.initial.elt.borrow().pretty_print(ctxt);
                let expr = bind_mut.update.elt.borrow().pretty_print(ctxt);
                ctxt.remove_argument(&bind_mut.name.name);
                bind_mut.name.pretty_print(ctxt)
                    + " "
                    + &ty
                    + " "
                    + &ctxt.color("=", ctxt.cs.operator)
                    + " "
                    + &expr
            }
            Stmt::Write(write) => {
                write.target.elt.borrow().pretty_print(ctxt)
                    + " "
                    + &ctxt.color(":=", ctxt.cs.operator)
                    + " "
                    + &write.value.elt.borrow().pretty_print(ctxt)
            }
            Stmt::Update(update) => {
                update.target.elt.borrow().pretty_print(ctxt)
                    + " "
                    + &update.op.pretty_print(ctxt)
                    + &ctxt.color("=", ctxt.cs.operator)
                    + " "
                    + &update.value.elt.borrow().pretty_print(ctxt)
            }
            Stmt::Expr(expr) => expr.elt.borrow().pretty_print(ctxt).to_string(),
            Stmt::Spacer => "".to_owned(),
        }
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Func<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        self.params.elt.borrow().pretty_print(ctxt)
            + " "
            + &self.body.elt.borrow().pretty_print(ctxt)
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Call<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        let mut s = String::new();
        if self.method_syntax {
            assert!(!self.args.elt.borrow().args.is_empty());
            s += &self.args.elt.borrow().args[0]
                .elt
                .borrow()
                .expr
                .elt
                .borrow()
                .pretty_print(ctxt);
            s += &ctxt.color(":", ctxt.cs.punctuation);
        }
        let mut func = self.func.elt.borrow().clone();
        loop {
            match &func {
                Expr::Ident(ident) => {
                    s += &if ident.is_type {
                        ctxt.style(&*ident.name, ctxt.cs.type_, true, false)
                    } else {
                        ctxt.color(&*ident.name, ctxt.cs.function)
                    };
                    break;
                }
                Expr::Project(Project { expr, field }) => {
                    s += &expr.elt.borrow().pretty_print(ctxt).to_string();
                    s += &ctxt.color(".", ctxt.cs.punctuation);
                    func = Expr::Ident(field.clone());
                    continue;
                }
                _ => {
                    s += &self.func.elt.borrow().pretty_print(ctxt);
                    break;
                }
            }
        }
        s += &ctxt.color("(", ctxt.cs.operator);
        let start_idx = if self.method_syntax { 1 } else { 0 };
        for i in start_idx..self.args.elt.borrow().args.len() {
            s += &self.args.elt.borrow().args[i]
                .elt
                .borrow()
                .expr
                .elt
                .borrow()
                .pretty_print(ctxt);
            if i < self.args.elt.borrow().args.len() - 1 {
                s += &ctxt.color(",", ctxt.cs.punctuation);
                s += " ";
            }
        }
        s += &ctxt.color(")", ctxt.cs.operator);
        s
    }
}

impl<'a, M: NodeMeta> PrettyPrint for Project<'a, M> {
    fn pretty_print(&self, ctxt: &mut PrettyPrintContext) -> String {
        self.expr.elt.borrow().pretty_print(ctxt)
            + &ctxt.color(".", ctxt.cs.punctuation)
            + &if self.field.is_type {
                ctxt.style(&*self.field.name, ctxt.cs.type_, true, false)
            } else {
                ctxt.color(&*self.field.name, ctxt.cs.normal)
            }
    }
}
