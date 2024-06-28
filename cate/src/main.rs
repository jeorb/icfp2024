use std::env;
use clap::Parser;

mod bscii;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    encode: String,
    #[arg(short, long)]
    decode: String,
    #[arg(short, long)]
    message: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.encode.is_empty() {
        encode(&args.message).await;
    }

    if !args.decode.is_empty() {
        decode(&args.message).await;
    }

    if !args.message.is_empty() {
        message(&args.message).await;
    }

    Ok(())
}


async fn encode(msg: &str){

}

async fn decode(msg: &str){

}

async fn message(msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let icfp_token = env::var("ICFP_TOKEN").expect("Missing ICFP_TOKEN environment variable.");
    let message = "S'%4}).$%8";

    println!("Sending message: {}", message);
    println!("Sending message: {}", bscii::decode_str(message));

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


