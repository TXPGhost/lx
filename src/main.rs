pub mod ast;
pub mod ir;

use ast::*;
use ir::IntoIr;

fn main() {
    println!("Hello, world!");

    let ast = Struct {
        fields: vec![
            Field::Field(vid("a"), Expr::I32(1)),
            Field::Field(vid("b"), Expr::I32(2)),
            Field::Field(vid("c"), Expr::I32(3)),
            Field::Field(
                tid("Vector3"),
                Expr::Struct(Struct {
                    fields: vec![
                        Field::Field(vid("x"), Expr::I32(1)),
                        Field::Field(vid("y"), Expr::I32(2)),
                        Field::Field(vid("z"), Expr::I32(3)),
                    ],
                }),
            ),
            Field::Inline(Expr::Struct(Struct {
                fields: vec![
                    Field::Field(vid("d"), Expr::I32(4)),
                    Field::Field(vid("e"), Expr::I32(5)),
                ],
            })),
        ],
    };

    dbg!(&ast);

    let ir = ast.into_ir(None).unwrap();

    dbg!(&ir);
}
