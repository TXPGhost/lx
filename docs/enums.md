# Enums

Enums shuold require explicit types.

```luau
Bool <true (), false ()>
..Bool

main() {
    x = true
    y Bool = false
}
```

This way, we can use the shorthand to create enums with the same names as their
variant types.

```luau
Ident <
    VIdent (value String)
    TIdent (value String)
    Void ()
>

Expr <
    Ident
    Prim <I32, String, Bool>
    Struct (fields []Field)
    Block (stmts []Stmt)
>

Field (
    ident Ident
    value Expr
)

Stmt <
    Bind (ident Ident, to Expr)
    Expr
>

main() {
    ast = Expr.Struct(fields [
        Field(ident "x", value Ident.TIdent("I32"))
        Field(ident "y", value Ident.TIdent("I32"))
        Field(ident "z", value Ident.TIdent("I32"))
    ])
}
```
