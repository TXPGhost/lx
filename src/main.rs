pub mod ast;
pub mod colorscheme;
pub mod eval;
pub mod ir;

use ast::helpers::*;
// use ir::IntoIr;

use crate::ast::pretty_print::PrettyPrint;

fn main() {
    let ast = estruct([
        field(
            tid("Vector3"),
            estruct([
                field(vid("x"), etid("I32")),
                field(vid("y"), etid("I32")),
                field(vid("z"), etid("I32")),
                fspacer(),
                field(
                    vid("zero"),
                    econstructor(
                        tid("Vector3"),
                        [
                            field(vid("x"), ei32(0)),
                            field(vid("y"), ei32(0)),
                            field(vid("z"), ei32(0)),
                        ],
                    ),
                ),
                fspacer(),
                field(
                    vid("unit_x"),
                    ecall(
                        eproj(etid("Vector3"), vid("new")),
                        [ei32(1), ei32(0), ei32(0)],
                    ),
                ),
                field(
                    vid("unit_y"),
                    ecall(
                        eproj(etid("Vector3"), vid("new")),
                        [ei32(0), ei32(1), ei32(0)],
                    ),
                ),
                field(
                    vid("unit_z"),
                    ecall(
                        eproj(etid("Vector3"), vid("new")),
                        [ei32(0), ei32(0), ei32(1)],
                    ),
                ),
                fspacer(),
                field(
                    vid("new"),
                    efunc(
                        args([
                            arg(vid("x"), etid("I32")),
                            arg(vid("y"), etid("I32")),
                            arg(vid("z"), etid("I32")),
                        ]),
                        econstructor(
                            tid("Vector3"),
                            [
                                field(vid("x"), eident(hoist(vid("x"), 1))),
                                field(vid("y"), eident(hoist(vid("y"), 1))),
                                field(vid("z"), eident(hoist(vid("z"), 1))),
                            ],
                        ),
                    ),
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
                            eproj(etid("Vector3"), vid("new")),
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
            tid("main"),
            efunc(
                args([arg_mut(vid("io"), etid("IO"))]),
                eblock([
                    sbind(
                        vid("vec_a"),
                        ecall(
                            eproj(etid("Vector3"), vid("new")),
                            [ei32(1), ei32(2), ei32(3)],
                        ),
                    ),
                    sbind(
                        vid("vec_b"),
                        ecall(
                            eproj(etid("Vector3"), vid("new")),
                            [ei32(4), ei32(5), ei32(6)],
                        ),
                    ),
                    sbind(
                        vid("vec_c"),
                        ecall(
                            eproj(etid("Vector3"), vid("add")),
                            [evid("vec_a"), evid("vec_b")],
                        ),
                    ),
                    sexpr(emethod(
                        evid("io"),
                        evid("println"),
                        [emethod(evid("vec_c"), evid("to_string"), [])],
                    )),
                ]),
            ),
        ),
    ]);

    println!("{}", ast.pretty_print_string());

    // let ir = ast.into_ir(None).unwrap();

    // dbg!(&ir);
}
