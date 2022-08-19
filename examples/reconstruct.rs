use berlewelch::*;

fn ascii_to_c29(ascii: u8) -> u8 {
    if ascii == b' ' {
        return 26;
    }
    if ascii == b'.' {
        return 27;
    }
    if ascii == b'_' {
        return 28;
    }

    assert!(b'a' <= ascii);
    assert!(ascii <= b'z');
    
    return ascii - b'a';
}

fn c29_to_ascii(c29: u8) -> u8 {
    if c29 == 26 {
        return b' ';
    }
    if c29 == 27 {
        return b'.';
    }
    if c29 == 28 {
        return b'_';
    }

    assert!(c29 <= 25);
    
    return b'a' + c29;
}

fn main() {
    let orig_enc_text = std::env::args().nth(1).expect("Message not given");
    let mut enc_text = orig_enc_text.as_bytes().to_owned();
    for i in 0..enc_text.len() {
        enc_text[i] = ascii_to_c29(enc_text[i])
    }
    let mut gfe_text = enc_text.iter().map(|x| Gfe::from(*x as i64)).collect::<Vec<_>>();
    correct(2, &mut gfe_text);
    let fixed_text = String::from_utf8(gfe_text[0..(gfe_text.len() - 4)].iter().map(|x| c29_to_ascii(**x as u8)).collect::<Vec<_>>()).unwrap();
    println!("corrupted message: \"{orig_enc_text}\"");
    println!("repaired message: \"{fixed_text}\"");
}