pub mod ast;
pub mod eval;
pub mod ir;

use ast::*;
// use ir::IntoIr;

use crate::ast::pretty_print::PrettyPrint;

fn main() {
    let ast = estruct([
        inline(evid("std")),
        fspacer(),
        field(vid("first"), ei32(1)),
        field(vid("second"), ei32(2)),
        field(vid("third"), ei32(3)),
        fspacer(),
        field(
            tid("Vector3"),
            estruct([
                field(vid("x"), etid("I32")),
                field(vid("y"), etid("I32")),
                field(vid("z"), etid("I32")),
            ]),
        ),
        fspacer(),
        istruct([field(vid("d"), ei32(4)), field(vid("e"), ei32(5))]),
        fspacer(),
        field(vid("ans"), add(evid("a"), evid("b"))),
        field(
            vid("block_example"),
            eblock([
                sbind(vid("x"), ei32(42)),
                sbindmut(vid("y"), etid("I32"), ei32(24)),
                sbind(vid("z"), add(evid("x"), evid("y"))),
                sbind(vid("tmp"), estring("some string value")),
                sbind(vid("escaped"), estring("some \"escaped\" string")),
                sbind(vid("char"), echar('x')),
                sbind(vid("escaped_char"), echar('\'')),
            ]),
        ),
        fspacer(),
        field(
            vid("result"),
            ecall(evid("get_value"), [ei32(1), ei32(2), evid("w")]),
        ),
        fspacer(),
        field(vid("empty_block"), eblock([])),
        fspacer(),
        field(vid("unit_block"), eblock([sexpr(ecall(evid("func"), []))])),
        field(vid("unit"), estruct([])),
        fspacer(),
        field(
            vid("single"),
            estruct([field(vid("value"), etid("String"))]),
        ),
        fspacer(),
        field(vid("projection"), eproject(evid("struct"), vid("field"))),
        field(tid("TypeProj"), eproject(evid("struct"), tid("Type"))),
        fspacer(),
        field(
            vid("main"),
            eblock([
                sbind(vid("msg"), estring("Hello, world!")),
                sexpr(ecall(evid("print"), [evid("msg")])),
            ]),
        ),
    ]);

    println!("{}", ast.pretty_print_string());

    // let ir = ast.into_ir(None).unwrap();

    // dbg!(&ir);
}
