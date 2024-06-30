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
    },
    /// Send message to https://boundvariable.space/communicate.
    /// 
    /// Provided message will be encoded from ASCII to a BSCII String.
    /// 
    /// ICFP_TOKEN must be provided by the environment (source ../env.sh).
    Send {
        /// ASCII string
        message: String,
        #[arg(long, default_value_t = false)]
        /// Print the raw decoded response from the server with no structure or type definitions
        raw: bool,
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
        Some(Commands::Decode { message }) => {
            let _ = decode(&encoder, &message).await;
        }
        Some(Commands::Send { message, raw }) => {
            let _ = send(&encoder, &message, *raw).await;
        }
        None => {}
    }

    Ok(())
}


async fn encode(encoder: &bscii::Encoder, msg: &str){
    println!("{}", encoder.encode_str(msg));
}


async fn decode(encoder: &bscii::Encoder, msg: &str){
    let tokens = encoder.decode_bytes(&bytes::Bytes::from(msg.to_owned()));
    for token in tokens {
        println!("{}", token);
    }
}


async fn send(encoder: &bscii::Encoder, msg: &str, raw: bool) -> Result<(), Box<dyn std::error::Error>> {
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
                Token::String { value } => { println!("{}", value); },
                Token::Boolean { value } => { println!("{}", value); },
                Token::Integer { value } => { println!("{}", value); },
                Token::Unary { value } => { println!("{}", value); },
                Token::Other { value } => { println!("{}", value); },
            }
        }
    } else {
        for token in tokens {
            println!("{}", token);
        }
    }

    Ok(())
}


