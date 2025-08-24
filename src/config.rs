use std::env;
use dotenv::dotenv;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct TelegramConfig {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub session_path: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        
        let telegram = TelegramConfig {
            api_id: env::var("TELEGRAM_API_ID")
                .unwrap_or_else(|_| "0".to_string())
                .parse()
                .map_err(|_| "TELEGRAM_API_ID deve ser um número válido")?,
            
            api_hash: env::var("TELEGRAM_API_HASH")
                .unwrap_or_else(|_| "".to_string()),
            
            phone_number: env::var("TELEGRAM_PHONE_NUMBER")
                .unwrap_or_else(|_| "".to_string()),
            
            session_path: env::var("TELEGRAM_SESSION_PATH")
                .unwrap_or_else(|_| "session.session".to_string()),
        };
        
        let database = DatabaseConfig {
            url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/f1000".to_string()),
        };
        
        info!("Configuração carregada com sucesso");
        
        if telegram.api_id == 0 {
            warn!("TELEGRAM_API_ID não configurado");
        }
        
        if telegram.api_hash.is_empty() {
            warn!("TELEGRAM_API_HASH não configurado");
        }
        
        if telegram.phone_number.is_empty() {
            warn!("TELEGRAM_PHONE_NUMBER não configurado");
        }
        
        Ok(Config { telegram, database })
    }
    
    pub fn is_telegram_configured(&self) -> bool {
        self.telegram.api_id != 0 
            && !self.telegram.api_hash.is_empty() 
            && !self.telegram.phone_number.is_empty()
    }
}
