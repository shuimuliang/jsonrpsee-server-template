use anyhow::anyhow;
use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::CallError;
use sea_orm::DatabaseConnection;
use std::fmt::Display;

mod entity;
mod query;

pub struct RpcImpl {
    pub conn: DatabaseConnection,
}

impl RpcImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[rpc(server)]
trait PostRpc {
    #[method(name = "say_hello")]
    async fn say_hello(&self) -> RpcResult<String>;

    #[method(name = "Slot.List")]
    async fn list(&self) -> RpcResult<Vec<entity::Model>>;
}

#[async_trait]
impl PostRpcServer for RpcImpl {
    async fn say_hello(&self) -> RpcResult<String> {
        Ok("Hello World".to_owned())
    }
    async fn list(&self) -> RpcResult<Vec<entity::Model>> {
        query::Query::find_all(&self.conn)
            .await
            .internal_call_error()
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
