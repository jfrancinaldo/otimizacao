use core::intrinsics::{likely, unlikely};
use std::panic::catch_unwind;

use mysql::prelude::FromRow;
use pushy::PushArray;

#[derive(Debug)]
pub struct Entry {
    pub id: u64,
    pub attempt: [u8; 20],
}

impl FromRow for Entry {
    fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let id: u64 = row
            .take(0)
            .expect("falhou em ler a coluna `idtabela` do banco de dados");
        let digits: String = row.take(1).expect("falhou em ler a coluna `digitos` do banco de dados");

        let mut attempt: PushArray<u8, 20> = PushArray::new();

        for digit in digits.split(',').map(|digite| {
            catch_unwind(|| parse_u8_panicking(digite)).unwrap_or_else(|_| panic!("falha em ler dezenas do ID {id}"))
        }) {
            attempt.push_checked(digit).expect(
                "problema: uma pessoa no banco de dados não deveria ter mais de 20 dezenas, porque 20 é o limite",
            )
        }

        Ok(Self {
            id,
            attempt: attempt.into_array().expect("problema: uma pessoa no banco de dados não deveria ter menos de 20 dezenas, porque 20 é o valor esperado"),
        })
    }
}

fn parse_u8_panicking(num: &str) -> u8 {
    let num_is_numeric = || num.chars().all(char::is_numeric);

    if unlikely(num_is_numeric()) {
        panic!("Problema: dezena '{num}' não é uma dezena válida");
    }

    if likely(cfg!(not(debug_assertions)) && num.len() == 2) {
        let num = num.as_bytes();
        (num[0] - b'0') * 10 + (num[1] - b'0')
    } else {
        num.parse().expect("decima number is in invalid format")
    }
}
