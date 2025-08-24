use grammers_client::{Client, Config};
use grammers_session::Session;
use tracing::{info, warn};
use std::io::{self, Write};
use grammers_client::SignInError;
use crate::db::{Database, NewTelegramUser, NewTelegramGroup, NewTelegramMessage, TelegramUser, TelegramGroup, TelegramMessage};
use grammers_client::types::{Message, Chat};
use chrono::Utc;

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
    
    pub async fn save_session(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from(&self.session_path);
        info!("💾 Salvando sessão em: {}", path.display());
        
        client.session().save_to_file(&path)
            .map_err(|e| -> Box<dyn std::error::Error> { format!("Falha ao salvar sessão: {}", e).into() })?;
        
        info!("✅ Sessão salva com sucesso!");
        Ok(())
    }

    fn load_or_create_session(&self) -> Result<Session, Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from(&self.session_path);
        
        if path.exists() {
            info!("📂 Carregando sessão existente de: {}", path.display());
            match Session::load_file(&path) {
                Ok(session) => {
                    info!("✅ Sessão carregada com sucesso!");
                    Ok(session)
                },
                Err(e) => {
                    warn!("⚠️ Erro ao carregar sessão: {}", e);
                    warn!("🔄 Criando nova sessão...");
                    self.create_new_session(&path)
                }
            }
        } else {
            info!("🆕 Criando nova sessão em: {}", path.display());
            self.create_new_session(&path)
        }
    }
    
    fn create_new_session(&self, path: &std::path::Path) -> Result<Session, Box<dyn std::error::Error>> {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        
        info!("🔍 Debug: Path original: {}", path.display());
        info!("🔍 Debug: Path absoluto: {}", absolute_path.display());
        
        if let Some(parent) = absolute_path.parent() {
            info!("🔍 Debug: Diretório pai: {}", parent.display());
            if !parent.exists() {
                info!("📁 Criando diretório pai: {}", parent.display());
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Falha ao criar diretório {}: {}", parent.display(), e))?;
            } else {
                info!("✅ Diretório pai já existe: {}", parent.display());
            }
        } else {
            info!("🔍 Debug: Sem diretório pai (arquivo na raiz)");
        }
        
        info!("📄 Criando arquivo vazio primeiro...");
        std::fs::write(&absolute_path, "")
            .map_err(|e| format!("Falha ao criar arquivo {}: {}", absolute_path.display(), e))?;
        info!("✅ Arquivo criado: {}", absolute_path.display());
        
        let session = Session::new();
        
        if let Some(parent) = absolute_path.parent() {
            let test_file = parent.join("test_write.tmp");
            match std::fs::write(&test_file, "test") {
                Ok(_) => {
                    std::fs::remove_file(&test_file).ok();
                    info!("✅ Permissão de escrita OK no diretório: {}", parent.display());
                },
                Err(e) => {
                    warn!("⚠️ Sem permissão de escrita em {}: {}", parent.display(), e);
                }
            }
        }
        
        info!("💾 Tentando salvar sessão em: {}", absolute_path.display());
        session.save_to_file(&absolute_path)
            .map_err(|e| format!("Falha ao salvar sessão em {}: {}", absolute_path.display(), e))?;
        
        info!("✅ Nova sessão criada e salva!");
        Ok(session)
    }

    pub async fn sign_in(&self, client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
        info!("Iniciando processo de login...");
        
        if client.is_authorized().await? {
            info!("Já está autenticado!");
            return Ok(());
        }
        
        let login_token = client.request_login_code(&self.phone_number).await?;
        info!("Código enviado para {}", self.phone_number);
        
        println!("\n🔐 Digite o código de verificação que você recebeu no SMS:");
        print!("Código: ");
        io::stdout().flush()?;
        
        let mut code = String::new();
        io::stdin().read_line(&mut code)?;
        let code = code.trim();
        
        info!("Tentando fazer login com o código...");
        
        match client.sign_in(&login_token, code).await { 
            Err(SignInError::PasswordRequired(password_token) ) => {
                println!("\n🔐 Digite sua senha 2FA:");
                print!("Senha: ");
                io::stdout().flush()?;
                
                let mut password = String::new();
                io::stdin().read_line(&mut password)?;
                let password = password.trim();
        
                client
                    .check_password(password_token, password)
                    .await.unwrap();
                Ok(())
            }
            Ok(_) => {
                info!("✅ Login realizado com sucesso!");
                Ok(())
            },
            Err(e) => {
                warn!("❌ Erro no login: {}", e);
                Err(e.into())
            }
        }
    }


    pub async fn start_listening(&self, client: &mut Client, database: &Database) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Iniciando coleta de mensagens em tempo real...");
        info!("📱 Aguardando mensagens... (Ctrl+C para parar)");
        
        loop {
            let update_result = tokio::time::timeout(
                tokio::time::Duration::from_secs(10),
                client.next_update()
            ).await;
            
            match update_result {
                Ok(Ok(update)) => {
                    if let grammers_client::Update::NewMessage(message) = update {
                        if let Err(e) = self.save_message_simple(message, database).await {
                            warn!("❌ Erro ao salvar mensagem: {}", e);
                        }
                    }
                },
                Ok(Err(e)) => {
                    warn!("❌ Erro ao receber update: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                },
                Err(_) => {
                }
            }
        }
    }
    
    async fn save_message_simple(&self, message: Message, database: &Database) -> Result<(), Box<dyn std::error::Error>> {
        let pool = database.get_pool();
        
        let user_id = match self.get_message_sender_info(&message).await {
            Ok((telegram_user_id, first_name, last_name, username, phone, is_bot, is_verified, is_premium, language_code)) => {
                match self.process_user_from_info(telegram_user_id, first_name, last_name, username, phone, is_bot, is_verified, is_premium, language_code, pool).await {
                    Ok(user) => {
                        info!("👤 Usuário processado: {} (ID: {})", 
                            user.first_name.as_deref().unwrap_or("Sem nome"), 
                            user.telegram_user_id);
                        Some(user.id)
                    },
                    Err(e) => {
                        warn!("❌ Erro ao processar usuário: {}", e);
                        None
                    }
                }
            },
            Err(_) => None,
        };
        
        let group_id = {
            let chat = message.chat();
            match self.process_chat_simple(chat, pool).await {
                Ok(group) => {
                    info!("👥 Grupo processado: {} (ID: {})", 
                        group.title.as_deref().unwrap_or("Sem título"), 
                        group.telegram_chat_id);
                    Some(group.id)
                },
                Err(e) => {
                    warn!("❌ Erro ao processar grupo: {}", e);
                    None
                }
            }
        };
        
        let message_text = message.text();
        let message_type = if !message_text.is_empty() { "text" } else { "unknown" };
        
        let new_message = NewTelegramMessage {
            telegram_message_id: message.id() as i64,
            user_id,
            group_id,
            message_text: if !message_text.is_empty() { Some(message_text.to_string()) } else { None },
            message_type: message_type.to_string(),
            date: Utc::now(),
            edit_date: None,
            forward_from_user_id: None,
            forward_from_group_id: None,
            forward_date: None,
            reply_to_message_id: None,
            media_file_id: None,
            media_file_unique_id: None,
            media_file_size: None,
            media_mime_type: None,
            media_file_name: None,
            location_latitude: None,
            location_longitude: None,
            contact_phone_number: None,
            contact_first_name: None,
            contact_last_name: None,
        };
        
        match TelegramMessage::create(pool, new_message).await {
            Ok(_) => {
                info!("💾 Mensagem salva: ID {} (tipo: {})", 
                    message.id(), message_type);
            },
            Err(e) => {
                if e.to_string().contains("duplicate key") {
                    info!("🔄 Mensagem duplicada ignorada: ID {}", message.id());
                } else {
                    warn!("❌ Erro ao salvar mensagem {}: {}", message.id(), e);
                }
            }
        }
        
        Ok(())
    }
    

    
    async fn process_chat_simple(&self, chat: Chat, pool: &sqlx::PgPool) -> Result<TelegramGroup, Box<dyn std::error::Error>> {
        if let Some(existing_group) = TelegramGroup::find_by_telegram_id(pool, chat.id()).await? {
            return Ok(existing_group);
        }
        
        let chat_type = match chat {
            Chat::User(_) => "private",
            Chat::Group(_) => "group",
            Chat::Channel(_) => "channel",
        }.to_string();
        
        let (title, username, description) = match chat {
            Chat::User(ref user) => (
                Some(format!("{} {}", 
                    user.first_name(), 
                    user.last_name().unwrap_or("")
                ).trim().to_string()),
                user.username().map(|s| s.to_string()),
                None
            ),
            Chat::Group(ref group) => (
                Some(group.title().to_string()),
                None,
                None
            ),
            Chat::Channel(ref channel) => (
                Some(channel.title().to_string()),
                channel.username().map(|s| s.to_string()),
                None
            ),
        };
        
        let new_group = NewTelegramGroup {
            telegram_chat_id: chat.id(),
            chat_type,
            title,
            username,
            description,
            invite_link: None,
            member_count: None,
            is_verified: false,
            is_restricted: false,
            is_scam: false,
            is_fake: false,
        };
        
        TelegramGroup::create(pool, new_group).await
            .map_err(|e| format!("Erro ao criar grupo: {}", e).into())
    }
    
    async fn get_message_sender_info(&self, message: &Message) -> Result<(i64, Option<String>, Option<String>, Option<String>, Option<String>, bool, bool, bool, Option<String>), Box<dyn std::error::Error>> {
        if let Some(sender) = message.sender() {
            let user_id = sender.id();
            
            let (first_name, last_name, username, phone, is_bot, is_verified, is_premium, language_code) = match sender {
                Chat::User(user) => (
                    Some(user.first_name().to_string()),
                    user.last_name().map(|s| s.to_string()),
                    user.username().map(|s| s.to_string()),
                    user.phone().map(|s| s.to_string()),
                    user.is_bot(),
                    user.verified(),
                    user.raw.premium,
                    user.lang_code().map(|s| s.to_string())
                ),
                _ => (
                    None,
                    None,
                    None,
                    None,
                    false,
                    false,
                    false,
                    None
                )
            };
            
            Ok((user_id, first_name, last_name, username, phone, is_bot, is_verified, is_premium, language_code))
        } else {
            Err("Mensagem sem remetente".into())
        }
    }
    
    async fn process_user_from_info(
        &self, 
        telegram_user_id: i64, 
        first_name: Option<String>, 
        last_name: Option<String>, 
        username: Option<String>, 
        phone: Option<String>,
        is_bot: bool,
        is_verified: bool,
        is_premium: bool,
        language_code: Option<String>,
        pool: &sqlx::PgPool
    ) -> Result<TelegramUser, Box<dyn std::error::Error>> {
        if let Some(existing_user) = TelegramUser::find_by_telegram_id(pool, telegram_user_id).await? {
            return Ok(existing_user);
        }
        
        let new_user = NewTelegramUser {
            telegram_user_id,
            username,
            first_name,
            last_name,
            phone_number: phone,
            is_bot,
            is_verified,
            is_premium,
            language_code,
        };
        
        TelegramUser::create(pool, new_user).await
            .map_err(|e| format!("Erro ao criar usuário: {}", e).into())
    }

    

}


