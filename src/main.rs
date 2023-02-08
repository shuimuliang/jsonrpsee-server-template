use jsonrpsee::server::ServerBuilder;
use jsonrpsee_server_template::{PostRpcServer, RpcImpl};
use log::info;
use sea_orm::Database;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use std::env;
use std::net::SocketAddr;
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    let _ = TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let server = ServerBuilder::default()
        .build(server_url.parse::<SocketAddr>()?)
        .await?;

    // create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();

    let rpc_impl = RpcImpl::new(conn);
    let server_addr = server.local_addr()?;
    let handle = server.start(rpc_impl.into_rpc()).unwrap();

    info!("starting listening {}", server_addr);
    let mut sig_int = signal(SignalKind::interrupt()).unwrap();
    let mut sig_term = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sig_int.recv() => info!("receive SIGINT"),
        _ = sig_term.recv() => info!("receive SIGTERM"),
        _ = ctrl_c() => info!("receive Ctrl C"),
    }
    handle.stop().unwrap();
    info!("Shutdown program");

    Ok(())
}

fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
