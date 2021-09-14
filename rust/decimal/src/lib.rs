use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

use num::Zero;
use num_bigint::BigInt;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, Eq, Ord, Clone)]
pub struct Decimal {
    digits: BigInt,
    n: i32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.contains('.') {
            let mut it = input.splitn(2, '.');
            let a = it.next()?.as_bytes();
            let b = it.next()?.trim_end_matches('0').as_bytes();
            let mut digits = Vec::with_capacity(a.len() + b.len());
            digits.extend(a);
            digits.extend(b);

            Some(Decimal {
                digits: BigInt::parse_bytes(&digits[..], 10)?,
                n: -(b.len() as i32),
            })
        } else {
            Some(Decimal {
                digits: BigInt::parse_bytes(input.as_bytes(), 10)?,
                n: 0,
            })
        }
    }

    fn normalize(&mut self) {
        if self.digits.is_zero() {
            self.n = 0;
        } else {
            while (&self.digits % 10u8).is_zero() {
                self.digits /= 10;
                self.n += 1;
            }
        }
    }

    fn equalize_power(a: &mut Decimal, b: &mut Decimal) {
        match a.n.cmp(&b.n) {
            Ordering::Less => {
                b.digits *= BigInt::from(10).pow((b.n - a.n) as u32);
                b.n = a.n;
            }
            Ordering::Greater => {
                a.digits *= BigInt::from(10).pow((a.n - b.n) as u32);
                a.n = b.n;
            }
            _ => {}
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut a = self.clone();
        let mut b = other.clone();
        Decimal::equalize_power(&mut a, &mut b);
        Some(a.digits.cmp(&b.digits))
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        Decimal::equalize_power(&mut self, &mut rhs);
        let mut result = Decimal {
            digits: self.digits + rhs.digits,
            n: self.n,
        };
        result.normalize();
        result
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        Decimal::equalize_power(&mut self, &mut rhs);
        let mut result = Decimal {
            digits: self.digits - rhs.digits,
            n: self.n,
        };
        result.normalize();
        result
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Decimal {
            digits: self.digits * rhs.digits,
            n: self.n + rhs.n,
        };
        result.normalize();
        result
    }
}
