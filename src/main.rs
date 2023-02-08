use jsonrpsee::server::{RpcModule, ServerBuilder};
use log::info;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
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

    let server = ServerBuilder::default()
        .build("127.0.0.1:3030".parse::<SocketAddr>()?)
        .await?;
    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _| Ok("lo"))?;

    let server_addr = server.local_addr()?;
    let handle = server.start(module)?;

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
