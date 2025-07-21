use super::*;

pub fn args<'a, M: NodeMeta>(args: impl Into<Vec<Node<Arg<'a, M>, M>>>) -> Args<'a, M> {
    Args { args: args.into() }
}

pub fn estruct<'a, M: NodeMeta>(
    fields: impl Into<Vec<Node<Field<'a, M>, M>>>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Struct(Struct {
            fields: fields.into(),
        }),
        M::default(),
    )
}

pub fn estring<'a, M: NodeMeta>(string: impl Into<String>) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Prim(Prim::String(string.into())), M::default())
}

pub fn echar<'a, M: NodeMeta>(char: char) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Prim(Prim::Char(char as u8)), M::default())
}

pub fn istruct<'a, M: NodeMeta>(
    fields: impl Into<Vec<Node<Field<'a, M>, M>>>,
) -> Node<Field<'a, M>, M> {
    Node::new(
        Field::Inline(Expr::Struct(Struct {
            fields: fields.into(),
        })),
        M::default(),
    )
}

pub fn field<'a, M: NodeMeta>(ident: Ident, expr: Node<Expr<'a, M>, M>) -> Node<Field<'a, M>, M> {
    Node::new(Field::Field(ident, expr), M::default())
}

pub fn arg<'a, M: NodeMeta>(ident: Ident, expr: Node<Expr<'a, M>, M>) -> Node<Arg<'a, M>, M> {
    Node::new(
        Arg::Named(NamedArg {
            is_mut: false,
            name: ident,
            value: expr,
        }),
        M::default(),
    )
}

pub fn arg_mut<'a, M: NodeMeta>(ident: Ident, expr: Node<Expr<'a, M>, M>) -> Node<Arg<'a, M>, M> {
    Node::new(
        Arg::Named(NamedArg {
            is_mut: true,
            name: ident,
            value: expr,
        }),
        M::default(),
    )
}

pub fn aident<M: NodeMeta>(ident: Ident) -> Node<Arg<'static, M>, M> {
    Node::new(
        Arg::Ident(IdentArg {
            is_mut: false,
            name: ident,
        }),
        M::default(),
    )
}

pub fn aident_mut<M: NodeMeta>(ident: Ident) -> Node<Arg<'static, M>, M> {
    Node::new(
        Arg::Ident(IdentArg {
            is_mut: true,
            name: ident,
        }),
        M::default(),
    )
}

pub fn fspacer<'a, M: NodeMeta>() -> Node<Field<'a, M>, M> {
    Node::new(Field::Spacer, M::default())
}

pub fn inline<'a, M: NodeMeta>(expr: Expr<'a, M>) -> Node<Field<'a, M>, M> {
    Node::new(Field::Inline(expr), M::default())
}

pub fn eident<'a, M: NodeMeta>(ident: Ident) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Ident(ident), M::default())
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

pub fn evid<'a, M: NodeMeta>(name: &'static str) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Ident(vid(name)), M::default())
}

pub fn etid<'a, M: NodeMeta>(name: &'static str) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Ident(tid(name)), M::default())
}

pub fn ei32<'a, M: NodeMeta>(n: i32) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Prim(Prim::I32(n)), M::default())
}

pub fn add<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Binop(Binop {
            left: lhs,
            op: BinopKind::Add,
            right: rhs,
        }),
        M::default(),
    )
}

pub fn sub<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Binop(Binop {
            left: lhs,
            op: BinopKind::Sub,
            right: rhs,
        }),
        M::default(),
    )
}

pub fn mul<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Binop(Binop {
            left: lhs,
            op: BinopKind::Mul,
            right: rhs,
        }),
        M::default(),
    )
}

pub fn div<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Binop(Binop {
            left: lhs,
            op: BinopKind::Div,
            right: rhs,
        }),
        M::default(),
    )
}

pub fn pow<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Binop(Binop {
            left: lhs,
            op: BinopKind::Pow,
            right: rhs,
        }),
        M::default(),
    )
}

pub fn concat<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Binop(Binop {
            left: lhs,
            op: BinopKind::Concat,
            right: rhs,
        }),
        M::default(),
    )
}

pub fn block<'a, M: NodeMeta>(stmts: impl Into<Vec<Node<Stmt<'a, M>, M>>>) -> Block<'a, M> {
    Block {
        stmts: stmts.into(),
    }
}

