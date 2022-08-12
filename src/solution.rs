use std::ffi::OsStr;

#[derive(Debug)]
pub struct Solution {
    pub numbers_picked: [bool; 61],
}

pub struct Attempt([u8; 20]);

impl Attempt {
    pub fn from_array(arr: [u8; 20]) -> Self {
        let is_in_range = |num| (1..=60).contains(num);
        assert!(arr.iter().all(is_in_range));

        Self(arr)
    }
}

impl Solution {
    pub fn new() -> Self {
        Self {
            numbers_picked: [false; 61],
        }
    }

    /// Adds a number to the solution
    pub fn add_number(&mut self, num: u8) {
        self.numbers_picked[num as usize] = true;
    }

    /// Checks if the given attempt is a winner
    pub fn is_a_winner(&self, attempt: &Attempt) -> bool {
        let number_was_picked = |num| self.number_was_picked(num);

        attempt.0.into_iter().all(number_was_picked)
    }

    /// Checks if the given attempt is armado
    pub fn is_armado(&self, attempt: &Attempt) -> bool {
        attempt.0.into_iter().filter(|num| self.number_was_picked(*num)).count() == 19
    }

    #[inline(always)]
    /// Returns true if the given number was picked in the solution
    pub fn number_was_picked(&self, num: u8) -> bool {
        self.numbers_picked[num as usize]
    }

    pub fn from_env() -> Self {
        let to_str_panicking =
            |arg: &'static OsStr| arg.to_str().expect("Problema argumento invalido encoding não UTF-8");

        let parse_u8_panicking = |arg: &str| {
            arg.parse().unwrap_or_else(|err| {
                panic!("Failed to parse integer '{arg}': {err}");
            })
        };

        argv::iter()
            .nth(1)
            .map(to_str_panicking)
            .expect("Missing arguments, numbers to be checked")
            .split(',')
            .map(parse_u8_panicking)
            .collect()
    }
}

impl FromIterator<u8> for Solution {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut this = Self::new();

        iter.into_iter().for_each(|num| this.add_number(num));

        this
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use crate::{Attempt, Solution};

    #[test]
    fn from_iter() {
        let valid_solution = Attempt::from_array([
            2, 4, 6, 10, 16, 18, 22, 26, 28, 29, 30, 32, 36, 38, 39, 40, 41, 45, 47, 48,
        ]);

        let invalid_solution_1 = Attempt::from_array([
            2, 4, 6, 10, 16, 18, 22, 26, 28, 29, 30, 32, 36, 38, 39, 40, 41, 45, 47, 49,
        ]);

        let invalid_solution_2 = Attempt::from_array([
            2, 4, 6, 10, 16, 18, 19, 22, 26, 28, 29, 30, 32, 36, 38, 39, 40, 41, 45, 47,
        ]);

        let invalid_solution_3 = Attempt::from_array([
            1, 2, 4, 6, 10, 16, 18, 19, 22, 26, 28, 29, 30, 32, 36, 38, 39, 40, 41, 45,
        ]);

        let solution = Solution::from_iter(valid_solution.0.iter().copied());

        assert!(solution.is_a_winner(&invalid_solution_1).not());
        assert!(solution.is_a_winner(&invalid_solution_2).not());
        assert!(solution.is_a_winner(&invalid_solution_3).not());

        assert!(solution.is_a_winner(&valid_solution));
    }
}
