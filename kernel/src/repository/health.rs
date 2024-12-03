use async_trait::async_trait;

#[async_trait]
// amux を使用するために Send と Sync を満たす必要がある
// Send と Sync は marker trait
// marker trait はコンパイラが特別扱いするトレイト
pub trait HealthCheckRepository: Send + Sync {
    async fn check_db(&self) -> bool;
}
