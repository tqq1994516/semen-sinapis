#![feature(impl_trait_in_assoc_type)]
#![feature(async_fn_in_trait)]
use volo_grpc::server::{Server, ServiceBuilder};
use std::net::SocketAddr;

use volo_gen::person_center::UserServer;
use layer::postgres::PostgresqlLayer;

mod controller;
mod service;
mod helper;

#[volo::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = std::env::var("ADDR")?.parse()?;
    let addr = volo::net::Address::from(addr);

    let user_service = controller::UserService {};

    Server::new()
        .layer_front(PostgresqlLayer)
        .add_service(ServiceBuilder::new(UserServer::new(user_service)).build())
        .run(addr)
        .await?;
    Ok(())
}
