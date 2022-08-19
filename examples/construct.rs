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
    let orig_text = std::env::args().nth(1).expect("Message not given");
    let mut text = orig_text.as_bytes().to_owned();
    for i in 0..text.len() {
        text[i] = ascii_to_c29(text[i])
    }
    let gfe_text = text.iter().map(|x| Gfe::from(*x as i64)).collect::<Vec<_>>();
    let encoded = encode(2, &gfe_text);
    let encoded_text = String::from_utf8(encoded.iter().map(|x| c29_to_ascii(**x as u8)).collect::<Vec<_>>()).unwrap();
    println!("message: \"{orig_text}\"");
    println!("error resistant message: \"{encoded_text}\"");
}