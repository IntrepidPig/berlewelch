use std::{ops::{Add, Mul, Deref}, fmt::{Display, Debug}};


pub const P: Gfe = Gfe(0x7fffffff); // GF(2^31-1)
//pub const P: Gfe = Gfe(19);
//pub const P: Gfe = Gfe(29);

/// An element of the Galois field GF(p) where p is the constant declared in this module.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gfe(u32);

impl Gfe {
	pub fn new(x: u32) -> Self {
		Self::from(x as i64)
	}

	pub fn zero() -> Self {
		Self(0)
	}

	pub fn one() -> Self {
		Self(1)
	}

	pub fn inverse(self) -> Self {
		assert!(self.0 != 0);
		let (_d, a, _b) = gcde(self.0 as i64, P.0 as i64);
		return Self::from(a);
	}

	pub fn negation(self) -> Self {
		Self((P.0 - self.0) % P.0)
	}

	pub fn power(self, e: i32) -> Self {
		if e < 0 {
			return self.power(-e).inverse();
		}
		if e == 0 {
			return Gfe(1);
		}
		if e == 1 {
			return self;
		}

		let mut r = self.power(e/2);
		r = r * r;
		if e % 2 == 1 { r = r * self };
		r
	}
}

impl Deref for Gfe {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for Gfe {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u64 + rhs.0 as u64) % P.0 as u64) as u32)
    }
}

impl Mul for Gfe {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(((self.0 as u64 * rhs.0 as u64) % P.0 as u64) as u32)
	}
}

impl From<i64> for Gfe {
	fn from(x: i64) -> Self {
		if x < 0 {
			let y = -x;
			let k = (y + P.0 as i64 - 1) / P.0 as i64; // ceil(y / P);
			Self((x + k * P.0 as i64) as u32)
		} else {
			Self((x as u64 % P.0 as u64) as u32)
		}
	}
}

impl Display for Gfe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Gfe {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

pub fn gcde(x: i64, y: i64) -> (i64, i64, i64) {
	if y == 0 {
		return (x, 1, 0);
	}

	let r = x % y;
	let (d, ap, bp) = gcde(y, r);
	(d, bp, ap - bp * (x/y))
}