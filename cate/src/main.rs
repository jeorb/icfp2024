use std::env;
use clap::{Parser, Subcommand};

mod bscii;
use bscii::Token;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode ASCII string to BSCII
    Encode {
        /// ASCII string
        message: String,
    },
    /// Encode BSCII string to ASCII
    Decode {
        /// BSCII string
        message: String,
        /// Process the BSCII string
        #[arg(long, default_value_t = false)]
        process: bool,
    },
    /// Send message to https://boundvariable.space/communicate.
    /// 
    /// Provided message will be encoded from ASCII to a BSCII String.
    /// 
    /// ICFP_TOKEN must be provided by the environment (source ../env.sh).
    Send {
        /// ASCII string
        message: String,
        /// Print the raw decoded response from the server with no structure or type definitions
        #[arg(long, default_value_t = false)]
        raw: bool,
        /// Process the response from the server
        #[arg(long, default_value_t = false)]
        process: bool,
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let encoder = bscii::Encoder::new();

    match &cli.command {
        Some(Commands::Encode { message }) => {
            let _ = encode(&encoder, &message).await;
        }
        Some(Commands::Decode { message, process }) => {
            let _ = decode(&encoder, &message, *process).await;
        }
        Some(Commands::Send { message, raw, process }) => {
            let _ = send(&encoder, &message, *raw, *process).await;
        }
        None => {}
    }

    Ok(())
}


async fn encode(encoder: &bscii::Encoder, msg: &str){
    println!("{}", encoder.encode_str(msg));
}


async fn decode(encoder: &bscii::Encoder, msg: &str, process: bool){
    let tokens = encoder.decode_bytes(&bytes::Bytes::from(msg.to_owned()));
    if process {
        process_tokens(&tokens);
    } else {
        print_tokens(&tokens);
    }
}


fn print_tokens(tokens: &Vec<Token>){
    let mut stack: Vec<usize> = Vec::new();
    for token in tokens {
        print!("{}", indent(stack.len()));
        match token {
            Token::Root               => { println!("Root"); },
            Token::String   { value } => { println!("\"{}\"", value); },
            Token::Boolean  { value } => { println!("{}", value); },
            Token::Integer  { value } => { println!("{}", value); },
            Token::Unary    { value } => { println!("{} (", value); },
            Token::Binary   { value } => { println!("{} (", value); },
            Token::Lambda   { value } => { println!("v{} (", value); },
            Token::Variable { value } => { println!("v{}", value); },
            Token::Other    { value } => { println!("{}", value); },
        }
        if token.num_args() > 0 {
            stack.push(token.num_args());
        } else {
            while !stack.is_empty() {
                let mut last = stack.pop().expect("Reached below the bottom of the stack.");
                last = last - 1;

                if last == 0 {
                    println!("{})", indent(stack.len()));
                } else {
                    stack.push(last);
                    break;
                }
            }
        }
    }
}

#[derive(Debug,Clone)]
struct Frame {
    token: Token,
    args: Vec<Frame>,
}

fn print_frame(frame: Frame, depth: usize){
    println!("{}{} (", indent(depth), frame.token);
    for arg in frame.args {
        print_frame(arg, depth+1);
    }
    println!("{})", indent(depth));
}

impl Default for Frame {
    fn default() -> Frame {
        Frame {
            token: Token::Root,
            args: Vec::new(),
        }
    }
}

impl Frame {
    pub fn new() -> Frame {
        Default::default()
    }
}


fn indent(level: usize) -> String {
    format!("{space:>depth$}", space="", depth=level*4)
}


fn process_tokens(tokens: &Vec<Token>){
    let mut stack: Vec<Frame> = Vec::new();
    let mut top = Frame::new();
    for token in tokens {
        print!("{}", indent(stack.len()));
        match token {
            Token::Root               => { println!("Root"); },
            Token::String   { value } => { println!("\"{}\"", value); },
            Token::Boolean  { value } => { println!("{}", value); },
            Token::Integer  { value } => { println!("{}", value); },
            Token::Unary    { value } => { println!("{} (", value); },
            Token::Binary   { value } => { println!("{} (", value); },
            Token::Lambda   { value } => { println!("v{} (", value); },
            Token::Variable { value } => { println!("v{}", value); },
            Token::Other    { value } => { println!("{}", value); },
        }

        let mut frame = Frame { token: token.clone(), args: Vec::new() };
        if token.num_args() > 0 {
            stack.push(frame);
        } else {
            while !stack.is_empty() {
                let last = stack.last_mut().expect("Reached below the bottom of the stack.");
                last.args.push(frame);
                if last.args.len() == last.token.num_args() {
                    println!("{})", indent(stack.len()-1));
                    if stack.len() == 1 {
                        top = stack.pop().expect("Reached below the bottom of the stack.");
                        break;
                    } else {
                        frame = stack.pop().expect("Reached below the bottom of the stack.");
                    }
                } else {
                    break;
                }
            }
        }
    }

    print_frame(top, 0);

    /*
    let t = last.token.clone();
    let v = match t {
        Token::Binary { value } => {
            let x = match last.args[0].token {
                Token::Integer { value } => { value },
                _ => { 0 },
            };
            let y = match last.args[1].token {
                Token::Integer { value } => { value },
                _ => { 0 },
            };
            match value {
                BinaryOperator::IntegerAddition => { 
                    &format!("{}", x + y)
                },
                BinaryOperator::IntegerMultiplication => { 
                    &format!("{}", x * y)
                },
                _ => { "Binary!" }
            }
        },
        _ => { "" },
    };
    */

}


async fn send(encoder: &bscii::Encoder, msg: &str, raw: bool, process: bool) -> Result<(), Box<dyn std::error::Error>> {
    let icfp_token = env::var("ICFP_TOKEN").expect("Missing ICFP_TOKEN environment variable.");
    //"get index" == "S'%4}).$%8";
    let mut message: String = "S".to_owned();
    message.push_str(&encoder.encode_str(msg));

    if !raw {
        println!("Sending message: '{}' encoded as '{}'", msg, message);
    }

    let client = reqwest::Client::new();
    let resp = client.post("https://boundvariable.space/communicate")
        .header("Authorization", icfp_token)
        .body(message)
        .send()
        .await?;

    let body = resp.bytes().await?;

    let tokens = encoder.decode_bytes(&body);

    if raw {
        for token in tokens {
            match token {
                Token::Root               => { },
                Token::String   { value } => { println!("{}", value); },
                Token::Boolean  { value } => { println!("{}", value); },
                Token::Integer  { value } => { println!("{}", value); },
                Token::Unary    { value } => { println!("{}", value); },
                Token::Binary   { value } => { println!("{}", value); },
                Token::Lambda   { value } => { println!("{}", value); },
                Token::Variable { value } => { println!("{}", value); },
                Token::Other    { value } => { println!("{}", value); },
            }
        }
    } else if process {
        process_tokens(&tokens);
    } else {
        print_tokens(&tokens);
    }

    Ok(())
}


