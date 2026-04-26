# CLEANUP MANIFEST — Arkheion Forge Standalone

> **Objetivo**: Remover TODA referência ao Arkheion Forge, eliminar redundâncias,  
> remover módulos de domínio de vendas, e refinar para o propósito único:  
> **Autonomous AI Coding Engine**.
> 
> **Regra**: Só documentar. A execução será feita fora do workspace original.
> 
> **Data**: 2026-04-23

---

## Sumário de Escala

| Categoria | Quantidade | Ação |
|-----------|-----------|------|
| Módulos Python core (manter) | **148 arquivos** | Refinar, remover `tenant_id` |
| Módulos sales/business (remover) | **77 arquivos** | Deletar completamente |
| Duplicatas `backend/` (remover) | **95 arquivos** | Deletar completamente |
| Top-level sales-specific (remover) | **35 arquivos** | Deletar |
| Arquivos com `tenant_id` | **125 arquivos** | Remover conceito multi-tenant |
| Arquivos com import `from src.` | **11 arquivos** | Corrigir imports |
| Arquivos com import `from backend.` | **4 arquivos** | Corrigir imports |
| Rust crates com "korus" | **5 arquivos** | Renomear/limpar referências |
| Docs com "korus" | **12 arquivos** | Reescrever referências |

---

## FASE 1: REMOÇÃO — Cortar o Morto

### 1.1 Deletar Diretório `ai-core/backend/` (95 arquivos)

**Razão**: É uma cópia duplicada inteira do backend original dentro do ai-core. Redundância total.

```bash
rm -rf arkheion-forge/ai-core/backend/
```

---

### 1.2 Deletar Módulos de Domínio Sales/Business (77 arquivos)

Estes módulos são **100% domínio de vendas** e não têm utilidade para um coding engine:

#### Diretórios inteiros para deletar:

| Diretório | Arquivos | Razão |
|-----------|---------|-------|
| `ai-core/coach/` | 8 | Sales coaching — irrelevante |
| `ai-core/seller_bot/` | 5 | Bot vendedor WhatsApp |
| `ai-core/financial/` | 16 | Prospero (educação financeira gamificada) |
| `ai-core/marketing/` | 13 | Google Ads, Instagram, TikTok, YouTube AI |
| `ai-core/regional_brasil/` | 9 | NLP regional brasileiro (27 estados) |
| `ai-core/video_production/` | 20 | Pipeline de vídeo IA (6 estágios) |
| `ai-core/image_gen/` | 4 | Geração de imagens IA |
| `ai-core/orchestration/` | 2 | Conversão + agenda diária (sales) |

```bash
rm -rf arkheion-forge/ai-core/{coach,seller_bot,financial,marketing}
rm -rf arkheion-forge/ai-core/{regional_brasil,video_production,image_gen,orchestration}
```

#### Arquivos top-level para deletar (35):

