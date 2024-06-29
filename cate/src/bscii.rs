use bytes::Bytes;

const BSCII: &str = r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ 
"##;


fn get_map() -> [char; 256]{
    let mut char_map: [char; 256] = ['X' as char; 256];
    let mut i = 33;
    for c in BSCII.chars() {
        char_map[i] = c;
        i += 1;
    }
    return char_map;
}


fn get_reverse_map() -> [char; 256]{
    let mut char_map: [char; 256] = ['X' as char; 256];
    let mut i: u8 = 33;
    for c in BSCII.chars() {
        char_map[c as usize] = i as char;
        i += 1;
    }
    return char_map;
}


pub fn encode_str(encoded: &str) -> String{
    let mut decoded = "".to_owned();
    let char_map = get_reverse_map();

    for char in encoded.chars() {
        decoded.push(char_map[char as usize]);
    }

    return decoded;
}


pub fn decode_str(encoded: &str) -> String{
    let mut decoded = "".to_owned();
    let char_map = get_map();

    for char in encoded.chars() {
        decoded.push(char_map[char as usize]);
    }

    return decoded;
}

pub fn decode_bytes(encoded: &Bytes) -> String{
    let mut decoded = "".to_owned();
    let char_map = get_map();

    for byte in encoded {
        decoded.push(char_map[*byte as usize]);
    }

    return decoded;
}

