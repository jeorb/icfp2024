use std::env;
use clap::{Parser, Subcommand};

mod bscii;


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
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encode { message }) => {
            let _ = encode(&message).await;
        }
        Some(Commands::Decode { message }) => {
            let _ = decode(&message).await;
        }
        Some(Commands::Send { message }) => {
            let _ = send(&message).await;
        }
        None => {}
    }

    Ok(())
}


async fn encode(msg: &str){
    println!("{}", bscii::encode_str(msg));
}

async fn decode(msg: &str){
    println!("{}", bscii::decode_str(msg));
}

async fn send(msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let icfp_token = env::var("ICFP_TOKEN").expect("Missing ICFP_TOKEN environment variable.");
    //let message = "S'%4}).$%8";
    let mut message: String = "S".to_owned();

    println!("Sending message: {}", msg);
    message.push_str(&bscii::encode_str(msg));
    println!("Sending message: {}", message);

    let client = reqwest::Client::new();
    let resp = client.post("https://boundvariable.space/communicate")
        .header("Authorization", icfp_token)
        .body(message)
        .send()
        .await?;

        //println!("resp: {resp:#?}");

        //let body = resp.text().await?;
        //println!("body:  {body:#?}");

        let body = resp.bytes().await?;

        println!("body:  {}", bscii::decode_bytes(&body));

        Ok(())

}


