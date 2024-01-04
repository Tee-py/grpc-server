use std::fmt::format;
use tonic::{transport::Server, Request, Response, Status};
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitCoinService {}

#[tonic::async_trait]
impl Bitcoin for BitCoinService {
    async fn send_payment(&self, request: Request<BtcPaymentRequest>) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let response = BtcPaymentResponse {
            successful: true,
            message: format!("Send {}BTC to {}.", req.amount, req.to_addr)
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5001".parse()?;
    let btc_service = BitCoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}
