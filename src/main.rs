mod telegram;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    info!("F1000 - Sistema de Threat Intel iniciando...");

    info!("Versão grammers-client: {}", env!("CARGO_PKG_VERSION"));
    info!("grammers-tl-types disponível");
    info!("Setup básico funcionando!");

    let _client = telegram::TelegramClient::new(
        12345,
        "your_api_hash_here".to_string(),
        "+1234567890".to_string(),
        "session.session".to_string(),
    );

    info!("Cliente Telegram criado com sucesso!");
    info!("Teste de conexão pulado - necessário credenciais reais");

    Ok(())
}
