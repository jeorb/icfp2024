use bytes::Bytes;

const BSCII: &str = r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ 
"##;


fn get_map() -> [char; 94]{
    let char_map: Vec<char> = BSCII.chars().collect();
    let char_map: [char; 94] = char_map.try_into().expect("ASCII");
    return char_map;
}


pub fn decode_str(encoded: &str) -> String{
    let mut decoded = "".to_owned();
    let char_map = get_map();

    for char in encoded.chars() {
        let i: usize = (char as usize - 33) as usize;
        decoded.push(char_map[i]);
    }

    return decoded;
}

pub fn decode_bytes(encoded: &Bytes) -> String{
    let mut decoded = "".to_owned();
    let char_map = get_map();

    for byte in encoded {
        let i: usize = (byte - 33) as usize;
        print!("{}", char_map[i]);
    }

    return decoded;
}