| Arquivo | Razão |
|---------|-------|
| `ab_testing.py` | A/B testing de vendas |
| `auto_ab_testing.py` | A/B testing automático de vendas |
| `ab_testing_models.py` | Modelos de A/B de vendas |
| `ab_handler_mixin.py` | Mixin de A/B handler |
| `calibration_engine.py` | Calibração de funil de vendas |
| `coach_base.py` | Base de coach de vendas |
| `humanizer.py` | Humanização de mensagens WhatsApp |
| `kpi_collector.py` | KPIs de vendas |
| `objection_cache.py` | Cache de objeções de vendas |
| `sales_response_boundary.py` | Limite de resposta de vendas |
| `sentiment_onnx.py` | ONNX sentiment (vendas) |
| `whatsapp_formatter.py` | Formatador WhatsApp |
| `forge_ontology.py` | Ontologia Arkheion Forge |
| `forge_trainer.py` | Trainer Arkheion Forge (47K LOC) |
| `cross_tenant_learning.py` | Learning cross-tenant |
| `rag_tenant_resolution.py` | RAG tenant isolation |
| `operational_intent_classifier.py` | Classificador operacional |
| `operational_events.py` | Eventos operacionais |
| `diagnostic_potency.py` | Diagnóstico de potência (vendas) |
| `curriculum_engine.py` | Engine de currículo (vendas) |
| `sentiment_analyzer.py` | Analisador sentiment (vendas) |
| `sentiment_models.py` | Modelos de sentiment |
| `adaptive_conversation.py` | Conversação adaptativa (vendas) |
| `conversation_state_machine.py` | FSM conversação WhatsApp |
| `response_selector.py` | Seletor de resposta (vendas) |
| `response_validator.py` | Validador de resposta (vendas) |
| `response_pipeline.py` | Pipeline de resposta (vendas) |
| `multilingual_intent.py` | Intent multilíngue (vendas) |
| `multilingual_models.py` | Modelos multilíngues |
| `multimodal_mixin.py` | Mixin multimodal (Whisper) |
| `whisper_processor.py` | Processador Whisper |
| `vision_processor.py` | Processador de visão |
| `gene_bank_service.py` | Service layer (sales) |
| `korus_subagent.py` | Subagent Arkheion Forge |
| `mcp_neural_bridge.py` | Bridge neural do Arkheion Forge MCP |

---

## FASE 2: LIMPEZA — Remover Referências Arkheion Forge

### 2.1 Renaming Global: `korus` → `forge`

Todas as referências a "korus", "forge", "arkheion" devem ser substituídas:

| Padrão antigo | Novo padrão |
|--------------|-------------|
| `forge` | `arkheion` ou `forge` |
| `korus` | `forge` |
| `arkheion` | `arkheion` |
| `Arkheion Forge` | `Arkheion Forge` |
| `Arkheion` | `Arkheion` |

**Escopo**: ~120 arquivos Python + 5 arquivos Rust + 12 docs

### 2.2 Corrigir Imports Quebrados (15 arquivos)

#### `from backend.` → imports locais (4 arquivos)

| Arquivo | Import antigo | Novo import |
|---------|--------------|-------------|
| `tools/aura_detective.py` | `from backend.src.ai.tools.base` | `from ai_core.tools.base` |
| `tools/aura_educator.py` | `from backend.src.ai.tools.base` | `from ai_core.tools.base` |
| `tools/aura_analyst.py` | `from backend.src.ai.tools.base` | `from ai_core.tools.base` |
| `tools/aura_autopilot.py` | `from backend.src.ai.tools.base` | `from ai_core.tools.base` |

#### `from src.` → imports locais (11 arquivos)

| Arquivo | Import antigo | Novo import |
|---------|--------------|-------------|
| `system_crystallizer/_populator.py` | `from src.ai.mcp_server` | `from ai_core.mcp_server` |
| `forge-intel-python/phi_calculator.py` | `from src.arkheion.services` | `from ai_core.arkheion` |
| `forge-intel-python/consciousness/iit_v3_real.py` | `from src.arkheion.constants` | Definir PHI localmente: `PHI = 1.618033988749895` |
| `self_healing.py` | `from src.core.redis_manager` | `import redis` direto |
| `learning/outcomes.py` | `from src.ai.learning_loop` | `from ai_core.learning_loop` |
| `learning/fine_tuning.py` | `from src.ai.metrics_to_training` | `from ai_core.metrics_to_training` |
| `learning/fine_tuning.py` | `from src.services.distillation_gate` | Criar stub local |
| `learning/fine_tuning.py` | `from src.services.dreamer` | Criar stub local |

### 2.3 Remover Multi-Tenancy (125 arquivos)

**Estratégia**: Substituir `tenant_id` por `workspace_id` onde fizer sentido (code repos podem ter múltiplos workspaces), ou remover completamente onde for apenas isolation de vendas.

```python
# ANTES (multi-tenant vendas):
async def get_data(tenant_id: str, query: str) -> Result:
    ...

# DEPOIS (single-workspace coding):
async def get_data(query: str, workspace_id: str = "default") -> Result:
    ...
```

### 2.4 Rust: Limpar Referências Korus (5 arquivos)

