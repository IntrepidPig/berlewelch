use std::{cmp::max, ops::{Add, Mul}, fmt::Display};

use crate::field::Gfe;

// TODO: Eq
#[derive(Debug, Clone)]
pub struct Polynomial {
	pub coeffs: Vec<Gfe>,
}

impl Polynomial {
	pub fn constant(c: Gfe) -> Self {
		Self { coeffs: vec![c] }
	}

	pub fn single(c: Gfe, d: usize) -> Self {
		let mut coeffs = vec![Gfe::zero(); d];
		coeffs.push(c);
		Self::new(coeffs)
	}
	
	pub fn eval(&self, x: Gfe) -> Gfe {
		let mut y = Gfe::zero();
		for i in 0..self.coeffs.len() {
			y = y + self.coeffs[i] * x.power(i as i32);
		}
		y
	}

	pub fn new(coeffs: Vec<Gfe>) -> Self {
		let mut coeffs = coeffs;
		while coeffs.len() > 1 && coeffs.last() == Some(&Gfe::zero()) {
			coeffs.pop();
		}
		if coeffs.len() == 0 {
			coeffs.push(Gfe::zero());
		}
		Self { coeffs }
	}

	/// Given roots r_1, r_2, ..., r_k generate the polynomial
	/// (x-r_1)(x-r_2)...(x-r_k). Roots may be duplicated and will generate a
	/// polynomial with higher multiplicity roots.
	pub fn from_roots(p: &[Gfe]) -> Self {
		assert!(!p.is_empty());

		let base = Polynomial::new(vec![p[0].negation(), Gfe::one()]);
		if p.len() == 1 {
			return base;
		}

		let rec = Polynomial::from_roots(&p[1..]);
		&base * &rec
	}

	/// Given `n` points, generate the unique degree at most `n-1` polynomial that passes through these points.
	/// 
	/// All passed x coordinates must be unique
	pub fn from_points(p: &[(Gfe, Gfe)]) -> Self {
		assert_ne!(p.len(), 0);
		// TODO: assert: all x coords different
		if p.len() == 1 {
			return Polynomial::constant(p[0].1)
		}

		let rec = Polynomial::from_points(&p[1..]);
		let a_num = p[0].1 + rec.eval(p[0].0).negation();
		let mut a_den = Gfe::one();
		for j in 1..p.len() {
			a_den = a_den * (p[0].0 + p[j].0.negation());
		}
		let a = a_num * a_den.inverse();

		&rec + &(&Polynomial::constant(a) * &Polynomial::from_roots(&p[1..].iter().map(|p| p.0).collect::<Vec<_>>())) // TODO: finish funny interpolation method
	}

	/// Divide this polynomial by another, assuming they evenly divide.
	/// Also assumes leading coefficient of divisor is one. TODO remove this assumption
	pub fn divide(&self, divisor: &Self) -> (Self, Self) {
		// Algorithm overview: Repeatedly subtract multiples of the divisor from the dividend until nothing remains.
		// First subtract a multiple of the divisor such that the leading term of the dividend is removed.
		// Continue until all terms in the dividend have been removed.

		// The quotient being constructed
		let mut quotient = Polynomial::constant(Gfe::zero());
		// The remaining amount of dividend
		let mut dividend = self.clone();

		// While the degree of the dividend is greater than the degree of the divisor
		// (AKA while we can still remove terms of the dividend by subtracting multiples of the divisor)
		while dividend.degree() >= divisor.degree() {
			// Get the leading coefficient of the dividend
			let dividend_leading_coeff = *dividend.coeffs.last().unwrap();
			// Create a constant multiple of a power of x such that when multiplied by the divisor and subtracted from
			// the dividend, the leading term of the dividend is removed.
			let piece = Polynomial::single(dividend_leading_coeff, dividend.degree() - divisor.degree());
			// Remove the leading term of the dividend by adding piece
			dividend = &dividend + &(&piece * divisor).negation();
			// Add piece to the final quotient
			quotient = &quotient + &piece;
		}

		(quotient, dividend)
	}

	pub fn degree(&self) -> usize {
		self.coeffs.len() - 1
	}

	pub fn negation(&self) -> Self {
		Self { coeffs: self.coeffs.iter().map(|x| x.negation()).collect() }
	}
}

impl Add for &'_ Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coeffs = Vec::new();
		let r = max(self.coeffs.len(), rhs.coeffs.len());
		for i in 0..r {
			coeffs.push(Gfe::zero());
			if i < self.coeffs.len() {
				coeffs[i] = coeffs[i] + self.coeffs[i];
			}
			if i < rhs.coeffs.len() {
				coeffs[i] = coeffs[i] + rhs.coeffs[i];
			}
		}
		
		Polynomial::new(coeffs)
    }
}

impl Mul for &'_ Polynomial {
	type Output = Polynomial;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut coeffs = vec![Gfe::zero(); self.coeffs.len() + rhs.coeffs.len()];
		for i in 0..self.coeffs.len() {
			for j in 0..rhs.coeffs.len() {
				coeffs[i+j] = coeffs[i+j] + self.coeffs[i] * rhs.coeffs[j];
			}
		}
		Polynomial::new(coeffs)
	}
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for i in 0..self.coeffs.len() {
			write!(f, "{}", self.coeffs[i])?;
			if i > 0 {
				write!(f, " x")?;
			}
			if i > 1 {
				write!(f, "^{i}")?;
			}
			if i + 1 < self.coeffs.len() {
				write!(f, " + ")?;
			}
		}

        Ok(())
    }
}

#[test]
fn multiply_test() {
	let a = Polynomial::from_roots(&[Gfe::from(1), Gfe::from(-3)]);
	let b = Polynomial::from_roots(&[Gfe::from(-2), Gfe::from(-1)]);
	let y = &a * &b;
	let c1 = Polynomial::new(vec![Gfe::from(-6), Gfe::from(-5), Gfe::from(5), Gfe::from(5), Gfe::from(1)]);
	let c2 = Polynomial::from_roots(&[Gfe::from(1), Gfe::from(-3), Gfe::from(-2), Gfe::from(-1)]);
	assert_eq!(y.coeffs, c1.coeffs);
	assert_eq!(y.coeffs, c2.coeffs);
}

#[test]
fn divide_test() {
	let c = Polynomial::new(vec![Gfe::from(-6), Gfe::from(-5), Gfe::from(5), Gfe::from(5), Gfe::from(1)]);
	let b = Polynomial::from_roots(&[Gfe::from(-2), Gfe::from(-1)]);
	let y = c.divide(&b);
	let a = Polynomial::from_roots(&[Gfe::from(1), Gfe::from(-3)]);
	assert_eq!(y.0.coeffs, a.coeffs);
	assert_eq!(y.1.coeffs, Polynomial::constant(Gfe::zero()).coeffs);
}