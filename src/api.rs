use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use serde_json::json;

const PRE_PROMPT: &str = "Você é um assistente de terminal que ajuda a executar comandos Linux. \
Ao responder perguntas, sempre comece com um comando no formato '/cmd:' se for um comando a ser executado. \
Aqui estão algumas diretrizes a seguir:\n\n\
- Para perguntas sobre execução de comandos, forneça a resposta no formato '/cmd: [comando]'.\n\
- Para outras perguntas, forneça a resposta normalmente.";

pub fn send_to_llm(query: &str) -> Result<String, Box<dyn Error>> {
    // Monta o prompt completo
    let full_prompt = format!("{}\n\nEis o prompt: {}", PRE_PROMPT, query);

    // Cria um cliente para fazer requisições HTTP
    let client = Client::new();

    // URL da API com a chave de API incluída
    let api_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=AIzaSyDK16haD0TT8HTXCutG-jisKims9bmYrW4";

    // Envia a requisição POST para a API
    let response = client.post(api_url)
        .header("Content-Type", "application/json")
        .json(&json!({
            "contents": [
                {
                    "parts": [
                        { "text": full_prompt } 
                    ]
                }
            ]
        }))
        .send()?;

    // Lê o texto da resposta
    let response_text = response.text()?;
    
    // Analisa o JSON da resposta
    let parsed: Value = serde_json::from_str(&response_text)?;

    // Extrai o texto da resposta
    Ok(parsed["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("Erro na resposta da LLM")
        .to_string())
}
