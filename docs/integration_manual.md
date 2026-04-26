# Archon Integration Manual 🏛️

Este manual descreve como integrar softwares de terceiros com o ecossistema **Archon**, utilizando o Daemon local e o SDK oficial.

## 1. O Conceito
O Archon atua como uma ponte inteligente entre sua aplicação e os modelos comprimidos do **Arkheion Forge**. Ele gerencia o ciclo de vida dos modelos na VRAM e roteia requisições automaticamente para o melhor "especialista" local.

## 2. Configurando o Daemon
O Daemon deve estar rodando em background para processar as requisições.

```bash
# Iniciar o daemon
cargo run --bin archon-daemon
```

O servidor ficará disponível por padrão em `http://127.0.0.1:8080`.

## 3. Integrando via Rust SDK
Adicione o `archon-sdk` ao seu `Cargo.toml`:

```toml
[dependencies]
archon-sdk = { path = "path/to/crates/archon-sdk" }
tokio = { version = "1", features = ["full"] }
```

### Exemplo de Uso:
```rust
use archon_sdk::ArchonClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializa o cliente (conecta ao daemon local)
    let client = ArchonClient::default();

    // Lista modelos disponíveis
    let models = client.list_models().await?;
    println!("Modelos ativos: {:?}", models);

    // Geração simples (O Archon decide qual modelo usar baseado no prompt)
    let response = client.generate("fn calculate_hash(data: &[u8]) -> u64 {").await?;
    
    println!("Modelo usado: {}", response.model_used);
    println!("Resultado:\n{}", response.text);

    Ok(())
}
```

## 4. Integração via API REST (Linguagens Externas)
Para Python, JavaScript ou C++, você pode usar requisições HTTP padrão.

### Endpoint: `POST /v1/generate`
**Payload:**
```json
{
  "prompt": "Explique o conceito de gravidade.",
  "max_tokens": 100,
  "temperature": 0.7
}
```

**Resposta:**
```json
{
  "text": "...texto gerado...",
  "model_used": "genesis",
  "tokens_generated": 45
}
```

---
**Nota:** O Archon Daemon utiliza compressão HTCV4, permitindo que múltiplos modelos residam na GPU simultaneamente com um impacto mínimo de memória.
