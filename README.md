# F1000 - Sistema de Threat Intel

Sistema de Threat Intelligence conectado ao Telegram Client (MTProto) desenvolvido em Rust.

## 🚀 Configuração

### 1. Instalar Dependências
```bash
cargo build
```

### 2. Configurar Credenciais do Telegram

Copie o arquivo de exemplo:
```bash
cp env.example .env
```

Edite o arquivo `.env` com suas credenciais:
```env
TELEGRAM_API_ID=12345
TELEGRAM_API_HASH=your_api_hash_here
TELEGRAM_PHONE_NUMBER=+5511999999999
TELEGRAM_SESSION_PATH=session.session
```

### 3. Obter Credenciais do Telegram

1. Acesse https://my.telegram.org
2. Faça login com seu número de telefone
3. Vá em "API development tools"
4. Crie um novo aplicativo
5. Copie o `api_id` e `api_hash`

## 🏃‍♂️ Executar

```bash
cargo run
```

## 📁 Estrutura do Projeto

```
src/
├── main.rs          # Ponto de entrada
├── config.rs        # Configuração (.env)
└── telegram.rs      # Cliente Telegram
```

## 🔧 Funcionalidades

- ✅ Cliente Telegram MTProto
- ✅ Gerenciamento de sessão
- ✅ Configuração via variáveis de ambiente
- ✅ Logs estruturados
- ✅ Tratamento de erros

## 🛡️ Segurança

- Credenciais armazenadas em `.env` (não commitado)
- Sessão do Telegram criptografada
- Logs sem informações sensíveis

## 📝 Próximos Passos

- [ ] Processamento de mensagens
- [ ] Sistema de keywords
- [ ] Detecção de ameaças
- [ ] API REST
- [ ] Banco de dados
