use grammers_client::{Client, Config};
use grammers_session::Session;
use tracing::{info, warn};
use std::path::Path;

pub struct TelegramClient {
    api_id: i32,
    api_hash: String,
    phone_number: String,
    session_path: String,
}

impl TelegramClient {
    pub fn new(api_id: i32, api_hash: String, phone_number: String, session_path: String) -> Self {
        Self {
            api_id,
            api_hash,
            phone_number,
            session_path,
        }
    }

    pub async fn connect(&self) -> Result<Client, Box<dyn std::error::Error>> {
        info!("Conectando ao Telegram...");
        
        let session = self.load_or_create_session()?;
        
        let config = Config {
            api_id: self.api_id,
            api_hash: self.api_hash.clone(),
            session,
            params: Default::default(),
        };
        
        let client = Client::connect(config).await?;
        
        info!("Conectado ao Telegram com sucesso!");
        Ok(client)
    }

    fn load_or_create_session(&self) -> Result<Session, Box<dyn std::error::Error>> {
        let path = Path::new(&self.session_path);
        
        if path.exists() {
            info!("Carregando sessão existente de {}", self.session_path);
            Session::load_file(path)
                .map_err(|e| format!("Falha ao carregar sessão: {}", e).into())
        } else {
            info!("Criando nova sessão em {}", self.session_path);
            
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Falha ao criar diretório: {}", e))?;
            }
            
            let session = Session::new();
            session.save_to_file(path)
                .map_err(|e| format!("Falha ao salvar sessão: {}", e))?;
            Ok(session)
        }
    }

    pub async fn sign_in(&self, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        info!("Iniciando processo de login...");
        
        if client.is_authorized().await? {
            info!("Já está autenticado!");
            return Ok(());
        }
        
        let _phone_code = client.request_login_code(&self.phone_number).await?;
        info!("Código enviado para {}", self.phone_number);
        
        warn!("Digite o código de verificação manualmente");
        warn!("Código de verificação recebido com sucesso");
        
        Ok(())
    }
}


