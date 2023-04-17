use aws_sdk_textract as textract;

#[tokio::main]
async fn main() -> Result<(), textract::Error> {
    let config = aws_config::load_from_env().await;
    let client = textract::Client::new(&config);

    // ... make some calls with the client
    

    Ok(())
}
