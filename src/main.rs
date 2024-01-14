use std::fs::File;
use std::io::prelude::*;
use uplc::{
    ast::{Constant, DeBruijn, Name, Program, Term},
    builtins::DefaultFunction,
    optimize::aiken_optimize_and_intern,
};

const UN_CONSTR_DATA: Term<Name> = Term::Builtin(DefaultFunction::UnConstrData);
const SND_PAIR: Term<Name> = Term::Builtin(DefaultFunction::SndPair);
const HEAD_LIST: Term<Name> = Term::Builtin(DefaultFunction::HeadList);
const TAIL_LIST: Term<Name> = Term::Builtin(DefaultFunction::TailList);
const UN_MAP_DATA: Term<Name> = Term::Builtin(DefaultFunction::UnMapData);

fn list_has() -> Term<Name> {
    (Term::Builtin(DefaultFunction::IfThenElse)
        .force()
        .apply(
            Term::equals_data()
                .apply(
                    Term::Builtin(DefaultFunction::FstPair)
                        .force()
                        .force()
                        .apply(
                            Term::Builtin(DefaultFunction::HeadList)
                                .force()
                                .apply(Term::var("list_list")),
                        ),
                )
                .apply(Term::var("list_search")),
        )
        .apply(Term::Constant(Constant::Unit.into()).delay())
        .apply(
            Term::var("self_list_has")
                .apply(Term::var("self_list_has"))
                .apply(Term::var("list_search"))
                .apply(
                    Term::Builtin(DefaultFunction::TailList)
                        .force()
                        .apply(Term::var("list_list")),
                )
                .delay(),
        ))
    .force()
    .lambda("list_list")
    .lambda("list_search")
    .lambda("self_list_has")
}

fn transaction_from_ctx() -> Term<Name> {
    let ctx_var = Term::var("ctx");
    HEAD_LIST.force().apply(
        SND_PAIR
            .force()
            .force()
            .apply(UN_CONSTR_DATA.apply(ctx_var)),
    )
}

fn withdrawal_from_ctx() -> Term<Name> {
    UN_MAP_DATA.apply(
        HEAD_LIST.force().apply(
            TAIL_LIST.force().apply(
                TAIL_LIST.force().apply(
                    TAIL_LIST.force().apply(
                        TAIL_LIST.force().apply(
                            TAIL_LIST.force().apply(
                                TAIL_LIST.force().apply(
                                    SND_PAIR
                                        .force()
                                        .force()
                                        .apply(UN_CONSTR_DATA.apply(transaction_from_ctx())),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}

fn main() {
    let named_program: Program<Name> = aiken_optimize_and_intern(Program {
        version: (1, 0, 0),
        term: Term::var("list_has")
            .apply(Term::var("list_has"))
            .apply(Term::var("script_withdrawal_credential"))
            .apply(withdrawal_from_ctx())
            .lambda("list_has")
            .apply(list_has())
            .lambda("ctx")
            .lambda("redeemer")
            .lambda("datum")
            .lambda("script_withdrawal_credential"),
    });
    let debruijn_program = <Program<DeBruijn>>::try_from(named_program.clone()).unwrap();

    let mut file = File::create("spend_named.uplc").expect("Unable to create file");
    file.write_all(named_program.to_pretty().as_bytes())
        .expect("Unable to write data");

    let mut file = File::create("spend_debruijn.uplc").expect("Unable to create file");
    file.write_all(debruijn_program.to_pretty().as_bytes())
        .expect("Unable to write data");

    let hex = debruijn_program.to_hex().unwrap();
    let mut file = File::create("spend.txt").expect("Unable to create file");
    file.write_all(hex.as_bytes())
        .expect("Unable to write data");
}
