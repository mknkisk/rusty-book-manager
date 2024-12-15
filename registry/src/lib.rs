use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::{book::BookRepositoryImpl, health::HealthCheckRepositoryImpl}};
use kernel::repository::health::HealthCheckRepository;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<HealthCheckRepositoryImpl>,
    book_repository: Arc<BookRepositoryImpl>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        // 依存解決。derive-new を使わずにコンストラクタの内部実装を手書きする
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));
        Self {
            health_check_repository,
            book_repository,
        }
    }

    // 依存解決したインスタンスを返すメソッド
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<BookRepositoryImpl> {
        self.book_repository.clone()
    }
}
