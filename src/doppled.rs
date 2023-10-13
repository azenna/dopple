use tonic::{transport::Server, Request, Response, Status};
use anyhow::Result;

use doppled::dopple_server::{Dopple, DoppleServer};
use doppled::{DoppleReply, DoppleRequest};

use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod doppled {
    tonic::include_proto!("dopple");
}

#[derive(Debug, Default)]
pub struct Dopples {}

#[tonic::async_trait]
impl Dopple for Dopples{
    async fn register(
        &self,
        request: Request<DoppleRequest>,
    ) -> Result<Response<DoppleReply>, Status> {

        let dopple = request.into_inner();

        let _ = recursive_copy(&Path::new(&dopple.path), &Path::new(&dopple.qath));

        let reply = doppled::DoppleReply {
            reply: "Doppled your dopple, dopple".into()
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let addr = "[::1]:50051".parse()?;
    let doppler = Dopples::default();

    Server::builder()
        .add_service(DoppleServer::new(doppler))
        .serve(addr)
        .await?;

    Ok(())
}

fn recursive_copy(p: &Path, q: &Path) -> Result<()> {
    if p.is_file() {
        fs::copy(p, q)?;
        Ok(())
    } else if p.is_dir() {
        fs::create_dir(q)?;

        let children = fs::read_dir(p)?;

        for child in children {
            if let Ok(entry) = child {
                recursive_copy(entry.path().as_path(), q.join(entry.file_name()).as_path())?;
            }
        }

        Ok(())
    } else {
        Ok(())
    }
}
