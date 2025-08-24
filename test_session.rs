use grammers_session::Session;
use std::path::Path;

fn main() {
    let path = Path::new("test_session.session");
    
    println!("Criando sessão...");
    let session = Session::new();
    
    println!("Salvando sessão...");
    match session.save_to_file(path) {
        Ok(_) => println!("Sessão salva com sucesso!"),
        Err(e) => println!("Erro ao salvar: {}", e),
    }
    
    println!("Carregando sessão...");
    match Session::load_file(path) {
        Ok(_) => println!("Sessão carregada com sucesso!"),
        Err(e) => println!("Erro ao carregar: {}", e),
    }
}
