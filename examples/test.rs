use berlewelch::*;

fn main() {
    let mut m = encode(2, &[Gfe::new(1), Gfe::new(3), Gfe::new(7), Gfe::new(2), Gfe::new(8), Gfe::new(1)]);
    println!("Constructed full message {m:?}");
    //let p = Polynomial::from_points(&[(Gfe::new(1), Gfe::new(2)), (Gfe::new(3), Gfe::new(6)), (Gfe::new(0), Gfe::new(3))]);
    //let p = Polynomial::from_zeroes(&[Gfe::new(1), Gfe::new(2), Gfe::new(5)]); //good
    //let p = &Polynomial::new(vec![Gfe::new(2), Gfe::new(1)]) * &Polynomial::new(vec![Gfe::new(3), Gfe::new(2), Gfe::new(3)]); //good
    m[1] = m[1] + Gfe::new(1);
    m[3] = m[3] + Gfe::new(2);
    println!("Message corrupted to {m:?}");
    correct(2, &mut m);
    println!("Message corrected to {m:?}");
    /* let q = Polynomial::new(vec![Gfe::new(1).negation(), Gfe::new(9), Gfe::new(5), Gfe::new(10)]);
    let e = Polynomial::new(vec![Gfe::new(1).negation(), Gfe::new(1)]);
    let p = q.divide(&e);
    println!("{q} / {e} = {p}"); */
}