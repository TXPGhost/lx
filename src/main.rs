pub mod ast;
pub mod ir;

use ast::*;
use ir::IntoIr;

fn main() {
    println!("Hello, world!");

    let module = Struct {
        fields: vec![
            Field::Field(vid("x"), Expr::I32(1)),
            Field::Field(vid("y"), Expr::I32(2)),
            Field::Field(vid("z"), Expr::I32(3)),
        ],
    };

    dbg!(&module);
}
