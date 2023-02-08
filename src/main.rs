use anyhow::anyhow;
use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::CallError;
use log::info;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use std::fmt::Display;
use std::net::SocketAddr;
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};

struct RpcImpl {
    // conn: DatabaseConnection,
}

#[rpc(server)]
trait PostRpc {
    #[method(name = "say_hello")]
    async fn say_hello(
        &self,
    ) -> RpcResult<String>;

    #[method(name = "Post.List")]
    async fn list(
        &self,
    ) -> RpcResult<i32>;
}

#[async_trait]
impl PostRpcServer for RpcImpl {
    async fn say_hello(
        &self,
    ) -> RpcResult<String> {
        Ok("Hello World".to_owned())
    }
    async fn list(
        &self,
    ) -> RpcResult<i32> {
        Ok(1)
    }
}

trait IntoJsonRpcResult<T> {
    fn internal_call_error(self) -> RpcResult<T>;
}

impl<T, E> IntoJsonRpcResult<T> for Result<T, E>
    where
        E: Display,
{
    fn internal_call_error(self) -> RpcResult<T> {
        self.map_err(|e| jsonrpsee::core::Error::Call(CallError::Failed(anyhow!("{}", e))))
    }
}

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

    let rpc_impl = RpcImpl{};
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