pub fn eblock<'a, M: NodeMeta>(
    stmts: impl Into<Vec<Node<Stmt<'a, M>, M>>>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Block(Block {
            stmts: stmts.into(),
        }),
        M::default(),
    )
}

pub fn efunc<'a, M: NodeMeta>(
    args: Args<'a, M>,
    body: Node<Expr<'a, M>, M>,
) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Func(Func { args, body }), M::default())
}

pub fn eproj<'a, M: NodeMeta>(expr: Node<Expr<'a, M>, M>, field: Ident) -> Node<Expr<'a, M>, M> {
    Node::new(Expr::Project(Project { expr, field }), M::default())
}

pub fn call<'a, M: NodeMeta>(
    func: Node<Expr<'a, M>, M>,
    args: impl Into<Vec<Node<Expr<'a, M>, M>>>,
) -> Call<'a, M> {
    Call {
        func,
        args: args.into(),
        method_syntax: false,
    }
}

pub fn ecall<'a, M: NodeMeta>(
    func: Node<Expr<'a, M>, M>,
    args: impl Into<Vec<Node<Expr<'a, M>, M>>>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Call(Call {
            func,
            args: args.into(),
            method_syntax: false,
        }),
        M::default(),
    )
}

pub fn econstructor<'a, M: NodeMeta>(
    ident: Ident,
    fields: impl Into<Vec<Node<Field<'a, M>, M>>>,
) -> Node<Expr<'a, M>, M> {
    Node::new(
        Expr::Constructor(Constructor {
            name: ident,
            fields: Struct {
                fields: fields.into(),
            },
        }),
        M::default(),
    )
}

pub fn method<'a, M: NodeMeta>(
    obj: Node<Expr<'a, M>, M>,
    func: Node<Expr<'a, M>, M>,
    args: impl Into<Vec<Node<Expr<'a, M>, M>>>,
) -> Call<'a, M> {
    let mut args = args.into();
    args.insert(0, obj);
    Call {
        func,
        args,
        method_syntax: true,
    }
}

pub fn emethod<'a, M: NodeMeta>(
    obj: Node<Expr<'a, M>, M>,
    func: Node<Expr<'a, M>, M>,
    args: impl Into<Vec<Node<Expr<'a, M>, M>>>,
) -> Node<Expr<'a, M>, M> {
    let mut args = args.into();
    args.insert(0, obj);
    Node::new(
        Expr::Call(Call {
            func,
            args,
            method_syntax: true,
        }),
        M::default(),
    )
}

pub fn sbind<'a, M: NodeMeta>(ident: Ident, expr: Node<Expr<'a, M>, M>) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Bind(Bind {
            name: ident,
            value: expr,
        }),
        M::default(),
    )
}

pub fn sbindmut<'a, M: NodeMeta>(
    ident: Ident,
    ty: Node<Expr<'a, M>, M>,
    expr: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::BindMut(BindMut {
            name: ident,
            initial: ty,
            update: expr,
        }),
        M::default(),
    )
}

pub fn swrite<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Write(Write {
            target: lhs,
            value: rhs,
        }),
        M::default(),
    )
}

pub fn sadd<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Update(Update {
            target: lhs,
            op: BinopKind::Add,
            value: rhs,
        }),
        M::default(),
    )
}

pub fn ssub<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Update(Update {
            target: lhs,
            op: BinopKind::Sub,
            value: rhs,
        }),
        M::default(),
    )
}

pub fn smul<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Update(Update {
            target: lhs,
            op: BinopKind::Mul,
            value: rhs,
        }),
        M::default(),
    )
}

pub fn sdiv<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Update(Update {
            target: lhs,
            op: BinopKind::Div,
            value: rhs,
        }),
        M::default(),
    )
}

pub fn spow<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Update(Update {
            target: lhs,
            op: BinopKind::Pow,
            value: rhs,
        }),
        M::default(),
    )
}

pub fn sconcat<'a, M: NodeMeta>(
    lhs: Node<Expr<'a, M>, M>,
    rhs: Node<Expr<'a, M>, M>,
) -> Node<Stmt<'a, M>, M> {
    Node::new(
        Stmt::Update(Update {
            target: lhs,
            op: BinopKind::Concat,
            value: rhs,
        }),
        M::default(),
    )
}

pub fn sexpr<'a, M: NodeMeta>(expr: Node<Expr<'a, M>, M>) -> Node<Stmt<'a, M>, M> {
    Node::new(Stmt::Expr(expr), M::default())
}
