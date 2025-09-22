pub mod ast;
pub mod colorscheme;
// pub mod eval;
pub mod ir;
pub mod node;
// pub mod subtype;
pub mod lexer;

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
                    ecall(
                        etid("Vector3"),
                        args([arg(ei32(0)), arg(ei32(0)), arg(ei32(0))]),
                    ),
                ),
                fspacer(),
                field(
                    vid("unit_x"),
                    ecall(
                        etid("Vector3"),
                        args([arg(ei32(1)), arg(ei32(0)), arg(ei32(0))]),
                    ),
                ),
                field(
                    vid("unit_y"),
                    ecall(
                        etid("Vector3"),
                        args([arg(ei32(0)), arg(ei32(1)), arg(ei32(0))]),
                    ),
                ),
                field(
                    vid("unit_z"),
                    ecall(
                        etid("Vector3"),
                        args([arg(ei32(0)), arg(ei32(0)), arg(ei32(1))]),
                    ),
                ),
                fspacer(),
                field(
                    vid("len_sq"),
                    efunc(
                        params([param(vid("self"), etid("Vector3"))]),
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
                        params([param(vid("self"), etid("Vector3"))]),
                        emethod(
                            evid("sqrt"),
                            args([arg(emethod(evid("len_sq"), args([arg(evid("self"))])))]),
                        ),
                    ),
                ),
                fspacer(),
                field(
                    vid("add"),
                    efunc(
                        params([
                            param(vid("lhs"), etid("Vector3")),
                            param(vid("rhs"), etid("Vector3")),
                        ]),
                        ecall(
                            etid("Vector3"),
                            args([
                                arg(add(
                                    eproj(evid("lhs"), vid("x")),
                                    eproj(evid("rhs"), vid("x")),
                                )),
                                arg(add(
                                    eproj(evid("lhs"), vid("y")),
                                    eproj(evid("rhs"), vid("y")),
                                )),
                                arg(add(
                                    eproj(evid("lhs"), vid("z")),
                                    eproj(evid("rhs"), vid("z")),
                                )),
                            ]),
                        ),
                    ),
                ),
                fspacer(),
                field(
                    vid("to_string"),
                    efunc(
                        params([param(vid("self"), etid("Vector3"))]),
                        concat(
                            concat(
                                concat(
                                    estring("("),
                                    emethod(
                                        evid("to_string"),
                                        args([arg(eproj(evid("self"), vid("x")))]),
                                    ),
                                ),
                                concat(
                                    estring(", "),
                                    emethod(
                                        evid("to_string"),
                                        args([arg(eproj(evid("self"), vid("y")))]),
                                    ),
                                ),
                            ),
                            concat(
                                concat(
                                    estring(", "),
                                    emethod(
                                        evid("to_string"),
                                        args([arg(eproj(evid("self"), vid("z")))]),
                                    ),
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
                        params([param_mut(vid("self"), etid("Vector3"))]),
                        eblock([
                            sbind(
                                vid("len"),
                                emethod(evid("len"), args([arg(eproj(evid("self"), vid("len")))])),
                            ),
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
                params([param_mut(vid("io"), etid("IO"))]),
                eblock([
                    sbind(
                        vid("vec_a"),
                        ecall(
                            etid("Vector3"),
                            args([arg(ei32(1)), arg(ei32(2)), arg(ei32(3))]),
                        ),
                    ),
                    sbind(
                        vid("vec_b"),
                        ecall(
                            etid("Vector3"),
                            args([arg(ei32(4)), arg(ei32(5)), arg(ei32(6))]),
                        ),
                    ),
                    sbind(
                        vid("vec_c"),
                        ecall(
                            eproj(etid("Vector3"), vid("add")),
                            args([arg(evid("vec_a")), arg(evid("vec_b"))]),
                        ),
                    ),
                    sspacer(),
                    sexpr(emethod(
                        evid("println"),
                        args([
                            arg(evid("io")),
                            arg(emethod(evid("to_string"), args([arg(evid("vec_c"))]))),
                        ]),
                    )),
                    sspacer(),
                    sexpr(ecall(
                        eproj(etid("Vector3"), vid("normalize")),
                        args([arg_mut(evid("vec_a"))]),
                    )),
                    sexpr(ecall(
                        eproj(evid("vec_b"), vid("normalize")),
                        args([arg_mut(evid("vec_b"))]),
                    )),
                    sexpr(emethod(evid("normalize"), args([arg_mut(evid("vec_c"))]))),
                    sspacer(),
                    sbind(
                        vid("x"),
                        ecall(
                            eproj(etid("Cow"), tid("Owned")),
                            args([arg(estring("Hello, world!"))]),
                        ),
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

    println!("{}", ast.get().pretty_print_string());

    // let ir = ast.into_ir(None).unwrap();
    // let ans = ir.eval(&mut HashMap::new()).unwrap();

    // dbg!(ans);
}
