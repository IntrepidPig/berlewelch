use self::{polynomial::Polynomial, matrix::Matrix};

pub mod field;
pub mod polynomial;
pub mod matrix;
#[cfg(test)]
mod test;

pub use self::field::Gfe;

/// Construct an error resistant message for a given message r. k is the number
/// of general errors to protect against. The encoded message will have 2k extra
/// values.
pub fn encode(k: usize, r: &[Gfe]) -> Vec<Gfe> {
	// Length of initial message
	let n = r.len();
	// Convert message to a set of points
	let p = r.iter().enumerate().map(|x| (Gfe::from(x.0 as i64), *x.1)).collect::<Vec<_>>();
	// Construct the unique polynomial of degree at most n-1 that passes through
	// these points.
	let poly = Polynomial::from_points(&p);
	// Construct the error-resistant message by including an additional 2k
	// points on the polynomial in the message.
	(0..(n+2*k)).map(|x| poly.eval(Gfe::from(x as i64))).collect()
}

/// Correct a message with up to k corruptions. It will be present in the first
/// r.len()-2*k items in r.
pub fn decode(k: usize, r: &mut [Gfe]) -> Result<(), ()> {
	// Length of the message
	let n = r.len() - 2 * k;
	// Total number of message packets and the total number of unknown coefficients
	// Number of rows and columns in non-augmented matrix
	let z = n + 2 * k;
	assert_eq!(z, r.len());

	let mut mat = Matrix {
		// Each row corresponds to a point received in the message
		m: z,
		// Columns are stored as follows:
		// The first n+k columns are coefficients a_0..a_{n+k-1} of polynomial Q(x) (degree n+k-1)
		// The next k columns are coefficients b_0..b_{k-1} of polynomial E(x) (degree k, leading coefficient = 1 by definition)
		// There is one more column to represent the augmented nature of the matrix
		n: z+1,
		// Entries initialized to zero to start
		elems: vec![Gfe::from(0); z * (z + 1)],
	};
	// Initialize entries of the matrix. For each row i (i = index in message and input to polynomial as well)
	for i in 0..z {
		// For each coefficient a of Q(x)
		for a in 0..(n+k) {
			// Set coefficient a_{ai} = i^{ai}
			*mat.elem_mut(i, a) = Gfe::new(i as u32).power(a as i32); // a_{0..n+k-1} * i^(0..n+k-1)
		}
		for b in 0..k {
			// Set the coefficient b_{bi} = -
			*mat.elem_mut(i, n + k + b) = (r[i] * Gfe::new(i as u32).power(b as i32)).negation(); // b_{0..k-1} * -r_i * i^(0..k-1)
		}
		*mat.elem_mut(i, n + 2 * k) = r[i] * Gfe::new(i as u32).power(k as i32); // = r_i * i^k
	}
	mat.row_reduce();

	// TODO: assert that:
	// 1. the matrix implies that there is a unique solution OR
	// the matrix implies that there are infinitely many solutions and all of the parameters are in the error polynomial (is this second part even a thing?)
	// 2. the system is not inconsistent

	// This assumes that there is a unique solution or the only parameters are the errors. It makes use of this since it can
	// then simply ignore error parameters by setting them to zero. Some more work might be required to detect cases where the
	// system is inconsistent or otherwise invalid.
	let mut q_coeffs = Vec::new();
	for i in 0..(n+k) {
		if let Some(row) = (0..z).find(|&row| mat.elem(row, i) == Gfe::one()) {
			q_coeffs.push(mat.elem(row, z));
		} else {
			// ERROR: No determinate value for coefficient i of Q polynomial
			return Err(());
		}
	}

	let mut e_coeffs = Vec::new();
	for i in (n+k)..(n+2*k) {
		if let Some(row) = (0..z).find(|&row| (0..i).all(|j| mat.elem(row, j) == Gfe::zero()) && mat.elem(row, i) == Gfe::one()) {
			e_coeffs.push(mat.elem(row, z));
		} else {
			// If this error is a parameter just we are assuming it is zero.
			e_coeffs.push(Gfe::zero());
		}
	}
	e_coeffs.push(Gfe::new(1));


	let q = Polynomial::new(q_coeffs);
	let e = Polynomial::new(e_coeffs);
	let (p, rem) = q.divide(&e);
	
	if rem != Polynomial::zero() {
		// Nonzero remainder indicates decoding failed
		return Err(())
	}

	for i in 0..(n+2*k) {
		r[i] = p.eval(Gfe::from(i as i64));
	}

	Ok(())
}




