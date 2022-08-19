use std::fmt::Display;

use crate::field::Gfe;

/// A matrix over GF(p) with m rows and n columns
#[derive(Debug, Clone)]
pub struct Matrix {
	pub m: usize,
	pub n: usize,
	pub elems: Vec<Gfe>,
}

impl Matrix {
	/// Multiply row i by scalar c
	pub fn mul_row(&mut self, i: usize, c: Gfe) {
		println!("Multiplying row {i} by {c}");
		let (_m, n) = (self.m, self.n);
		for j in 0..n {
			self.elems[i * n + j] = self.elems[i * n + j] * c;
		}
	}

	/// Add c * row[i1] to row[i2]
	pub fn add_c_row_to(&mut self, i1: usize, i2: usize, c: Gfe) {
		println!("Adding {c} times row {i1} to row {i2}");
		let (_m, n) = (self.m, self.n);
		for j in 0..n {
			self.elems[i2 * n + j] = self.elems[i2 * n + j] + self.elems[i1 * n + j] * c;
		}
	}

	/// Swap rows i1 and i2
	pub fn swap_rows(&mut self, i1: usize, i2: usize) {
		println!("Swapping rows {i1} and {i2}");
		let (_m, n) = (self.m, self.n);
		for j in 0..n {
			let tmp = self.elems[i1 * n + j];
			self.elems[i1 * n + j] = self.elems[i2 * n + j];
			self.elems[i2 * n + j] = tmp;
		}
	}

	pub fn elem(&self, i: usize, j: usize) -> Gfe {
		self.elems[i * self.n + j]
	}

	pub fn elem_mut(&mut self, i: usize, j: usize) -> &mut Gfe {
		&mut self.elems[i * self.n + j]
	}

	/// Attempt to row reduce the matrix as far as possible down the diagonal from left to right
	pub fn row_reduce(&mut self) {
		let (m, _n) = (self.m, self.n);
		// For each row
		// TODO: do it by column instead to get better (correct) results
		for row in 0..m {
			// Find a row with a nonzero coefficient in the matching column for this row. If it is different from the
			// current row, swap them so that this row has a nonzero coefficient in its diagonal entry.
			let mut found = false;
			for row_below in row..m {
				if self.elem(row_below, row) != Gfe::zero() {
					if row_below != row {
						self.swap_rows(row_below, row);
					}
					found = true;
					break;
				}
			}

			// If this column is all 0 below, ignore it, it will become a parameter
			if !found {
				continue
			}

			// Set the diagonal entry of this row to 1
			self.mul_row(row, self.elem(row, row).inverse());
			println!("{self}");

			// For every row below this one, add a multiple of this one such that everything below this diagonal entry is 0
			for row_below in (row+1)..m {
				self.add_c_row_to(row, row_below, self.elem(row_below, row).negation());
			}
			println!("{self}");
		}

		// Backpropagate: For each row starting from the last, add a multiple of it to all rows above it to make all
		// entries in columns above this rows diagonal 0.
		for row in (0..m).rev() {
			for row_above in 0..row {
				self.add_c_row_to(row, row_above, self.elem(row_above, row).negation());
			}
		}
	}
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.m * self.n == 0 {
			return write!(f, "<empty matrix>");
		}

        let maxw = self.elems.iter().map(|x| format!("{x}").len()).max().unwrap();
		for i in 0..self.m {
			write!(f, "| ")?;
			for j in 0..self.n {
				let buf = format!("{}", self.elems[i * self.n + j]);
				for _ in 0..(maxw - buf.len() + 1) {
					write!(f, " ")?;
				}
				write!(f, "{buf} ")?;
			}
			write!(f, " |\n")?;
		}

		Ok(())
    }
}


#[test]
fn test_row_reduce_messy() {
	let elems = [
		1, 2, 0, 1, -1, 6,
		1, 3, 0, 1, 2, 2,
		0, 3, 0, 2, 5, 3,
		2, 5, 0, 2, 1, 8,
		3, 2, 0, 6, -4, 3,
	].into_iter().map(|x| Gfe::from(x)).collect::<Vec<_>>();
	let mut matrix = Matrix { m: 5, n: 6, elems };
	matrix.row_reduce();
	println!("{matrix}");
}