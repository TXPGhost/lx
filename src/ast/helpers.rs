use super::*;

pub fn args(args: impl Into<Vec<Arg>>) -> Args {
    Args { args: args.into() }
}

pub fn estruct(fields: impl Into<Vec<Field>>) -> Expr {
    Expr::Struct(Struct {
        fields: fields.into(),
    })
}

pub fn estring(string: impl Into<String>) -> Expr {
    Expr::Prim(Prim::String(string.into()))
}

pub fn echar(char: char) -> Expr {
    Expr::Prim(Prim::Char(char as u8))
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
    Arg::Named(false, ident, expr)
}

pub fn arg_mut(ident: Ident, expr: Expr) -> Arg {
    Arg::Named(true, ident, expr)
}

pub fn aident(ident: Ident) -> Arg {
    Arg::Ident(false, ident)
}

pub fn aident_mut(ident: Ident) -> Arg {
    Arg::Ident(true, ident)
}

pub fn fspacer() -> Field {
    Field::Spacer
}

pub fn inline(expr: Expr) -> Field {
    Field::Inline(expr)
}

pub fn eident(ident: Ident) -> Expr {
    Expr::Ident(ident)
}

pub fn vid(name: &'static str) -> Ident {
    let name: Rc<str> = name.into();
    assert!(!name.is_empty());
    assert!(!name.chars().next().unwrap().is_ascii_uppercase());
    Ident {
        name,
        is_type: false,
        is_void: false,
        nhoist: 0,
    }
}

pub fn tid(name: &'static str) -> Ident {
    let name: Rc<str> = name.into();
    assert!(!name.is_empty());
    assert!(!name.chars().next().unwrap().is_ascii_lowercase());
    Ident {
        name,
        is_type: true,
        is_void: false,
        nhoist: 0,
    }
}

pub fn void() -> Ident {
    Ident {
        name: "_".into(),
        is_type: false,
        is_void: true,
        nhoist: 0,
    }
}

pub fn hoist(ident: Ident, by: usize) -> Ident {
    Ident {
        nhoist: by,
        ..ident
    }
}

pub fn evid(name: &'static str) -> Expr {
    Expr::Ident(vid(name))
}

pub fn etid(name: &'static str) -> Expr {
    Expr::Ident(tid(name))
}

pub fn ei32(n: i32) -> Expr {
    Expr::Prim(Prim::I32(n))
}

pub fn add(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Add, Box::new(rhs))
}

pub fn sub(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Sub, Box::new(rhs))
}

pub fn mul(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Mul, Box::new(rhs))
}

pub fn div(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Div, Box::new(rhs))
}

pub fn pow(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Pow, Box::new(rhs))
}

pub fn concat(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Binop(Box::new(lhs), Binop::Concat, Box::new(rhs))
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

pub fn eproj(expr: Expr, field: Ident) -> Expr {
    Expr::Project(Project {
        expr: Box::new(expr),
        field,
    })
}

pub fn call(func: Expr, args: impl Into<Vec<Expr>>) -> Call {
    Call {
        func: Box::new(func),
        args: args.into(),
        method_syntax: false,
    }
}

pub fn ecall(func: Expr, args: impl Into<Vec<Expr>>) -> Expr {
    Expr::Call(Call {
        func: Box::new(func),
        args: args.into(),
        method_syntax: false,
    })
}

pub fn econstructor(ident: Ident, fields: impl Into<Vec<Field>>) -> Expr {
    Expr::Constructor(
        ident,
        Struct {
            fields: fields.into(),
        },
    )
}

pub fn method(obj: Expr, func: Expr, args: impl Into<Vec<Expr>>) -> Call {
    let mut args = args.into();
    args.insert(0, obj);
    Call {
        func: Box::new(func),
        args,
        method_syntax: true,
    }
}

pub fn emethod(obj: Expr, func: Expr, args: impl Into<Vec<Expr>>) -> Expr {
    let mut args = args.into();
    args.insert(0, obj);
    Expr::Call(Call {
        func: Box::new(func),
        args,
        method_syntax: true,
    })
}

pub fn sbind(ident: Ident, expr: Expr) -> Stmt {
    Stmt::Bind(ident, expr)
}

pub fn sbindmut(ident: Ident, ty: Expr, expr: Expr) -> Stmt {
    Stmt::BindMut(ident, ty, expr)
}

pub fn swrite(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Write(lhs, rhs)
}

pub fn sadd(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Update(lhs, Binop::Add, rhs)
}

pub fn ssub(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Update(lhs, Binop::Sub, rhs)
}

pub fn smul(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Update(lhs, Binop::Mul, rhs)
}

pub fn sdiv(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Update(lhs, Binop::Div, rhs)
}

pub fn spow(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Update(lhs, Binop::Pow, rhs)
}

pub fn sconcat(lhs: Expr, rhs: Expr) -> Stmt {
    Stmt::Update(lhs, Binop::Concat, rhs)
}

pub fn sexpr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}
