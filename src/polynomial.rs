use std::{
	cmp::max,
	fmt::Display,
	ops::{Add, Mul},
};

use crate::field::Gfe;

// TODO: Eq
#[derive(Debug, Clone)]
pub struct Polynomial<const M: u32> {
	pub coeffs: Vec<Gfe<M>>,
}

impl<const M: u32> Polynomial<M> {
	pub fn constant(c: Gfe<M>) -> Self {
		Self { coeffs: vec![c] }
	}

	pub fn zero() -> Self {
		Self::constant(Gfe::zero())
	}

	pub fn single(c: Gfe<M>, d: usize) -> Self {
		let mut coeffs = vec![Gfe::zero(); d];
		coeffs.push(c);
		Self::new(coeffs)
	}

	pub fn eval(&self, x: Gfe<M>) -> Gfe<M> {
		let mut y = Gfe::zero();
		for i in 0..self.coeffs.len() {
			y = y + self.coeffs[i] * x.power(i as i32);
		}
		y
	}

	pub fn new(coeffs: Vec<Gfe<M>>) -> Self {
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
	pub fn from_roots(p: &[Gfe<M>]) -> Self {
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
	// TODO: try solving system of linear eqn instead of lagrange interpolation
	pub fn from_points(points: &[(Gfe<M>, Gfe<M>)]) -> Self {
		assert!(!points.is_empty());

		// Base case, there is one point simply return a constant polynomial with that point
		if points.len() == 1 {
			return Polynomial::constant(points[0].1);
		}

		// Recursive case: Given points (x0, y0), ..., (xn, yn). Create a partial polynomial P(x) that passes through
		// all points except point 0. Let Z(x) be the polynomial (x-x1)...(x-xn) that has a leading coefficient of 1 and
		// roots on all points 1 to n. Then the new polynomial that passes through every point is given by P(x) +
		// Z(x)/Z(x0) * (y0 - P(x0))

		// P(x)
		let partial = Polynomial::from_points(&points[1..]);
		// Z(x)
		let helper = Polynomial::from_roots(&points[1..].iter().map(|p| p.0).collect::<Vec<_>>());
		// (y0 - P(x0))/Z(x0)
		let coeff = (points[0].1 + partial.eval(points[0].0).negation()) * helper.eval(points[0].0).inverse();
		// P(x) + (y0 - P(x0))/Z(x0) * Z(x)
		partial + Polynomial::constant(coeff) * helper
	}

	/// Divide this polynomial by another
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
			// Get the leading coefficient of the divisor and dividend
			let divisor_leading_coeff = *divisor.coeffs.last().unwrap();
			let dividend_leading_coeff = *dividend.coeffs.last().unwrap();
			// Create a constant multiple of a power of x such that when multiplied by the divisor and subtracted from
			// the dividend, the leading term of the dividend is removed.
			let piece = Polynomial::single(
				divisor_leading_coeff.inverse() * dividend_leading_coeff,
				dividend.degree() - divisor.degree(),
			);
			// Remove the leading term of the dividend by adding piece
			dividend = &dividend + &(&piece * &divisor).negation();
			// Add piece to the final quotient
			quotient = &quotient + &piece;
		}

		(quotient, dividend)
	}

	pub fn degree(&self) -> usize {
		self.coeffs.len() - 1
	}

	pub fn negation(&self) -> Self {
		Self {
			coeffs: self.coeffs.iter().map(|x| x.negation()).collect(),
		}
	}
}

impl<const M: u32> Add for &'_ Polynomial<M> {
	type Output = Polynomial<M>;

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

impl<const M: u32> Add for Polynomial<M> {
	type Output = Polynomial<M>;

	fn add(self, rhs: Self) -> Self::Output {
		&self + &rhs
	}
}

impl<const M: u32> Mul for &'_ Polynomial<M> {
	type Output = Polynomial<M>;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut coeffs = vec![Gfe::zero(); self.coeffs.len() + rhs.coeffs.len()];
		for i in 0..self.coeffs.len() {
			for j in 0..rhs.coeffs.len() {
				coeffs[i + j] = coeffs[i + j] + self.coeffs[i] * rhs.coeffs[j];
			}
		}
		Polynomial::new(coeffs)
	}
}

impl<const M: u32> Mul for Polynomial<M> {
	type Output = Polynomial<M>;

	fn mul(self, rhs: Self) -> Self::Output {
		&self * &rhs
	}
}

impl<const M: u32> PartialEq for Polynomial<M> {
	fn eq(&self, other: &Self) -> bool {
		// TODO: ensure invariants are held such that this implementation is valid
		self.coeffs == other.coeffs
	}
}

impl<const M: u32> Display for Polynomial<M> {
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
fn add_test() {
	let a = gfe_poly(&[1, 3, 3]);
	let b = gfe_poly(&[2, 5, 3, 2]);
	assert_eq!(a + b, gfe_poly::<19>(&[3, 8, 6, 2]));
}

#[test]
fn multiply_test() {
	let a = Polynomial::from_roots(&[Gfe::from(1), Gfe::from(-3)]);
	let b = Polynomial::from_roots(&[Gfe::from(-2), Gfe::from(-1)]);
	let y = &a * &b;
	let c1 = Polynomial::new(vec![Gfe::from(-6), Gfe::from(-5), Gfe::from(5), Gfe::from(5), Gfe::from(1)]);
	let c2 = Polynomial::from_roots(&[Gfe::<19>::from(1), Gfe::from(-3), Gfe::from(-2), Gfe::from(-1)]);
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
	assert_eq!(y.1.coeffs, Polynomial::constant(Gfe::<19>::zero()).coeffs);
}

#[test]
fn divide_test_non_one_leading_coeff() {
	let a = gfe_poly_roots(&[1, 3, 5]);
	let b = gfe_poly_roots(&[3, 5]);
	assert_eq!(a.divide(&b), (gfe_poly_roots(&[1]), Polynomial::<19>::zero()));

	let a = gfe_poly_roots(&[1, 5, 3]) * gfe_poly_const(4);
	let b = gfe_poly_roots(&[1, 5]) * gfe_poly_const(2);
	assert_eq!(a.divide(&b), (gfe_poly_roots(&[3]) * gfe_poly_const(2), Polynomial::<19>::zero()));
}

#[cfg(test)]
fn gfe_poly<const M: u32>(coeffs: &[i64]) -> Polynomial<M> {
	Polynomial::new(coeffs.iter().map(|&x| Gfe::from(x)).collect())
}

#[cfg(test)]
fn gfe_poly_const<const M: u32>(constant: i64) -> Polynomial<M> {
	Polynomial::constant(Gfe::from(constant))
}

#[cfg(test)]
fn gfe_poly_roots<const M: u32>(roots: &[i64]) -> Polynomial<M> {
	Polynomial::from_roots(&roots.iter().map(|&x| Gfe::from(x)).collect::<Vec<_>>())
}