| Arquivo | Ação |
|---------|------|
| `forge-core/src/formats/korus.rs` | Renomear para `legacy.rs` |
| `forge-core/src/formats/mod.rs` | Atualizar referência |
| `forge-core/src/lib.rs` | Remover export |
| `forge-core/src/genome.rs` | Substituir `korus` → `forge` |
| `forge-bridge/src/devbrain.rs` | Limpar referências |

---

## FASE 3: INTEGRAÇÃO — Reorganizar para o Propósito

### 3.1 Nova Estrutura de Diretórios

```
ai_core/
├── __init__.py                    # [NOVO] Package root
├── forge_api.py                   # [NOVO] Unified Parametric Coding API
├── config.py                      # [NOVO] Configuração standalone
│
├── perception/                    # [REORGANIZAR] Módulos de percepção
│   ├── rag.py                     # ← rag_pipeline.py + rag_auto_reindex.py + graph_rag_bridge.py
│   ├── cache.py                   # ← hdcache/ (5 layers)
│   ├── mesh.py                    # ← neural_mesh/ (crystals + embeddings)
│   └── semantic.py                # ← semantic_cache.py + semantic_resolver.py
│
├── reasoning/                     # [REORGANIZAR] Módulos de raciocínio
│   ├── agent.py                   # ← agent.py (sem sales context)
│   ├── planner.py                 # ← planning/ + macro_micro_planner.py
│   ├── autopilot.py               # ← autopilot/ (sessões autônomas de coding)
│   ├── thinking.py                # ← cognitive_thinking.py + thinking_tools.py
│   └── dream.py                   # ← dream/ (counterfactual)
│
├── synthesis/                     # [REORGANIZAR] Módulos de geração
│   ├── inference.py               # ← streaming_inference.py + speculative_decoder.py
│   ├── model_router.py            # ← neural_model_router.py + llm_router.py
│   ├── prompt.py                  # ← prompt_engine.py
│   └── crystallizer.py            # ← system_crystallizer/
│
├── evolution/                     # [REORGANIZAR] Módulos de aprendizado
│   ├── learning.py                # ← learning/ (auto-melhoria)
│   ├── fine_tuning.py             # ← fine_tuning/
│   ├── memory.py                  # ← memory/ + dialog_memory.py
│   └── feedback.py                # ← human_feedback.py + feedback_collector.py
│
├── kernel/                        # [MANTER] De arkheion/kernels/
│   ├── phi.py, graph.py, embedding.py, compression.py
│   ├── resonance.py, fingerprint.py, clustering.py, synthesis.py
│   └── igpu_cache.py
│
├── temporal/                      # [MANTER] De arkheion/temporal/
│   └── wal.py, chain.py, snapshot.py, store.py, invariant.py
│
├── intel/                         # [MANTER] Forge Intel Python
│   ├── compression/, consciousness/, sacred_geometry/, synthesis/
│   └── gene_evolution.py, mutation.py, neural_transplant.py
│
├── bridge/                        # [SIMPLIFICAR]
│   ├── gpu.py                     # ← gpu_bridge.py
│   ├── rust.py                    # ← arkheion_forge_bridge.py
│   ├── mcp.py                     # ← mcp_server.py + mcp_transport.py
│   └── editor.py                  # [NOVO] Bridge para forge-editor
│
└── tools/                         # [REFINAR] Só tools para coding
    ├── base.py                    # BaseTool (manter)
    ├── code_search.py             # [NOVO] Busca semântica de código
    ├── code_edit.py               # [NOVO] Edição paramétrica
    ├── code_generate.py           # [NOVO] Geração de código
    ├── code_test.py               # [NOVO] Teste e validação
    └── code_analyze.py            # [NOVO] Análise estática
```

### 3.2 Integração Editor ↔ AI Pipeline

Atualmente o `forge-editor/ai_chat.rs` tem:
- ✅ Chat messages (User, Assistant, System)
- ✅ Metadata (Φ-coherence, latency, tokens)
- ✅ ProposedChange struct (file_path, before, after)
- ❌ **Não conectado ao Python AI pipeline**

