use std::fmt::Display;

use crate::field::Gfe;

/// A matrix over GF(p) with m rows and n columns
#[derive(Debug, Clone)]
pub struct Matrix<const M: u32> {
	pub m: usize,
	pub n: usize,
	pub elems: Vec<Gfe<M>>,
}

impl<const M: u32> Matrix<M> {
	/// Multiply row i by scalar c
	pub fn mul_row(&mut self, i: usize, c: Gfe<M>) {
		let (_m, n) = (self.m, self.n);
		for j in 0..n {
			self.elems[i * n + j] = self.elems[i * n + j] * c;
		}
	}

	/// Add c * row[i1] to row[i2]
	pub fn add_c_row_to(&mut self, i1: usize, i2: usize, c: Gfe<M>) {
		let (_m, n) = (self.m, self.n);
		for j in 0..n {
			self.elems[i2 * n + j] = self.elems[i2 * n + j] + self.elems[i1 * n + j] * c;
		}
	}

	/// Swap rows i1 and i2
	pub fn swap_rows(&mut self, i1: usize, i2: usize) {
		let (_m, n) = (self.m, self.n);
		for j in 0..n {
			let tmp = self.elems[i1 * n + j];
			self.elems[i1 * n + j] = self.elems[i2 * n + j];
			self.elems[i2 * n + j] = tmp;
		}
	}

	pub fn elem(&self, i: usize, j: usize) -> Gfe<M> {
		self.elems[i * self.n + j]
	}

	pub fn elem_mut(&mut self, i: usize, j: usize) -> &mut Gfe<M> {
		&mut self.elems[i * self.n + j]
	}

	/// Attempt to row reduce the matrix as far as possible down the diagonal from left to right
	pub fn row_reduce(&mut self) {
		let (m, n) = (self.m, self.n);

		// The next row is the index of the earliest row that doesn't have a pivot yet
		let mut next_row = 0;
		// Reduce the matrix by each column from left-to-right
		for column in 0..n {
			// Find the next row with a non zero element at this column that is not already a pivot row.
			if let Some(nonzero_row) = (next_row..m).find(|&row| self.elem(row, column) != Gfe::zero()) {
				// Swap that row with the whatever row is current in the position of the next to-be pivot row.
				self.swap_rows(nonzero_row, next_row);
				// Rename the pivot row to just row
				let row = next_row;
				// Set the pivot element of this row to 1
				self.mul_row(row, self.elem(row, column).inverse());
				// Add multiples of this row to all other rows such that elements in the same column as
				// this row's pivot are zeroed out
				for other_row in (0..row).chain((row + 1)..m) {
					self.add_c_row_to(row, other_row, self.elem(other_row, column).negation());
				}
				// The following row will be the location of the next pivot
				next_row += 1;
			}
		}
	}
}

#[test]
fn test_row_reduce_non_square_non_trivial() {
	use crate::field::Gfe19;
	#[rustfmt::skip]
	let elems = [
		1, 2, 0, 1, -1, 6,
		1, 3, 0, 1, 2, 2,
		0, 3, 0, 2, 5, 3,
		2, 5, 0, 2, 1, 8,
		3, 2, 0, 6, -4, 3,
	]
		.into_iter()
		.map(|x| Gfe19::from(x))
		.collect::<Vec<_>>();
	let mut matrix = Matrix { m: 5, n: 6, elems };
	matrix.row_reduce();
	println!("{matrix}");
}

#[test]
fn test_row_reduce_inconsistent() {
	use crate::field::Gfe19;
	#[rustfmt::skip]
	let elems = [
		1, 2, 3, 5,
		1, 2, 3, 6
	]
		.into_iter().map(|x| Gfe19::from(x)).collect::<Vec<_>>();
	let mut matrix = Matrix { m: 2, n: 4, elems };
	matrix.row_reduce();
	println!("{matrix}");
}

impl<const M: u32> Display for Matrix<M> {
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
