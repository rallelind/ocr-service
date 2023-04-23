use aws_sdk_sqs as sqs;
use aws_sdk_textract as textract;
use aws_sdk_textract::types::{Document, S3Object};
use dotenv::dotenv;

async fn ocr_textract_extract(textract_client: &textract::Client, file_name: String) {
    let s3_object = S3Object::builder()
        .bucket("learningio")
        .name(file_name)
        .build();

    let res = textract_client
        .detect_document_text()
        .document(Document::builder().s3_object(s3_object).build())
        .send()
        .await
        .expect("file");

    for text in res.blocks().expect("error reading block") {
        println!("{:?}", text.clone());
    }
}

async fn receive(
    client: &sqs::Client,
    textract_client: &textract::Client,
    queue_url: &String,
) -> Result<(), sqs::Error> {
    loop {
        let rcv_message_output = client.receive_message().queue_url(queue_url).send().await?;

        for message in rcv_message_output.messages.unwrap_or_default() {
            println!("Got the message: {:#?}", message);

            ocr_textract_extract(textract_client, message.body());

            let delete_message_output = client
                .delete_message()
                .queue_url(queue_url)
                .receipt_handle(message.receipt_handle.clone().unwrap())
                .send()
                .await?;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqs::Error> {
    dotenv().ok();

    let queue_url = std::env::var("AWS_SQS_URL").expect("aws sqs url should be provided");

    let config = aws_config::load_from_env().await;
    let textract_client = textract::Client::new(&config);
    let sqs_client = sqs::Client::new(&config);

    receive(&sqs_client, &textract_client, &queue_url).await?;

    Ok(())
}
