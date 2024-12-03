use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<HealthCheckRepositoryImpl>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        // 依存解決。derive-new を使わずにコンストラクタの内部実装を手書きする
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool));
        Self {
            health_check_repository,
        }
    }

    // 依存解決したインスタンスを返すメソッド
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}
