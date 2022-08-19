use crate::*;
use rand::Rng;

#[test]
fn random_trials() {
    const M: u32 =  0x7fffffff; // 2^31-1
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
			.sample_iter(rand::distributions::Uniform::new(0, M))
			.take(n)
			.map(|x| Gfe::<M>::new(x))
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

fn corrupt<const M: u32>(message: &mut [Gfe<M>], e: usize) {
	for i in rand::seq::index::sample(&mut rand::thread_rng(), message.len(), e) {
		message[i] = rand_gfe_except(message[i]);
	}
}

fn rand_gfe_except<const M: u32>(x: Gfe<M>) -> Gfe<M> {
	let y = rand::thread_rng().sample(rand::distributions::Uniform::new(0, M - 1));
	if y < *x {
		Gfe::new(y)
	} else {
		Gfe::new(y + 1)
	}
}

fn gfe_msg<const M: u32>(ints: &[i64]) -> Vec<Gfe<M>> {
	ints.iter().map(|&x| Gfe::from(x)).collect()
}

#[test]
fn specific_trials() {
	let message = gfe_msg(&[1, 5, 3, 4]);
	let k = 2;
	let encoded = encode(k, &message);
	let mut corrupted = encoded.clone();
	corrupted[1] = Gfe::<19>::new(6);
	let mut decoded = corrupted.clone();
	decode(k, &mut decoded).unwrap();
	println!("Message: {message:?}\nEncoded: {encoded:?}\nCorrupted: {corrupted:?}\nDecoded: {decoded:?}");
	assert_eq!(&message, &decoded[..message.len()]);
}
