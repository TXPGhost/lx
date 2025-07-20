pub mod ast;
pub mod eval;
pub mod ir;

use ast::*;
// use ir::IntoIr;

use crate::ast::pretty_print::PrettyPrint;

fn main() {
    let ast = estruct(vec![
        field(vid("a"), ei32(1)),
        field(vid("b"), ei32(2)),
        field(vid("c"), ei32(3)),
        field(
            tid("Vector3"),
            estruct(vec![
                field(vid("x"), etid("I32")),
                field(vid("y"), etid("I32")),
                field(vid("z"), etid("I32")),
            ]),
        ),
        inline_struct(vec![field(vid("d"), ei32(4)), field(vid("e"), ei32(5))]),
        field(vid("ans"), add(evid("a"), evid("b"))),
        field(
            vid("block_example"),
            eblock(vec![
                bind(vid("x"), ei32(42)),
                bind_mut(vid("y"), etid("I32"), ei32(24)),
                bind(vid("z"), add(evid("x"), evid("y"))),
            ]),
        ),
        field(
            vid("result"),
            ecall(evid("get_value"), vec![ei32(1), ei32(2), evid("w")]),
        ),
        field(vid("empty_block"), eblock(vec![])),
    ]);

    println!("{}", ast.pretty_print_string());

    // let ir = ast.into_ir(None).unwrap();

    // dbg!(&ir);
}
