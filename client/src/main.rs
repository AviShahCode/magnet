use rustyline::DefaultEditor;
use anyhow::Result;
use rustyline::error::ReadlineError;
use client::ApiClient;

#[tokio::main]
async fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) => {
                println!("{}", line);
                let mut client = ApiClient::new("http://localhost:7742".to_string(), None);
                let res = client::urls::login::login(&mut client, "avi_shah".to_string(), "password".to_string()).await;
                // let res = client::urls::login::login(&mut client, "user1234".to_string(), "user1234".to_string()).await;
                println!("{:?}", res);
                let res = client::urls::admin::admin(&mut client).await;
                println!("{:?}", res);
            }
            Err(ReadlineError::Interrupted) => {

            }
            Err(ReadlineError::Eof) => {
                println!("exiting");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    Ok(())
}