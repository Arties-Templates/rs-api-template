mod server;

use std::{net::SocketAddr, error::Error};

use server::routes;
use tracing::warn;
use warp::{self, Filter};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let addr = format!("{}:{}", env::var("PORT")?, env::var("HOST")?).parse::<SocketAddr>()?;

    let index = warp::get().and_then(routes::index);
    let routes = warp::any().and(index);

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        warn!("Received Termination Signal...");
        std::process::exit(0)
    });


    warp::serve(routes).run(addr).await;

    Ok(())
}
