pub mod ast;
pub mod eval;
pub mod ir;

use ast::*;
// use ir::IntoIr;

use crate::ast::pretty_print::PrettyPrint;

fn main() {
    let ast = estruct([
        field(vid("a"), ei32(1)),
        field(vid("b"), ei32(2)),
        field(vid("c"), ei32(3)),
        field(
            tid("Vector3"),
            estruct([
                field(vid("x"), etid("I32")),
                field(vid("y"), etid("I32")),
                field(vid("z"), etid("I32")),
            ]),
        ),
        istruct([field(vid("d"), ei32(4)), field(vid("e"), ei32(5))]),
        field(vid("ans"), add(evid("a"), evid("b"))),
        field(
            vid("block_example"),
            eblock([
                sbind(vid("x"), ei32(42)),
                sbindmut(vid("y"), etid("I32"), ei32(24)),
                sbind(vid("z"), add(evid("x"), evid("y"))),
                sbind(vid("tmp"), estring("some string value")),
                sexpr(evid("z")),
            ]),
        ),
        field(
            vid("result"),
            ecall(evid("get_value"), [ei32(1), ei32(2), evid("w")]),
        ),
        field(vid("empty_block"), eblock([])),
        field(vid("unit_block"), eblock([sexpr(ecall(evid("func"), []))])),
        field(vid("unit"), estruct([])),
        field(
            vid("single"),
            estruct([field(vid("value"), etid("String"))]),
        ),
        field(vid("msg"), estring("Hello, world!")),
    ]);

    println!("{}", ast.pretty_print_string());

    // let ir = ast.into_ir(None).unwrap();

    // dbg!(&ir);
}
