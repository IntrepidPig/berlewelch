use crate::*;
use rand::Rng;

#[test]
fn random_trials() {
	const TRIALS: usize = 20;
	const N_MIN: usize = 10;
	const N_MAX: usize = 50;
	const K_MIN: usize = 10;
	const K_MAX: usize = 20;

	for _ in 0..TRIALS {
		// Length of message
		let n: usize = rand::thread_rng().gen_range(N_MIN..=N_MAX);
		// Number of errors to tolerate
		let k: usize = rand::thread_rng().gen_range(K_MIN..=K_MAX);

		// Randomly generated message
		let message = rand::thread_rng()
			.sample_iter(rand::distributions::Uniform::new(0, *crate::field::P))
			.take(n)
			.map(|x| Gfe::new(x))
			.collect::<Vec<_>>();
		// Encoded error resistant message
		let encoded = encode(k, &message);

		// Corrupt the message with e general errors
		//let e: usize = rand::thread_rng().gen_range(0..=k); // TODO
		let e = k;
		let mut corrupted = encoded.clone();
		corrupt(&mut corrupted, e);

		decode(k, &mut corrupted).unwrap();
		assert_eq!(corrupted, encoded);
	}
}

fn corrupt(message: &mut [Gfe], e: usize) {
	for i in rand::seq::index::sample(&mut rand::thread_rng(), message.len(), e) {
		message[i] = rand_gfe_except(message[i]);
	}
}

fn rand_gfe_except(x: Gfe) -> Gfe {
	let y = rand::thread_rng().sample(rand::distributions::Uniform::new(0, *crate::field::P - 1));
	if y < *x {
		Gfe::new(y)
	} else {
		Gfe::new(y + 1)
	}
}

fn gfe_msg(ints: &[i64]) -> Vec<Gfe> {
	ints.iter().map(|&x| Gfe::from(x)).collect()
}

#[test]
fn specific_trials() {
	let message = gfe_msg(&[1, 5, 3, 4]);
	let k = 2;
	let encoded = encode(k, &message);
	let mut corrupted = encoded.clone();
	corrupted[1] = Gfe::new(6);
	let mut decoded = corrupted.clone();
	decode(k, &mut decoded).unwrap();
	println!("Message: {message:?}\nEncoded: {encoded:?}\nCorrupted: {corrupted:?}\nDecoded: {decoded:?}");
	assert_eq!(&message, &decoded[..message.len()]);
}
