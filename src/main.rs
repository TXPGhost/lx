pub mod ast;
pub mod eval;
pub mod ir;

use ast::*;
// use ir::IntoIr;

use crate::ast::pretty_print::PrettyPrint;

fn main() {
    let ast = expr_struct(vec![
        field(vid("a"), ei32(1)),
        field(vid("b"), ei32(2)),
        field(vid("c"), ei32(3)),
        field(
            tid("Vector3"),
            expr_struct(vec![
                field(vid("x"), ei32(1)),
                field(vid("y"), ei32(2)),
                field(vid("z"), ei32(3)),
            ]),
        ),
        inline_struct(vec![field(vid("d"), ei32(4)), field(vid("e"), ei32(5))]),
        field(vid("ans"), add(evid("a"), evid("b"))),
        field(
            vid("block_example"),
            expr_block(vec![
                bind(vid("x"), ei32(42)),
                bind(vid("y"), ei32(24)),
                bind(vid("z"), add(evid("x"), evid("y"))),
            ]),
        ),
    ]);

    println!("{}", ast.pretty_print_string());

    // let ir = ast.into_ir(None).unwrap();

    // dbg!(&ir);
}
