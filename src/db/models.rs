use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramUser {
    pub id: Uuid,
    pub telegram_user_id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub is_bot: bool,
    pub is_verified: bool,
    pub is_premium: bool,
    pub language_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramGroup {
    pub id: Uuid,
    pub telegram_chat_id: i64,
    pub chat_type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub member_count: Option<i32>,
    pub is_verified: bool,
    pub is_restricted: bool,
    pub is_scam: bool,
    pub is_fake: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub id: Uuid,
    pub telegram_message_id: i64,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub message_text: Option<String>,
    pub message_type: String,
    pub date: DateTime<Utc>,
    pub edit_date: Option<DateTime<Utc>>,
    pub forward_from_user_id: Option<Uuid>,
    pub forward_from_group_id: Option<Uuid>,
    pub forward_date: Option<DateTime<Utc>>,
    pub reply_to_message_id: Option<i64>,
    pub media_file_id: Option<String>,
    pub media_file_unique_id: Option<String>,
    pub media_file_size: Option<i64>,
    pub media_mime_type: Option<String>,
    pub media_file_name: Option<String>,
    pub location_latitude: Option<f64>,
    pub location_longitude: Option<f64>,
    pub contact_phone_number: Option<String>,
    pub contact_first_name: Option<String>,
    pub contact_last_name: Option<String>,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTelegramUser {
    pub telegram_user_id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub is_bot: bool,
    pub is_verified: bool,
    pub is_premium: bool,
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTelegramGroup {
    pub telegram_chat_id: i64,
    pub chat_type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub member_count: Option<i32>,
    pub is_verified: bool,
    pub is_restricted: bool,
    pub is_scam: bool,
    pub is_fake: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTelegramMessage {
    pub telegram_message_id: i64,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub message_text: Option<String>,
    pub message_type: String,
    pub date: DateTime<Utc>,
    pub edit_date: Option<DateTime<Utc>>,
    pub forward_from_user_id: Option<Uuid>,
    pub forward_from_group_id: Option<Uuid>,
    pub forward_date: Option<DateTime<Utc>>,
    pub reply_to_message_id: Option<i64>,
    pub media_file_id: Option<String>,
    pub media_file_unique_id: Option<String>,
    pub media_file_size: Option<i64>,
    pub media_mime_type: Option<String>,
    pub media_file_name: Option<String>,
    pub location_latitude: Option<f64>,
    pub location_longitude: Option<f64>,
    pub contact_phone_number: Option<String>,
    pub contact_first_name: Option<String>,
    pub contact_last_name: Option<String>,
}


impl TelegramUser {
    pub async fn create(
        pool: &sqlx::PgPool,
        new_user: NewTelegramUser,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO telegram_users 
            (id, telegram_user_id, username, first_name, last_name, phone_number, is_bot, is_verified, is_premium, language_code, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (telegram_user_id) DO UPDATE SET
                username = EXCLUDED.username,
                first_name = EXCLUDED.first_name,
                last_name = EXCLUDED.last_name,
                phone_number = EXCLUDED.phone_number,
                is_bot = EXCLUDED.is_bot,
                is_verified = EXCLUDED.is_verified,
                is_premium = EXCLUDED.is_premium,
                language_code = EXCLUDED.language_code,
                updated_at = NOW()
            RETURNING id, telegram_user_id, username, first_name, last_name, phone_number, is_bot, is_verified, is_premium, language_code, created_at, updated_at
            "#,
            id,
            new_user.telegram_user_id,
            new_user.username,
            new_user.first_name,
            new_user.last_name,
            new_user.phone_number,
            new_user.is_bot,
            new_user.is_verified,
            new_user.is_premium,
            new_user.language_code,
            now,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(TelegramUser {
            id,
            telegram_user_id: new_user.telegram_user_id,
            username: new_user.username,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            phone_number: new_user.phone_number,
            is_bot: new_user.is_bot,
            is_verified: new_user.is_verified,
            is_premium: new_user.is_premium,
            language_code: new_user.language_code,
            created_at: now,
            updated_at: now,
        })
    }
    
    pub async fn find_by_telegram_id(
        pool: &sqlx::PgPool,
        telegram_user_id: i64,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, telegram_user_id, username, first_name, last_name, phone_number, is_bot, is_verified, is_premium, language_code, created_at, updated_at FROM telegram_users WHERE telegram_user_id = $1",
            telegram_user_id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(row.map(|r| TelegramUser {
            id: r.id,
            telegram_user_id: r.telegram_user_id,
            username: r.username,
            first_name: r.first_name,
            last_name: r.last_name,
            phone_number: r.phone_number,
            is_bot: r.is_bot.unwrap_or(false),
            is_verified: r.is_verified.unwrap_or(false),
            is_premium: r.is_premium.unwrap_or(false),
            language_code: r.language_code,
            created_at: r.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: r.updated_at.unwrap_or_else(|| Utc::now()),
        }))
    }
}

impl TelegramGroup {
    pub async fn create(
        pool: &sqlx::PgPool,
        new_group: NewTelegramGroup,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO telegram_groups 
            (id, telegram_chat_id, chat_type, title, username, description, invite_link, member_count, is_verified, is_restricted, is_scam, is_fake, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (telegram_chat_id) DO UPDATE SET
                chat_type = EXCLUDED.chat_type,
                title = EXCLUDED.title,
                username = EXCLUDED.username,
                description = EXCLUDED.description,
                invite_link = EXCLUDED.invite_link,
                member_count = EXCLUDED.member_count,
                is_verified = EXCLUDED.is_verified,
                is_restricted = EXCLUDED.is_restricted,
                is_scam = EXCLUDED.is_scam,
                is_fake = EXCLUDED.is_fake,
                updated_at = NOW()
            RETURNING id, telegram_chat_id, chat_type, title, username, description, invite_link, member_count, is_verified, is_restricted, is_scam, is_fake, created_at, updated_at
            "#,
            id,
            new_group.telegram_chat_id,
            new_group.chat_type,
            new_group.title,
            new_group.username,
            new_group.description,
            new_group.invite_link,
            new_group.member_count,
            new_group.is_verified,
            new_group.is_restricted,
            new_group.is_scam,
            new_group.is_fake,
            now,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(TelegramGroup {
            id,
            telegram_chat_id: new_group.telegram_chat_id,
            chat_type: new_group.chat_type,
            title: new_group.title,
            username: new_group.username,
            description: new_group.description,
            invite_link: new_group.invite_link,
            member_count: new_group.member_count,
            is_verified: new_group.is_verified,
            is_restricted: new_group.is_restricted,
            is_scam: new_group.is_scam,
            is_fake: new_group.is_fake,
            created_at: now,
            updated_at: now,
        })
    }
    
    pub async fn find_by_telegram_id(
        pool: &sqlx::PgPool,
        telegram_chat_id: i64,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, telegram_chat_id, chat_type, title, username, description, invite_link, member_count, is_verified, is_restricted, is_scam, is_fake, created_at, updated_at FROM telegram_groups WHERE telegram_chat_id = $1",
            telegram_chat_id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(row.map(|r| TelegramGroup {
            id: r.id,
            telegram_chat_id: r.telegram_chat_id,
            chat_type: r.chat_type,
            title: r.title,
            username: r.username,
            description: r.description,
            invite_link: r.invite_link,
            member_count: r.member_count,
            is_verified: r.is_verified.unwrap_or(false),
            is_restricted: r.is_restricted.unwrap_or(false),
            is_scam: r.is_scam.unwrap_or(false),
            is_fake: r.is_fake.unwrap_or(false),
            created_at: r.created_at.unwrap_or_else(|| Utc::now()),
            updated_at: r.updated_at.unwrap_or_else(|| Utc::now()),
        }))
    }
}

impl TelegramMessage {
    pub async fn create(
        pool: &sqlx::PgPool,
        new_message: NewTelegramMessage,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO telegram_messages 
            (id, telegram_message_id, user_id, group_id, message_text, message_type, date, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (telegram_message_id, group_id) DO NOTHING
            RETURNING id
            "#,
            id,
            new_message.telegram_message_id,
            new_message.user_id,
            new_message.group_id,
            new_message.message_text,
            new_message.message_type,
            new_message.date,
            now
        )
        .fetch_one(pool)
        .await?;
        
        Ok(TelegramMessage {
            id,
            telegram_message_id: new_message.telegram_message_id,
            user_id: new_message.user_id,
            group_id: new_message.group_id,
            message_text: new_message.message_text,
            message_type: new_message.message_type,
            date: new_message.date,
            edit_date: new_message.edit_date,
            forward_from_user_id: new_message.forward_from_user_id,
            forward_from_group_id: new_message.forward_from_group_id,
            forward_date: new_message.forward_date,
            reply_to_message_id: new_message.reply_to_message_id,
            media_file_id: new_message.media_file_id,
            media_file_unique_id: new_message.media_file_unique_id,
            media_file_size: new_message.media_file_size,
            media_mime_type: new_message.media_mime_type,
            media_file_name: new_message.media_file_name,
            location_latitude: new_message.location_latitude,
            location_longitude: new_message.location_longitude,
            contact_phone_number: new_message.contact_phone_number,
            contact_first_name: new_message.contact_first_name,
            contact_last_name: new_message.contact_last_name,
            created_at: now,
        })
    }
    

}