**Ação**: Criar `forge-bridge/src/ollama.rs`:

```rust
pub struct OllamaClient {
    base_url: String,  // http://localhost:11435
}

impl OllamaClient {
    pub async fn generate(&self, prompt: &str, model: &str) -> Result<String>;
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>>;
}
```

**E conectar**: `forge-editor` → `forge-bridge` → `OllamaClient` → `Ollama`

### 3.3 Integração forge-brain ↔ Python AI

O `forge-brain` (Rust) tem reasoning completo (2876 LOC think.rs) mas opera apenas sobre **modelos neurais (GGUF/SafeTensors)**, não sobre código.

**Ação**: Estender `forge-brain` para aceitar **código como input**:

```
forge-brain::think()   →  recebe código  →  analisa via embeddings
forge-brain::plan()    →  recebe intent  →  decompõe em steps
forge-brain::surgery() →  recebe diff    →  aplica em model/code
```

---

## FASE 4: REFINO — Polimento Final

### 4.1 Documentação

| Doc | Ação |
|-----|------|
| `docs/architecture/arquitetura.md` | **DELETAR** (cópia do Arkheion Forge) |
| `docs/architecture/MCP-ARCHITECTURE.md` | **DELETAR** (específico Arkheion Forge) |
| `docs/PROJECT_CONTEXT.md` | **DELETAR** (específico Arkheion Forge) |
| `docs/architecture/ARCHITECTURE.md` | Atualizar após remoções |
| `docs/architecture/SUBSYSTEM_MAP.md` | Remover módulos sales |
| `docs/architecture/ELM_ARCHITECTURE.md` | Substituir korus → forge |
| `docs/subsystems/*.md` | Atualizar após remoções |
| `docs/guides/*.md` | Atualizar após remoções |
| `docs/README.md` | Atualizar inventário |

### 4.2 Testes: Limpar Mirror Lattice

Após remoção dos módulos de vendas, deletar testes que referenciam módulos removidos.
Manter apenas testes de: arkheion, dream, mcp (refatorado), memory (se relevante).

### 4.3 Verificações de Build

```bash
# Rust
cd arkheion-forge && cargo build --release

# Python
cd arkheion-forge && python -c "import ai_core"

# Testes
cargo test --workspace
pytest tests/ -v
```

---

## FASE 5: Construção dos Módulos Novos

### 5.1 Novos Módulos Python

| Módulo | Propósito | Prioridade |
|--------|-----------|-----------|
| `ai_core/forge_api.py` | API unificada — entry point | 🔴 P0 |
| `ai_core/config.py` | Configuração standalone | 🔴 P0 |
| `ai_core/tools/code_search.py` | Busca semântica código | 🟡 P1 |
| `ai_core/tools/code_edit.py` | Edição paramétrica | 🟡 P1 |
| `ai_core/tools/code_generate.py` | Geração de código | 🟡 P1 |
| `ai_core/tools/code_test.py` | Teste e validação | 🟡 P1 |
| `ai_core/bridge/editor.py` | Bridge forge-editor | 🟡 P1 |

### 5.2 Novos Módulos Rust

| Módulo | Propósito | Prioridade |
|--------|-----------|-----------|
| `forge-bridge/src/ollama.rs` | Cliente Ollama HTTP | 🔴 P0 |
| `forge-bridge/src/python_api.rs` | Chamadas à forge_api.py | 🟡 P1 |
| `forge-editor/src/code_actions.rs` | Ações de código AI-driven | 🟡 P1 |

---

## Resultado Esperado

```
ANTES                          DEPOIS
530 .py files                  ~280 .py files (-47%)
401 test files                 ~250 test files (-38%)
~125 com tenant_id             0 com tenant_id
~120 com "korus"               0 com "korus"
77 módulos de vendas           0 módulos de vendas
95 duplicatas backend/         0 duplicatas
0 coding tools                 5 coding tools
0 editor integration           1 completa (Rust↔Python↔Ollama)
```
