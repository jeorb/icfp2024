use core::fmt;

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
    Unary {
        value: UnaryOperator,
    },
    Other {
        value: String,
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    IntegerNegation,
    BooleanNot,
    StringToInt,
    IntToString,
    Invalid,
}

impl ::core::fmt::Display for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Boolean { value: v } => { write!(f, "Boolean: {}", &v) },
            Token::Integer { value: v } => { write!(f, "Integer: {}", &v) },
            Token::String  { value: v } => { write!(f, "String: {}",  &v) },
            Token::Unary   { value: v } => { write!(f, "Unary: {}",   &v) },
            Token::Other   { value: v } => { write!(f, "Other: {}",   &v) },
        }
    }
}


impl ::core::fmt::Display for UnaryOperator {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperator::IntegerNegation => { write!(f, "IntegerNegation") },
            UnaryOperator::BooleanNot      => { write!(f, "BooleanNot") },
            UnaryOperator::StringToInt     => { write!(f, "StringToInt") },
            UnaryOperator::IntToString     => { write!(f, "IntToString") },
            UnaryOperator::Invalid         => { write!(f, "InvalidUnaryOperator") },
        }
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
    
    
    /*pub fn decode_str(&self, encoded: &str) -> String{
        let mut decoded = "".to_owned();
    
        for char in encoded.chars() {
            decoded.push(self.bscii[char as usize]);
        }
    
        return decoded;
    }*/
    
    
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
                let mut v: u32 = 0;
                for byte in encoded.iter().skip(1) {
                    v = (v * 94) + (*byte as u32 - 33);
                }
                Token::Integer { value: v }  
            },
            b'S' => {
                for byte in encoded.iter().skip(1) {
                    decoded.push(self.bscii[*byte as usize]);
                }
                Token::String { value: decoded }  
            },
            b'U' => {
                let op = match encoded[1] {
                    b'-' => { UnaryOperator::IntegerNegation }
                    b'!' => { UnaryOperator::BooleanNot }
                    b'#' => { UnaryOperator::StringToInt }
                    b'$' => { UnaryOperator::IntToString }
                    _ => { UnaryOperator::Invalid }
                };
                Token::Unary { value: op }  
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



