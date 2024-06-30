use bytes::Bytes;


pub struct Encoder {
    ascii: [char; 256],
    bscii: [char; 256],
}

#[derive(Debug)]
pub enum Token {
    Boolean {
        value: bool,
    },
    Integer {
        value: u32,
    },
    String {
        value: String,
    },
    Other {
        value: String,
    }
}

impl Default for Encoder {
    fn default() -> Encoder {
        Encoder {
            ascii: get_reverse_map(),
            bscii: get_map(),
        }
    }
}

impl Encoder {
    pub fn new() -> Encoder {
        Default::default()
    }
    
    pub fn encode_str(&self, encoded: &str) -> String{
        let mut decoded = "".to_owned();
    
        for char in encoded.chars() {
            decoded.push(self.ascii[char as usize]);
        }
    
        return decoded;
    }
    
    
    pub fn decode_str(&self, encoded: &str) -> String{
        let mut decoded = "".to_owned();
    
        for char in encoded.chars() {
            decoded.push(self.bscii[char as usize]);
        }
    
        return decoded;
    }
    
    
    pub fn decode_token(&self, encoded: &[u8]) -> Token {
        let mut decoded = "".to_owned();
    
        match encoded[0] {
            b'T' => {
                Token::Boolean { value: true }  
            },
            b'F' => {
                Token::Boolean { value: false }  
            },
            b'I' => {
                Token::Integer { value: encoded[1] as u32 }  
            },
            b'S' => {
                for byte in encoded.iter().skip(1) {
                    decoded.push(self.bscii[*byte as usize]);
                }
                Token::String { value: decoded }  
            },
            _ => {
                Token::Other { value: std::str::from_utf8(encoded).expect("invalid ASCII bytes").to_owned() }
            }
        }
    }
    
    
    pub fn decode_bytes(&self, encoded: &Bytes) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
    
        let encoded_tokens = encoded.split(|&b| b == b' ');
    
        for encoded_token in encoded_tokens {
            tokens.push(self.decode_token(encoded_token));
        }
    
        return tokens;
    }
}

const BSCII: &str = r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ 
"##;

/// Returns array with the ascii byte as the index and the bscii byte as the value
fn get_map() -> [char; 256]{
    let mut char_map: [char; 256] = ['X' as char; 256];
    let mut i = 33;
    for c in BSCII.chars() {
        char_map[i] = c;
        i += 1;
    }
    return char_map;
}


/// Returns array with the bscii byte as the index and the ascii byte as the value
fn get_reverse_map() -> [char; 256]{
    let mut char_map: [char; 256] = ['X' as char; 256];
    let mut i: u8 = 33;
    for c in BSCII.chars() {
        char_map[c as usize] = i as char;
        i += 1;
    }
    return char_map;
}



