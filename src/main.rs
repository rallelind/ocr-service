use aws_sdk_textract as textract;

// Resources for learning: 
// https://medium.com/intelliconnect-engineering/uploading-files-to-aws-s3-using-axum-a-rust-framework-c96b1c774dfc
// https://www.youtube.com/watch?v=DLmyW58egg4

#[tokio::main]
async fn main() -> Result<(), textract::Error> {
    let config = aws_config::load_from_env().await;
    let client = textract::Client::new(&config);

    // ... make some calls with the client
    

    Ok(())
}
