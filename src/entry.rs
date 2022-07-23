use core::intrinsics::likely;

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
        let id: u64 = row.take(0).expect("failed to obtain `idtabela`'s value");
        let digits: String = row.take(1).expect("failed to obtain `digitos`'s value");

        let mut attempt: PushArray<u8, 20> = PushArray::new();

        for digit in digits.split(',').map(parse_u8_panicking) {
            attempt
                .push_checked(digit)
                .expect("more than 20 digits found in `digitos`")
        }

        Ok(Self {
            id,
            attempt: attempt.into_array().expect("less than 20 digits found in `digitos`"),
        })
    }
}

fn parse_u8_panicking(num: &str) -> u8 {
    if likely(cfg!(not(debug_assertions)) && num.len() == 2) {
        let num = num.as_bytes();
        (num[0] - b'0') * 10 + (num[1] - b'0')
    } else {
        num.parse().expect("decima number is in invalid format")
    }
}
