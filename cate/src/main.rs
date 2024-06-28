use std::env;


const ICFPSCII: &str = r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ 
"##;
//const ASCII: &str = r##" !"#$%&'()*+,-./:;<=>?@[\]^_`|~0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"##;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ascii: Vec<char> = ICFPSCII.chars().collect();
    let ascii: [char; 94] = ascii.try_into().expect("ASCII");

    let icfp_token = env::var("ICFP_TOKEN").expect("Missing ICFP_TOKEN environment variable.");
    let message = "S'%4}).$%8";

    println!("Sending message: {}", message);
    print!("Sending message: ", );
    for char in message.chars() {
        let i: usize = (char as usize - 33) as usize;
        print!("{}", ascii[i]);
    }
    println!("");

    let client = reqwest::Client::new();
    let resp = client.post("https://boundvariable.space/communicate")
        .header("Authorization", icfp_token)
        .body(message)
        .send()
        .await?;

        println!("resp: {resp:#?}");

        //let body = resp.text().await?;
        //println!("body:  {body:#?}");

        let body = resp.bytes().await?;

        for byte in &body {
            let i: usize = (byte - 33) as usize;
            print!("{}", ascii[i]);
        }

        //println!("body:  {body:#?}");

        Ok(())
}



