use anyhow::Result;
use clap::Parser;
use dopple::dopple_client::DoppleClient;
use dopple::DoppleRequest;

pub mod dopple {
    tonic::include_proto!("dopple");
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    qath: String,
}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    let mut client = DoppleClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(DoppleRequest {
        path: args.path,
        qath: args.qath,
    });

    let response = client.register(request).await?;
    println!("Response={:?}", response);

    Ok(())
}

