//! Day 25: Full of Hot Air
//! https://adventofcode.com/2022/day/25

use std::{fmt::Display, fs, iter::repeat, ops::Add, str::FromStr, u8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnafuDigitParseError {
    InvalidChar,
    EmptyInput,
    NegativeValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnafuDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnafuCarry {
    One,
    Minus,
    Zero,
}

#[derive(Debug, PartialEq, Eq)]
struct SnafuNumber(Vec<SnafuDigit>);

impl Default for SnafuDigit {
    #[inline]
    fn default() -> Self {
        Self::Zero
    }
}

impl TryFrom<u8> for SnafuDigit {
    type Error = SnafuDigitParseError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'2' => Ok(Self::Two),
            b'1' => Ok(Self::One),
            b'0' => Ok(Self::Zero),
            b'-' => Ok(Self::Minus),
            b'=' => Ok(Self::DoubleMinus),
            _ => Err(SnafuDigitParseError::InvalidChar),
        }
    }
}

impl From<SnafuDigit> for char {
    #[inline]
    fn from(value: SnafuDigit) -> Self {
        match value {
            SnafuDigit::Two => '2',
            SnafuDigit::One => '1',
            SnafuDigit::Zero => '0',
            SnafuDigit::Minus => '-',
            SnafuDigit::DoubleMinus => '=',
        }
    }
}

impl Add for SnafuCarry {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::One, Self::One) | (Self::Minus, Self::Minus) => {
                panic!("Overflow! Can only carry `One` or `Minus` at most.")
            }
            (Self::Zero, Self::Zero) | (Self::One, Self::Minus) | (Self::Minus, Self::One) => {
                Self::Zero
            }
            (Self::Zero, Self::One) | (Self::One, Self::Zero) => Self::One,
            (Self::Zero, Self::Minus) | (Self::Minus, Self::Zero) => Self::Minus,
        }
    }
}

impl Add<Self> for SnafuDigit {
    type Output = (SnafuCarry, SnafuDigit);

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, result) | (result, Self::Zero) => (SnafuCarry::Zero, result),
            (Self::Two, Self::Two) => (SnafuCarry::One, Self::Minus),
            (Self::Two, Self::One) | (Self::One, Self::Two) => (SnafuCarry::One, Self::DoubleMinus),
            (Self::One, Self::One) => (SnafuCarry::Zero, Self::Two),
            (Self::Two, Self::Minus) | (Self::Minus, Self::Two) => (SnafuCarry::Zero, Self::One),
            (Self::Two, Self::DoubleMinus)
            | (Self::DoubleMinus, Self::Two)
            | (Self::One, Self::Minus)
            | (Self::Minus, Self::One) => (SnafuCarry::Zero, Self::Zero),
            (Self::One, Self::DoubleMinus) | (Self::DoubleMinus, Self::One) => {
                (SnafuCarry::Zero, Self::Minus)
            }
            (Self::Minus, Self::Minus) => (SnafuCarry::Zero, Self::DoubleMinus),
            (Self::Minus, Self::DoubleMinus) | (Self::DoubleMinus, Self::Minus) => {
                (SnafuCarry::Minus, Self::Two)
            }
            (Self::DoubleMinus, Self::DoubleMinus) => (SnafuCarry::Minus, Self::One),
        }
    }
}

impl Add<SnafuCarry> for SnafuDigit {
    type Output = (SnafuCarry, SnafuDigit);

    #[inline]
    fn add(self, rhs: SnafuCarry) -> Self::Output {
        match (self, rhs) {
            (result, SnafuCarry::Zero) => (SnafuCarry::Zero, result),
            (Self::Two, SnafuCarry::One) => (SnafuCarry::One, Self::DoubleMinus),
            (Self::One, SnafuCarry::One) => (SnafuCarry::Zero, Self::Two),
            (Self::Two, SnafuCarry::Minus) | (Self::Zero, SnafuCarry::One) => {
                (SnafuCarry::Zero, Self::One)
            }
            (Self::One, SnafuCarry::Minus) | (Self::Minus, SnafuCarry::One) => {
                (SnafuCarry::Zero, Self::Zero)
            }
            (Self::Zero, SnafuCarry::Minus) | (Self::DoubleMinus, SnafuCarry::One) => {
                (SnafuCarry::Zero, Self::Minus)
            }
            (Self::Minus, SnafuCarry::Minus) => (SnafuCarry::Zero, Self::DoubleMinus),
            (Self::DoubleMinus, SnafuCarry::Minus) => (SnafuCarry::Minus, Self::Two),
        }
    }
}

impl SnafuDigit {
    #[inline]
    fn add_with_carry(self, rhs: Self, carry: SnafuCarry) -> (SnafuCarry, Self) {
        let (current_carry_0, added_digit) = self + rhs;
        let (current_carry_1, result_digit) = added_digit + carry;
        (current_carry_0 + current_carry_1, result_digit)
    }

    #[inline]
    fn is_valid_first(self) -> bool {
        match self {
            SnafuDigit::Two | SnafuDigit::One => true,
            SnafuDigit::Zero | SnafuDigit::Minus | SnafuDigit::DoubleMinus => false,
        }
    }
}

