# Archon API Reference 🛠️

A API do Archon Daemon segue um padrão REST simplificado para facilitar a integração local e de alta performance.

## Base URL
`http://127.0.0.1:8080`

---

## Endpoints

### 1. Listar Modelos
Retorna a lista de IDs de modelos que estão atualmente carregados ou disponíveis no Daemon.

- **URL:** `/v1/models`
- **Método:** `GET`
- **Sucesso:** `200 OK`
- **Corpo da Resposta:** `["genesis", "oracle"]`

### 2. Geração de Texto (Roteada)
O endpoint principal para inferência. Se o campo `model` for omitido, o roteador inteligente escolherá o modelo baseado no conteúdo do `prompt`.

- **URL:** `/v1/generate`
- **Método:** `POST`
- **Corpo da Requisição (JSON):**
    - `prompt` (string, obrigatório): O texto de entrada.
    - `model` (string, opcional): ID específico do modelo.
    - `max_tokens` (int, opcional): Limite de tokens (default: 64).
    - `temperature` (float, opcional): Criatividade (0.0 a 1.0, default: 0.7).

- **Corpo da Resposta (JSON):**
    - `text` (string): O texto gerado pela rede.
    - `model_used` (string): O ID do modelo que processou a requisição.
    - `tokens_generated` (int): Quantidade de tokens produzidos.

---

## Códigos de Erro
- `404 Not Found`: Modelo solicitado não existe ou não há modelos carregados.
- `400 Bad Request`: JSON malformado ou parâmetros inválidos.
- `500 Internal Server Error`: Falha no motor de inferência (GPU/CPU).

## Performance & Concorrência
O Archon Daemon é construído sobre `Axum` e `Tokio`, suportando centenas de requisições paralelas. O bloqueio de leitura de modelos é feito via `RwLock`, permitindo que múltiplas inferências ocorram simultaneamente no mesmo modelo enquanto ele não estiver sendo atualizado.
