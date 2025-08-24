use sqlx::PgPool;
use tracing::info;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🔌 Conectando ao banco de dados PostgreSQL...");
        
        let pool = PgPool::connect(database_url).await?;
        
        info!("✅ Conectado ao banco de dados com sucesso!");
        
        Ok(Self { pool })
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