impl Default for SnafuNumber {
    #[inline]
    fn default() -> Self {
        Self(vec![SnafuDigit::default(); 1])
    }
}

impl TryFrom<Vec<SnafuDigit>> for SnafuNumber {
    type Error = SnafuDigitParseError;

    #[inline]
    fn try_from(value: Vec<SnafuDigit>) -> Result<Self, Self::Error> {
        if let Some(&first) = value.first() {
            if !first.is_valid_first() {
                return Err(SnafuDigitParseError::NegativeValue);
            }
        } else {
            return Err(SnafuDigitParseError::EmptyInput);
        }

        Ok(Self(value))
    }
}

impl FromStr for SnafuNumber {
    type Err = SnafuDigitParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.bytes()
            .map(SnafuDigit::try_from)
            .collect::<Result<Vec<SnafuDigit>, SnafuDigitParseError>>()?
            .try_into()
    }
}

impl From<&SnafuNumber> for String {
    fn from(value: &SnafuNumber) -> Self {
        value
            .0
            .iter()
            .map(|&digit| char::from(digit))
            .collect::<String>()
    }
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

// impl SnafuNumber {
//     fn add_in_place(&mut self, other: &SnafuNumber) {
//         let self_iter = self.0.iter_mut().rev();
//         let other_iter = other.0.iter().rev();
//
//         let mut carry = SnafuCarry::Zero;
//         zip(self_iter, other_iter).for_each(|(lhs, rhs)| {
//             (carry, *lhs) = lhs.add_with_carry(*rhs, carry);
//         });
//
//         let self_len = self.0.len();
//         let other_len = other.0.len();
//         if self_len > other_len {
//             self.0[..(self_len - other_len)]
//                 .iter_mut()
//                 .rev()
//                 .for_each(|digit| {
//                     (carry, *digit) = *digit + carry;
//                 });
//         } else if other_len > self_len {
//             other.0[..(other_len - self_len)]
//                 .iter()
//                 .rev()
//                 .for_each(|&digit| {
//                     let new_digit;
//                     (carry, new_digit) = digit + carry;
//                     self.0.insert(0, new_digit);
//                 });
//         }
//
//         match carry {
//             SnafuCarry::Minus => panic!("Overflow! Negative SNAFU number is not supported."),
//             SnafuCarry::One => self.0.insert(0, SnafuDigit::One),
//             SnafuCarry::Zero => (),
//         }
//     }
// }

impl Add for SnafuNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_len = self.0.len();
        let other_len = rhs.0.len();

        let self_iter = self.0.into_iter().rev();
        let other_iter = rhs.0.into_iter().rev();

        let mut carry = SnafuCarry::Zero;

        let mut result = if self_len < other_len {
            self_iter.chain(repeat(SnafuDigit::Zero)).zip(other_iter)
        } else {
            other_iter.chain(repeat(SnafuDigit::Zero)).zip(self_iter)
        }
        .map(|(lhs, rhs)| {
            let new_digit;
            (carry, new_digit) = lhs.add_with_carry(rhs, carry);
            new_digit
        })
        .collect::<Vec<_>>();

        match carry {
            SnafuCarry::One => result.push(SnafuDigit::One),
            _ => (),
        }

        Self(result.into_iter().rev().collect())
    }
}

fn parse_snafu_digit(value: u8) -> Option<i64> {
    match value {
        b'2' => Some(2),
        b'1' => Some(1),
        b'0' => Some(0),
        b'-' => Some(-1),
        b'=' => Some(-2),
        _ => None,
    }
}

fn snafu_to_i64(input: &str) -> Option<i64> {
    input
        .bytes()
        .rev()
        .enumerate()
        .try_fold(0, |acc, (index, digit)| {
            Some(acc + 5_i64.pow(index as u32) * parse_snafu_digit(digit)?)
        })
}

const INPUT_FILENAME: &str = "input/full_of_hot_air.txt";

pub fn main() -> Result<(), String> {
    let input = fs::read_to_string(INPUT_FILENAME).map_err(|e| e.to_string())?;
    let (result, result_i64) = input
        .lines()
        .try_fold((SnafuNumber::default(), 0i64), |(acc, acc_i64), line| {
            let num: SnafuNumber = line.parse().ok()?;
            let num_i64 = snafu_to_i64(line)?;
            print!("{} | {} | {}; ", line, &num, num_i64);
            let sum = num + acc;
            let parsed_sum_i64 = snafu_to_i64(String::from(&sum).as_str())?;
            let sum_i64 = num_i64 + acc_i64;
            println!("sum: {} | {} | {}", sum, parsed_sum_i64, sum_i64);
            assert_eq!(parsed_sum_i64, sum_i64);
            Some((sum, sum_i64))
        })
        .ok_or("Invalid SNAFU number.")?;

    println!("{}, {}", result, result_i64);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_number_add() {
        assert_eq!(
            SnafuNumber::default() + SnafuNumber::from_str("221-=-").unwrap(),
            SnafuNumber::from_str("221-=-").unwrap()
        );
        assert_eq!(
            SnafuNumber::from_str("21211").unwrap() + SnafuNumber::from_str("221-=-").unwrap(),
            SnafuNumber::from_str("1=-21-0").unwrap()
        );
    }
}
