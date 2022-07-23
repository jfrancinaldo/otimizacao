#![feature(iter_intersperse, core_intrinsics)]

mod entry;
mod solution;
mod stopwatch;

use mysql::{from_row, prelude::Queryable, Opts, OptsBuilder, Pool};

use crate::{
    entry::Entry,
    solution::{Attempt, Solution},
};

fn main() {
    let solution = Solution::from_env();

    let numbers_received = solution.numbers_picked.iter().filter(|&&was_picked| was_picked).count();
    if numbers_received < 18 {
        // Don't need to check because the quantity is too low.
        return;
    }

    let opts: Opts = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .user(Some("root"))
        .pass(Some("abcabcabcAA12!@"))
        .db_name(Some("sorteio"))
        .ssl_opts(None)
        .into();

    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let query = conn.query_iter("SELECT idtitulo,dezenas FROM apostas").unwrap();

    let mut ganhadores = vec![];
    let mut armados = 0;

    for entry in query {
        let row = entry.unwrap();
        let entry: Entry = from_row(row);
        let attempt = Attempt::from_array(entry.attempt);
        if solution.is_a_winner(&attempt) {
            ganhadores.push(entry);
        } else if solution.is_armado(&attempt) {
            armados += 1;
        }
    }

    let ids = ganhadores
        .iter()
        .map(|entry| &entry.id)
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    println!(
        "{}|{}|{}",
        armados,
        ganhadores.len(),
        ids.iter().map(AsRef::as_ref).intersperse(",").collect::<String>(),
    );
}
