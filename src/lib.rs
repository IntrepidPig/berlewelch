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
pub fn correct(k: usize, r: &mut [Gfe]) {
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
		for ai in 0..(n+k) {
			// Set coefficient a_{ai} = i^{ai}
			mat.elems[i * (z + 1) + ai] = Gfe::new(i as u32).power((ai) as i32); // a_{0..n+k-1} * i^(0..n+k-1)
		}
		for bi in 0..k {
			mat.elems[i * (z + 1) + n + k + bi] = (r[i] * Gfe::new(i as u32).power((bi) as i32)).negation(); // b_{0..k-1} * -r_i * i^(0..k-1)
		}
		mat.elems[i * (z + 1) + n + 2 * k] = r[i] * Gfe::new(i as u32).power(k as i32); // = r_i * i^k
	}
	println!("{mat}");
	mat.row_reduce();
	println!("{mat}");

	let q = Polynomial::new((0..n+k).map(|i| mat.elems[(i + 1) * (z + 1) - 1]).collect::<Vec<_>>());
	let mut e = Polynomial::new((n+k..n+2*k).map(|i| mat.elems[(i + 1) * (z + 1) - 1]).collect::<Vec<_>>());
	e.coeffs.push(Gfe::new(1));
	println!("Q = {q:?}\nE = {e:?}");
	let (p, rem) = q.divide(&e);
	assert!(&rem.coeffs == &[Gfe::zero()]);
	for i in 0..(n+2*k) {
		r[i] = p.eval(Gfe::from(i as i64));
	}
	// get polynomials Q(x) and E(x) from solved coefficients in matrix
	// divide Q(x) by E(x) to get P(x)
}




