use core::fmt;

use bytes::Bytes;


pub struct Encoder {
    ascii: [char; 256],
    bscii: [char; 256],
}

#[derive(Debug, Clone)]
pub enum Token {
    Root,
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
    Binary {
        value: BinaryOperator,
    },
    Lambda {
        value: u32,
    },
    Variable {
        value: u32,
    },
    Other {
        value: String,
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    IntegerNegation,
    BooleanNot,
    StringToInt,
    IntToString,
    Invalid {
        value: String,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    IntegerAddition,
    IntegerSubtraction,
    IntegerMultiplication,
    IntegerDivision,
    IntegerModulo,
    IntegerLessThan,
    IntegerGreaterThan,
    Equality,
    StringConcatenation,
    StringTake,
    StringDrop,
    Apply,
    Invalid {
        value: String,
    },
}

impl Token {
    pub fn num_args(&self) -> usize {
        match self {
            Token::Unary     { value: _ } => { 1 },
            Token::Binary    { value: _ } => { 2 },
            Token::Lambda    { value: _ } => { 1 },
            _                             => { 0 },
        }
    }
}

impl ::core::fmt::Display for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Root                   => { write!(f, "Root",           ) },
            Token::Boolean   { value: v } => { write!(f, "Boolean: {}",  &v) },
            Token::Integer   { value: v } => { write!(f, "Integer: {}",  &v) },
            Token::String    { value: v } => { write!(f, "String: {}",   &v) },
            Token::Unary     { value: v } => { write!(f, "Unary: {}",    &v) },
            Token::Binary    { value: v } => { write!(f, "Binary: {}",   &v) },
            Token::Lambda    { value: v } => { write!(f, "Lambda: {}",   &v) },
            Token::Variable  { value: v } => { write!(f, "Variable: {}", &v) },
            Token::Other     { value: v } => { write!(f, "Other: {}",    &v) },
        }
    }
}


impl ::core::fmt::Display for UnaryOperator {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperator::IntegerNegation   => { write!(f, "IntegerNegation") },
            UnaryOperator::BooleanNot        => { write!(f, "BooleanNot") },
            UnaryOperator::StringToInt       => { write!(f, "StringToInt") },
            UnaryOperator::IntToString       => { write!(f, "IntToString") },
            UnaryOperator::Invalid { value } => { write!(f, "InvalidUnaryOperator \"{}\"", value) },
        }
    }
}


impl ::core::fmt::Display for BinaryOperator {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOperator::IntegerAddition       => { write!(f, "IntegerAddition") },
            BinaryOperator::IntegerSubtraction    => { write!(f, "IntegerSubtraction") },
            BinaryOperator::IntegerMultiplication => { write!(f, "IntegerMultiplication") },
            BinaryOperator::IntegerDivision       => { write!(f, "IntegerDivision") },
            BinaryOperator::IntegerModulo         => { write!(f, "IntegerModulo") },
            BinaryOperator::IntegerLessThan       => { write!(f, "IntegerLessThan") },
            BinaryOperator::IntegerGreaterThan    => { write!(f, "IntegerGreaterThan") },
            BinaryOperator::Equality              => { write!(f, "Equality") },
            BinaryOperator::StringConcatenation   => { write!(f, "StringConcatenation") },
            BinaryOperator::StringTake            => { write!(f, "StringTake") },
            BinaryOperator::StringDrop            => { write!(f, "StringDrop") },
            BinaryOperator::Apply                 => { write!(f, "Apply") },
            BinaryOperator::Invalid { value }     => { write!(f, "InvalidBinaryOperator \"{}\"", value) },
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
            b'L' => {
                let mut v: u32 = 0;
                for byte in encoded.iter().skip(1) {
                    v = (v * 94) + (*byte as u32 - 33);
                }
                Token::Lambda { value: v }  
            },
            b'v' => {
                let mut v: u32 = 0;
                for byte in encoded.iter().skip(1) {
                    v = (v * 94) + (*byte as u32 - 33);
                }
                Token::Variable { value: v }  
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
                    _ => { UnaryOperator::Invalid { value: std::str::from_utf8(encoded).expect("invalid ASCII bytes").to_owned() } }
                };
                Token::Unary { value: op }  
            },
            b'B' => {
                let op = match encoded[1] {
                    b'+' => { BinaryOperator::IntegerAddition }
                    b'-' => { BinaryOperator::IntegerSubtraction }
                    b'*' => { BinaryOperator::IntegerMultiplication }
                    b'/' => { BinaryOperator::IntegerDivision }
                    b'%' => { BinaryOperator::IntegerModulo }
                    b'<' => { BinaryOperator::IntegerLessThan }
                    b'>' => { BinaryOperator::IntegerGreaterThan }
                    b'=' => { BinaryOperator::Equality }
                    b'.' => { BinaryOperator::StringConcatenation }
                    b'T' => { BinaryOperator::StringTake }
                    b'D' => { BinaryOperator::StringDrop }
                    b'$' => { BinaryOperator::Apply }
                    _ => { BinaryOperator::Invalid { value: std::str::from_utf8(encoded).expect("invalid ASCII bytes").to_owned() } }
                };
                Token::Binary { value: op }  
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



