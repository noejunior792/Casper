use std::process::{Command, Output};
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    InvalidCommand,
    ExecutionError(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidCommand => write!(f, "Comando inválido."),
            CommandError::ExecutionError(msg) => write!(f, "Erro na execução do comando: {}", msg),
        }
    }
}


pub fn validate_command(command: &str) -> Result<(), CommandError> {
    // Definir lista de comandos bloqueados ou considerados perigosos
    let forbidden_commands = vec!["rm -rf", "shutdown", "reboot"];
    for forbidden in forbidden_commands {
        if command.contains(forbidden) {
            return Err(CommandError::InvalidCommand);
        }
    }
    Ok(())
}

pub fn execute_command(command: &str) -> Result<String, CommandError> {
    let output: Output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(CommandError::ExecutionError(String::from_utf8_lossy(&output.stderr).into_owned()))
    }
}
