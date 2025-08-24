use crate::db::{Database, NewTelegramUser, NewTelegramGroup, NewTelegramMessage, TelegramUser, TelegramGroup, TelegramMessage};
use grammers_client::types::{Message, User, Chat};
use chrono::{DateTime, Utc};
use tracing::{info, warn};
use uuid::Uuid;

pub struct MessageProcessor {
    database: Database,
}

impl MessageProcessor {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
    
    pub async fn process_message(&self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        let pool = self.database.get_pool();
        
        let user_id = if let Some(sender) = message.sender() {
            let telegram_user = self.process_user(sender).await?;
            Some(telegram_user.id)
        } else {
            None
        };
        
        let group_id = {
            let telegram_group = self.process_chat(message.chat()).await?;
            Some(telegram_group.id)
        };
        
        self.process_message_content(message, user_id, group_id).await?;
        
        Ok(())
    }
    
    async fn process_user(&self, user: User) -> Result<TelegramUser, Box<dyn std::error::Error>> {
        let pool = self.database.get_pool();
        
        if let Some(existing_user) = TelegramUser::find_by_telegram_id(pool, user.id()).await? {
            return Ok(existing_user);
        }
        
        let new_user = NewTelegramUser {
            telegram_user_id: user.id(),
            username: user.username().map(|s| s.to_string()),
            first_name: user.first_name().map(|s| s.to_string()),
            last_name: user.last_name().map(|s| s.to_string()),
            phone_number: None, // Não disponível via API
            is_bot: user.is_bot(),
            is_verified: user.is_verified(),
            is_premium: user.is_premium(),
            language_code: user.language_code().map(|s| s.to_string()),
        };
        
        let telegram_user = TelegramUser::create(pool, new_user).await?;
        info!("👤 Usuário processado: {} ({})", 
            telegram_user.first_name.as_deref().unwrap_or("Sem nome"), 
            telegram_user.telegram_user_id);
        
        Ok(telegram_user)
    }
    
    async fn process_chat(&self, chat: Chat) -> Result<TelegramGroup, Box<dyn std::error::Error>> {
        let pool = self.database.get_pool();
        

        if let Some(existing_group) = TelegramGroup::find_by_telegram_id(pool, chat.id()).await? {
            return Ok(existing_group);
        }
        
        let chat_type = match chat {
            Chat::User(_) => "private",
            Chat::Group(_) => "group",
            Chat::Supergroup(_) => "supergroup",
            Chat::Channel(_) => "channel",
        }.to_string();
        
        let (title, username, description, member_count) = match chat {
            Chat::User(user) => (
                Some(format!("{} {}", 
                    user.first_name(), 
                    user.last_name().unwrap_or("")
                ).trim().to_string()),
                user.username().map(|s| s.to_string()),
                None,
                None
            ),
            Chat::Group(group) => (
                Some(group.title().to_string()),
                None,
                None,
                None
            ),
            Chat::Supergroup(supergroup) => (
                Some(supergroup.title().to_string()),
                supergroup.username().map(|s| s.to_string()),
                supergroup.description().map(|s| s.to_string()),
                None // member_count não disponível diretamente
            ),
            Chat::Channel(channel) => (
                Some(channel.title().to_string()),
                channel.username().map(|s| s.to_string()),
                channel.description().map(|s| s.to_string()),
                None
            ),
        };
        
        // Criar novo grupo
        let new_group = NewTelegramGroup {
            telegram_chat_id: chat.id(),
            chat_type,
            title,
            username,
            description,
            invite_link: None, 
            member_count,
            is_verified: false, 
            is_restricted: false, 
            is_scam: false, 
            is_fake: false, 
        };
        
        let telegram_group = TelegramGroup::create(pool, new_group).await?;
        info!("👥 Grupo processado: {} ({})", 
            telegram_group.title.as_deref().unwrap_or("Sem título"), 
            telegram_group.telegram_chat_id);
        
        Ok(telegram_group)
    }
    
    async fn process_message_content(
        &self, 
        message: Message, 
        user_id: Option<Uuid>, 
        group_id: Option<Uuid>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pool = self.database.get_pool();
        
        // Determinar tipo da mensagem
        let message_type = if message.text().is_some() {
            "text"
        } else if message.photo().is_some() {
            "photo"
        } else if message.video().is_some() {
            "video"
        } else if message.audio().is_some() {
            "audio"
        } else if message.document().is_some() {
            "document"
        } else if message.sticker().is_some() {
            "sticker"
        } else if message.location().is_some() {
            "location"
        } else if message.contact().is_some() {
            "contact"
        } else {
            "unknown"
        }.to_string();
        
       
        let message_text = message.text().map(|s| s.to_string());
        
        
        let (media_file_id, media_file_unique_id, media_file_size, media_mime_type, media_file_name) = 
            if let Some(photo) = message.photo() {
                let file = photo.media();
                (
                    Some(file.id().to_string()),
                    Some(file.file_unique_id().to_string()),
                    None, 
                    Some("image/jpeg".to_string()),
                    None
                )
            } else if let Some(video) = message.video() {
                let file = video.media();
                (
                    Some(file.id().to_string()),
                    Some(file.file_unique_id().to_string()),
                    None,
                    Some("video/mp4".to_string()),
                    None
                )
            } else if let Some(document) = message.document() {
                let file = document.media();
                (
                    Some(file.id().to_string()),
                    Some(file.file_unique_id().to_string()),
                    None,
                    document.mime_type().map(|s| s.to_string()),
                    document.file_name().map(|s| s.to_string())
                )
            } else {
                (None, None, None, None, None)
            };
        
       
        let (location_latitude, location_longitude) = if let Some(location) = message.location() {
            (Some(location.latitude()), Some(location.longitude()))
        } else {
            (None, None)
        };
        
       
        let (contact_phone_number, contact_first_name, contact_last_name) = if let Some(contact) = message.contact() {
            (
                Some(contact.phone_number().to_string()),
                Some(contact.first_name().to_string()),
                contact.last_name().map(|s| s.to_string())
            )
        } else {
            (None, None, None)
        };
        
        let new_message = NewTelegramMessage {
            telegram_message_id: message.id(),
            user_id,
            group_id,
            message_text,
            message_type,
            date: DateTime::from_timestamp(message.date() as i64, 0).unwrap_or_else(|| Utc::now()),
            edit_date: message.edit_date().map(|d| DateTime::from_timestamp(d as i64, 0).unwrap_or_else(|| Utc::now())),
            forward_from_user_id: None, // TODO: Implementar forward
            forward_from_group_id: None, // TODO: Implementar forward
            forward_date: None, // TODO: Implementar forward
            reply_to_message_id: message.reply_to().map(|r| r.id()),
            media_file_id,
            media_file_unique_id,
            media_file_size,
            media_mime_type,
            media_file_name,
            location_latitude,
            location_longitude,
            contact_phone_number,
            contact_first_name,
            contact_last_name,
        };
        
        // Salvar mensagem
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
}
