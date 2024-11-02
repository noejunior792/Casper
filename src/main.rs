// main.rs
mod api;
mod command_executor;
use std::io::{self, Write};
use api::send_to_llm;
use command_executor::{execute_command, validate_command};

fn main() {
    loop {
        // Captura entrada do usuário
        let mut input = String::new();
        print!("Digite sua pergunta ou comando (/cmd: para comandos): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
        let input = input.trim();

        // Executa comando direto se começar com /cmd:
        if input.starts_with("/cmd:") {
            let command = input.trim_start_matches("/cmd:").trim();
            println!("Executando comando: {}", command);
            match validate_command(command) {
                Ok(_) => match execute_command(command) {
                    Ok(output) => println!("Resultado:\n{}", output),
                    Err(e) => println!("Erro ao executar comando: {}", e),
                },
                Err(e) => println!("Comando inválido ou perigoso: {}", e),
            }
        } else {
            // Processa como pergunta via LLM
            match send_to_llm(&input) {
                Ok(response) => {
                    println!("Resposta da LLM: {}", response);
                    // Se a resposta da LLM contém um comando, executa-o
                    if response.starts_with("/cmd:") {
                        let command = response.trim_start_matches("/cmd:").trim();
                        println!("\nExecutando o comando sugerido: {}", command);
                        match validate_command(command) {
                            Ok(_) => match execute_command(command) {
                                Ok(output) => println!("Resultado:\n{}", output),
                                Err(e) => println!("Erro ao executar comando: {}", e),
                            },
                            Err(e) => println!("Comando inválido ou perigoso: {}", e),
                        }
                    }
                }
                Err(e) => println!("Erro de API: {}", e),
            }
        }
        
        println!("Deseja fazer outra pergunta? (s/n): ");
        let mut continuar = String::new();
        io::stdin().read_line(&mut continuar).expect("Falha ao ler a linha");
        if continuar.trim().to_lowercase() != "s" {
            break;
        }
    }
}