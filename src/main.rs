use aws_sdk_textract as textract;
use aws_sdk_sqs::{Client, Error};
use dotenv::dotenv;

async fn receive(client: &Client, queue_url: &String) -> Result<(), Error> {
    loop {
        let rcv_message_output = client.receive_message().queue_url(queue_url).send().await?;
        
        for message in rcv_message_output.messages.unwrap_or_default() {
            println!("Got the message: {:#?}", message);

            let delete_message_output = client.delete_message()
            .queue_url(queue_url)
            .receipt_handle(message.receipt_handle.clone().unwrap())
            .send()
            .await?;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let queue_url = std::env::var("AWS_SQS_URL").expect("aws sqs url should be provided");

    let config = aws_config::load_from_env().await;
    let textract_client = textract::Client::new(&config);
    let sqs_client = Client::new(&config);

    receive(&sqs_client, &queue_url).await?;


    Ok(())
}
