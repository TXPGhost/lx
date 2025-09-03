pub mod ast;
pub mod colorscheme;
// pub mod eval;
pub mod ir;
pub mod node;
// pub mod subtype;

use ast::helpers::*;
// use ir::IntoIr;

use crate::ast::pretty_print::PrettyPrint;

fn main() {
    let ast = estruct::<()>([
        field(
            tid("Vector3"),
            estruct([
                field(vid("x"), etid("I32")),
                field(vid("y"), etid("I32")),
                field(vid("z"), etid("I32")),
                fspacer(),
                field(
                    vid("zero"),
                    ecall(etid("Vector3"), [ei32(0), ei32(0), ei32(0)]),
                ),
                fspacer(),
                field(
                    vid("unit_x"),
                    ecall(etid("Vector3"), [ei32(1), ei32(0), ei32(0)]),
                ),
                field(
                    vid("unit_y"),
                    ecall(etid("Vector3"), [ei32(0), ei32(1), ei32(0)]),
                ),
                field(
                    vid("unit_z"),
                    ecall(etid("Vector3"), [ei32(0), ei32(0), ei32(1)]),
                ),
                fspacer(),
                field(
                    vid("len_sq"),
                    efunc(
                        args([arg(vid("self"), etid("Vector3"))]),
                        add(
                            add(
                                pow(eproj(evid("self"), vid("x")), ei32(2)),
                                pow(eproj(evid("self"), vid("y")), ei32(2)),
                            ),
                            pow(eproj(evid("self"), vid("z")), ei32(2)),
                        ),
                    ),
                ),
                field(
                    vid("len"),
                    efunc(
                        args([arg(vid("self"), etid("Vector3"))]),
                        emethod(emethod(evid("self"), evid("len_sq"), []), evid("sqrt"), []),
                    ),
                ),
                fspacer(),
                field(
                    vid("add"),
                    efunc(
                        args([
                            arg(vid("lhs"), etid("Vector3")),
                            arg(vid("rhs"), etid("Vector3")),
                        ]),
                        ecall(
                            etid("Vector3"),
                            [
                                add(eproj(evid("lhs"), vid("x")), eproj(evid("rhs"), vid("x"))),
                                add(eproj(evid("lhs"), vid("y")), eproj(evid("rhs"), vid("y"))),
                                add(eproj(evid("lhs"), vid("z")), eproj(evid("rhs"), vid("z"))),
                            ],
                        ),
                    ),
                ),
                fspacer(),
                field(
                    vid("to_string"),
                    efunc(
                        args([arg(vid("self"), etid("Vector3"))]),
                        concat(
                            concat(
                                concat(
                                    estring("("),
                                    emethod(eproj(evid("self"), vid("x")), evid("to_string"), []),
                                ),
                                concat(
                                    estring(", "),
                                    emethod(eproj(evid("self"), vid("y")), evid("to_string"), []),
                                ),
                            ),
                            concat(
                                concat(
                                    estring(", "),
                                    emethod(eproj(evid("self"), vid("z")), evid("to_string"), []),
                                ),
                                estring(")"),
                            ),
                        ),
                    ),
                ),
                fspacer(),
                field(
                    vid("normalize"),
                    efunc(
                        args([arg_mut(vid("self"), etid("Vector3"))]),
                        eblock([
                            sbind(vid("len"), emethod(evid("self"), evid("len"), [])),
                            sdiv(eproj(evid("self"), vid("x")), evid("len")),
                            sdiv(eproj(evid("self"), vid("y")), evid("len")),
                            sdiv(eproj(evid("self"), vid("z")), evid("len")),
                        ]),
                    ),
                ),
            ]),
        ),
        fspacer(),
        field(
            vid("main"),
            efunc(
                args([arg_mut(vid("io"), etid("IO"))]),
                eblock([
                    sbind(
                        vid("vec_a"),
                        ecall(etid("Vector3"), [ei32(1), ei32(2), ei32(3)]),
                    ),
                    sbind(
                        vid("vec_b"),
                        ecall(etid("Vector3"), [ei32(4), ei32(5), ei32(6)]),
                    ),
                    sbind(
                        vid("vec_c"),
                        ecall(
                            eproj(etid("Vector3"), vid("add")),
                            [evid("vec_a"), evid("vec_b")],
                        ),
                    ),
                    sspacer(),
                    sexpr(emethod(
                        evid("io"),
                        evid("println"),
                        [emethod(evid("vec_c"), evid("to_string"), [])],
                    )),
                    sspacer(),
                    sbind(
                        vid("x"),
                        ecall(eproj(etid("Cow"), tid("Owned")), [estring("Hello, world!")]),
                    ),
                    sspacer(),
                    sbind(vid("y"), copy(evid("x"))),
                    sbind(vid("z"), copy(evid("x"))),
                    sspacer(),
                    sconcat(evid("z"), estring("test")),
                ]),
            ),
        ),
    ]);

    println!("{}", ast.elt.borrow().pretty_print_string());

    // let ir = ast.into_ir(None).unwrap();
    // let ans = ir.eval(&mut HashMap::new()).unwrap();

    // dbg!(ans);
}
