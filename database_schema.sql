CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE telegram_users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    telegram_user_id BIGINT UNIQUE NOT NULL,
    username VARCHAR(255),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    phone_number VARCHAR(50),
    is_bot BOOLEAN DEFAULT FALSE,
    is_verified BOOLEAN DEFAULT FALSE,
    is_premium BOOLEAN DEFAULT FALSE,
    language_code VARCHAR(10),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE telegram_groups (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    telegram_chat_id BIGINT UNIQUE NOT NULL,
    chat_type VARCHAR(50) NOT NULL,
    title VARCHAR(255),
    username VARCHAR(255),
    description TEXT,
    invite_link VARCHAR(255),
    member_count INTEGER,
    is_verified BOOLEAN DEFAULT FALSE,
    is_restricted BOOLEAN DEFAULT FALSE,
    is_scam BOOLEAN DEFAULT FALSE,
    is_fake BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE telegram_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    telegram_message_id BIGINT NOT NULL,
    user_id UUID REFERENCES telegram_users(id),
    group_id UUID REFERENCES telegram_groups(id),
    message_text TEXT,
    message_type VARCHAR(50) NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    edit_date TIMESTAMP WITH TIME ZONE,
    forward_from_user_id UUID REFERENCES telegram_users(id),
    forward_from_group_id UUID REFERENCES telegram_groups(id),
    forward_date TIMESTAMP WITH TIME ZONE,
    reply_to_message_id BIGINT,
    media_file_id VARCHAR(255),
    media_file_unique_id VARCHAR(255),
    media_file_size BIGINT,
    media_mime_type VARCHAR(100),
    media_file_name VARCHAR(255),
    location_latitude DECIMAL(10, 8),
    location_longitude DECIMAL(11, 8),
    contact_phone_number VARCHAR(50),
    contact_first_name VARCHAR(255),
    contact_last_name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(telegram_message_id, group_id)
);

CREATE INDEX idx_telegram_users_telegram_id ON telegram_users(telegram_user_id);
CREATE INDEX idx_telegram_users_username ON telegram_users(username);

CREATE INDEX idx_telegram_groups_telegram_id ON telegram_groups(telegram_chat_id);
CREATE INDEX idx_telegram_groups_type ON telegram_groups(chat_type);
CREATE INDEX idx_telegram_groups_title ON telegram_groups(title);

CREATE INDEX idx_telegram_messages_user_id ON telegram_messages(user_id);
CREATE INDEX idx_telegram_messages_group_id ON telegram_messages(group_id);
CREATE INDEX idx_telegram_messages_date ON telegram_messages(date);
CREATE INDEX idx_telegram_messages_type ON telegram_messages(message_type);
CREATE INDEX idx_telegram_messages_text ON telegram_messages USING gin(to_tsvector('portuguese', message_text));

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_telegram_users_updated_at 
    BEFORE UPDATE ON telegram_users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_telegram_groups_updated_at 
    BEFORE UPDATE ON telegram_groups 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
