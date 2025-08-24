mod config;
mod db;
mod telegram;

use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use config::Config;
use db::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    info!("F1000 - Sistema de Threat Intel iniciando...");

                   let config = Config::load()?;
    

    let database = Database::new(&config.database.url).await?;
    
    if config.is_telegram_configured() {
        info!("Credenciais do Telegram configuradas");
        
        let client = crate::telegram::TelegramClient::new(
            config.telegram.api_id,
            config.telegram.api_hash,
            config.telegram.phone_number,
            config.telegram.session_path,
        );
        
        info!("Cliente Telegram criado com sucesso!");
        
        match client.connect().await {
            Ok(mut telegram_client) => {
                info!("Conectado ao Telegram!");
                
                match client.sign_in(&mut telegram_client).await {
                    Ok(_) => {
                        info!("✅ Login realizado com sucesso!");
                        info!("🎉 Sistema F1000 conectado ao Telegram!");
                        
                        if let Err(e) = client.save_session(&telegram_client).await {
                            warn!("⚠️ Erro ao salvar sessão: {}", e);
                        }
                        
                        match client.start_listening(&mut telegram_client, &database).await {
                            Ok(_) => info!("📱 Teste de coleta concluído"),
                            Err(e) => warn!("❌ Erro na coleta: {}", e),
                        }
                    },
                    Err(e) => warn!("❌ Erro no login: {}", e),
                }
            },
            Err(e) => warn!("Erro ao conectar: {}", e),
        }
    } else {
        warn!("Credenciais do Telegram não configuradas");
        warn!("Configure as variáveis de ambiente ou crie um arquivo .env");
        info!("Exemplo de configuração em env.example");
    }

    Ok(())
}
