use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:5051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "1234".to_owned(),
        to_addr: "23456".to_owned(),
        amount: 209
    });

    let response = client.send_payment(request).await?;

    println!("Response {:?}", response);
    Ok(())
}