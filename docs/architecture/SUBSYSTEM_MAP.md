# Subsystem Map — Complete Module Reference

> Every module in Arkheion Forge catalogued with purpose, size, and interconnections.

---

## 1. Rust Crates (Layer 0 — Native Core)

### forge-core (42 files)

The computational foundation. All tensor operations, format I/O, and SIMD-optimized routines.

| File | Purpose |
|------|---------|
| `lib.rs` | Public API surface |
| `ops.rs` | Core tensor operations (add, mul, softmax, layer_norm) |
| `simd.rs` | SIMD-optimized inner loops |
| `backward.rs` | Gradient computation (backpropagation) |
| `inference.rs` | Forward pass inference engine |
| `tokenizer.rs` | BPE/SentencePiece tokenizer |
| `training.rs` | Training loop orchestrator |
| `brain_predictor.rs` | Brain-based prediction module |
| `lattice.rs` | Lattice structure management |
| `gene.rs` | Gene abstraction (neural network layer as gene) |
| `genome.rs` | Genome = collection of genes = full model |
| `elm_gene.rs` | ELM-specific gene representation |
| `corpus.rs` | Training corpus management |
| `curriculum.rs` | Curriculum learning strategies |
| `cross_training.rs` | Multi-model cross-training |
| `dual_nucleus.rs` | Dual nucleus architecture |
| `synthesizer.rs` | Code/model synthesis from components |
| `distillation.rs` | Knowledge distillation |
| `empowerment.rs` | Empowerment-based learning |
| `coherence.rs` | Φ-coherence calculations |
| `frequency_bands.rs` | Frequency-domain analysis |
| `mutation_log.rs` | Log of all mutations applied |
| `mutation_strategy.rs` | Mutation selection strategies |
| `resilience.rs` | Fault tolerance and recovery |
| `runtime_learning.rs` | Online learning at runtime |
| `scale_calibration.rs` | Multi-scale parameter calibration |
| `versioning.rs` | Model version management |
| `metrics_export.rs` | Performance metrics export |
| `marketplace.rs` | Gene marketplace (share/trade genes) |
| `tracer.rs` | Execution tracing |
| **codec/** | |
| `mod.rs` | Codec registry |
| `trit.rs` | Ternary (trit) codec |
| **formats/** | |
| `mod.rs` | Format registry |
| `gguf.rs` | GGUF format reader/writer |
| `safetensors.rs` | SafeTensors reader/writer |
| `nucleus.rs` | Nucleus v1/v2 format |
| `nucleus_v3.rs` | Nucleus v3 with Φ metadata |
| `arktern.rs` | ArkTern ternary format |
| `mobile.rs` | Mobile-optimized format |
| `legacy.rs` | Legacy format (formerly Korus) |

### forge-gpu (11 files)

GPU-accelerated operations via ROCm/HIP (AMD) and optionally CUDA.

| File | Purpose |
|------|---------|
| `lib.rs` | GPU engine public API |
| `gpu_engine.rs` | Core GPU dispatch engine |
| `inference.rs` | GPU-accelerated inference |
| `training_gpu.rs` | GPU training loop |
| `backward_gpu.rs` | GPU backpropagation |
| `distill_gpu.rs` | GPU knowledge distillation |
| `renderer.rs` | GPU-based visualization renderer |
| `training_advisor.rs` | Optimal training configuration advisor |
| **tests/** | |
| `mod.rs` | Test module |
| `test_gpu_lib_loading.rs` | GPU library loading tests |
| **bin/** | |
| `bench_gpu.rs` | GPU benchmark tool |

### forge-brain (15 files)

AI reasoning, planning, and surgical intervention.

| File | Purpose |
|------|---------|
| `lib.rs` | Brain public API |
| `analyzer.rs` | Code/model analysis engine |
| `causal.rs` | Causal reasoning |
| `diagnosis.rs` | Problem diagnosis engine |
| `empowerment.rs` | Empowerment calculation |
| `evolution.rs` | Evolutionary optimization |
| `executor.rs` | Plan execution engine |
| `genesis.rs` | Genesis cycle (create from nothing) |
| `memory.rs` | Working memory management |
| `phi.rs` | Φ-coherence integration |
| `planner.rs` | Multi-step plan generation |
| `prompter.rs` | Dynamic prompt construction |
| `surgery.rs` | Surgical model editing |
| `think.rs` | Extended thinking / reasoning |
| **bin/** | |
| `brain_cli.rs` | Brain CLI tool |

### forge-bridge (14 files)

Inter-process communication — D-Bus, MCP, DevBrain, and runtime feedback.

| File | Purpose |
|------|---------|
| `lib.rs` | Bridge public API |
| `bridge.rs` | Core bridge abstraction |
| `dbus_client.rs` | D-Bus client (connect to system services) |
| `dbus_server.rs` | D-Bus server (expose forge services) |
| `devbrain.rs` | DevBrain MCP integration |
| `huam_client.rs` | HUAM client connection |
| `kernel.rs` | Kernel-level bridge |
| `notify.rs` | Notification dispatch |
| `nucleus.rs` | Nucleus format bridge |
| `resonance.rs` | Resonance field bridge |
| `runtime_feedback.rs` | Runtime feedback collection |
| `security.rs` | Security layer |
| `training_stream.rs` | Training data streaming |
| `types.rs` | Bridge type definitions |

### forge-editor (8 files)

Native code editor with AI integration.

| File | Purpose |
|------|---------|
| `lib.rs` | Editor public API |
| `code_editor.rs` | Core code editing engine |
| `ai_chat.rs` | AI chat panel |
| `file_explorer.rs` | File system browser |
| `language.rs` | Language detection and highlighting |
| `tab_manager.rs` | Multi-tab management |
| `terminal.rs` | Embedded terminal |
| `recombinator_state.rs` | Gene recombinator UI state |

### forge-ui (13 files)

egui-based native UI with 3D visualization.

### forge-bank (10 files)

Gene bank for storing, cataloging, and retrieving neural network genes.

| File | Purpose |
|------|---------|
| `lib.rs` | Bank public API |
| `bank.rs` | Gene bank storage |
| `catalog.rs` | Gene catalog (searchable index) |
| `decomposer.rs` | Model → gene decomposition |
| `lineage.rs` | Gene lineage tracking |
| `observatory.rs` | Gene observation and metrics |
| `query.rs` | Gene query engine |
| `similarity.rs` | Gene similarity computation |
| `synthesizer.rs` | Gene synthesis from components |

### forge-intel (17 files)

Evolutionary intelligence — mutation, selection, gene evolution.

### forge-mcp (5 files)

MCP protocol implementation for AI assistant integration.

### forge-python (3 files)

PyO3 bindings exposing Rust functions to Python.

---

## 2. Python AI Core (Layer 1 — Intelligence)

### Top-Level Modules (Direct in ai-core/)

| Module | LOC | Purpose |
|--------|-----|---------|
| `agent.py` | 83K | Core AI agent — message handling, intent classification |
| `handler.py` | 58K | Request handler pipeline |
| `operator_agent.py` | 80K | Operational AI agent |
| `streaming_inference.py` | 54K | Streaming LLM inference engine |
| `rag_pipeline.py` | 51K | Retrieval-Augmented Generation |
| `autonomous_conductor.py` | 32K | Autonomous operation orchestrator |
| `pipeline_chain.py` | 39K | Multi-stage pipeline chaining |
| *(removed: forge_trainer.py)* | — | *(deleted — Arkheion Forge legacy)* |
| `self_healing.py` | 48K | Self-healing system |
| `semantic_cache.py` | 36K | Semantic similarity cache |
| `model_registry.py` | 36K | Model registration and management |
| `neural_model_router.py` | 36K | Intelligent model routing |
| `intelligent_cache.py` | 34K | AI-powered caching |
| `cognitive_thinking.py` | 35K | Extended thinking engine |
| `mesh_bridge.py` | 33K | Neural mesh bridge |
| `consolidation_engine.py` | 41K | Memory consolidation |
| `auto_learning_cache.py` | 31K | Self-learning cache |

| `training_data_extractor.py` | 30K | Training data extraction |
| `metrics_to_training.py` | 33K | Metrics → training data |
| `autonomy_loop.py` | 33K | Autonomous execution loop |
| `human_feedback.py` | 27K | Human feedback integration |
| `cloud_provider.py` | 33K | Cloud LLM provider |
| `semantic_intent_router.py` | 27K | Intent-based routing |
| `analytics.py` | 27K | AI analytics engine |
| `dialog_memory.py` | 22K | Dialog memory management |
| `adaptive_conversation.py` | 22K | Adaptive conversation |
| `llm_handler_mixin.py` | 24K | LLM handler base mixin |
| `execution_runtime.py` | 24K | AI execution runtime |
| `execution_graph_manager.py` | 22K | Execution graph DAG |
| `runtime_model_config.py` | 24K | Runtime model configuration |
| `quality_monitor.py` | 20K | Quality monitoring |
| `prompt_engine.py` | 21K | Prompt construction engine |
| `semantic_resolver.py` | 20K | Semantic resolution |
| `arkheion_forge_bridge.py` | 26K | Bridge to Rust forge |
| `gpu_bridge.py` | 17K | GPU acceleration bridge |
| `mcp_server.py` | 24K | MCP server implementation |
| `cross_lattice_bridge.py` | 4K | Cross-lattice communication |
| `speculative_decoder.py` | 9K | Speculative decoding |
| `llm_router.py` | 14K | LLM routing logic |

### Arkheion (45 files) — Kernel Layer

See §3.1 in ARCHITECTURE.md.

### HDCache (15 files) — 5-Layer Cache

See §3.2 in ARCHITECTURE.md.

### Neural Mesh (7 files) — Cognitive Lattice

See §3.3 in ARCHITECTURE.md.

### RAG (4 files) — Retrieval Engine

See §3.4 in ARCHITECTURE.md.

### Dream (5 files) — Counterfactual Engine

See §3.5 in ARCHITECTURE.md.

### Autopilot (8 files) — Autonomous Sessions

| Module | Purpose |
|--------|---------|
| `engine.py` | 85K LOC — Autopilot execution engine |
| `session.py` | 45K LOC — Session management |
| `tool_registry.py` | 59K LOC — Tool registration |
| `hardening_gate.py` | Safety gates |
| `introspection.py` | Self-analysis |
| `knowledge.py` | Knowledge integration |
| `simulation.py` | Outcome simulation |

### Planning (4 files) — Strategic Planning

| Module | Purpose |
|--------|---------|
| `engine.py` | 68K LOC — Planning engine with decomposer |
| `decomposer.py` | 20K LOC — Task decomposition |
| `models.py` | Planning data models |

### Fine-Tuning (7 files) — Model Training

| Module | Purpose |
|--------|---------|
| `__init__.py` | 18K LOC — Pipeline orchestration |
| `data_collector.py` | 28K LOC — Training data collection |
| `trainer.py` | 29K LOC — Training execution |
| `evaluator.py` | 16K LOC — Model evaluation |
| `preprocessor.py` | 11K LOC — Data preprocessing |
| `_manager.py` | Training job management |
| `_types.py` | Training type definitions |

### Learning (7 files) — Self-Improvement

| Module | Purpose |
|--------|---------|
| `evolution.py` | Evolutionary learning strategies |
| `fine_tuning.py` | Learning-driven fine-tuning |
| `maintenance.py` | Knowledge maintenance |
| `objections.py` | Objection learning |
| `outcomes.py` | Outcome tracking and learning |

### Memory (8 files) — Persistent Memory

| Module | Purpose |
|--------|---------|
| `memory_search.py` | 10K LOC — Memory search engine |
| `structured_index.py` | Structured memory index |
| `causal_reconstructor.py` | Causal chain reconstruction |
| `weekly_replay.py` | Weekly memory replay |
| `daily_log.py` | Daily activity logging |
| `lessons_learned.py` | Lessons learned repository |
| `anchor_service.py` | Memory anchor points |

### System Crystallizer (8 files) — Build Engine

See §3.6 in ARCHITECTURE.md.

### Forge Intel Python (30 files) — Evolutionary Intelligence

See §3.7 in ARCHITECTURE.md.

### Tools (27 files) — AI Tool Registry

All AI-callable tools for autonomous operations.

> **Note**: The following modules were removed during the Arkheion Forge migration cleanup:
> Video Production (20 files), Financial/Prospero (16 files), Regional Brasil (9 files),
> Coach (8 files), Seller Bot (5 files), Marketing (13 files), Image Gen (4 files), Orchestration (2 files).

---

## 3. Test Mirror Lattice (401 files)

```
tests/
├── mirrors/
│   ├── ai/                    # Direct mirrors of ai-core modules
│   │   ├── arkheion/          # Kernel tests
│   │   │   ├── kernels/       # test_phi_kernel, test_graph_kernel, etc.
│   │   │   ├── temporal/      # test_wal, test_chain, test_snapshot
│   │   │   ├── condensation/  # test_kernel
│   │   │   ├── holographic/   # test_point_cloud, test_observer
│   │   │   ├── graph/         # test_csr
│   │   │   └── ternary/       # test_quantizer
│   │   ├── dream/             # test_counterfactual, test_consolidator
│   │   ├── services/          # test_neuron_dream_induction
│   │   └── test_*.py          # Top-level AI tests
│   ├── mcp_server/            # MCP bridge tests
│   │   ├── handlers/          # test_mesh
│   │   └── test_mcp_neural_bridge.py
│   └── memory/                # Memory system tests
│       └── test_lead_memory_rag.py
│
├── unit/                      # Additional unit tests
│   ├── arkheion/              # Arkheion unit tests
│   ├── hdcache/               # HDCache tests
│   ├── neural_mesh/           # Neural Mesh tests
│   ├── autopilot/             # Autopilot tests
│   └── ...
│
└── integration/               # Integration tests
```

---

## 4. Infrastructure

### Docker Services (docker-compose.elm-forge.yml)

| Service | Image | Port | Purpose |
|---------|-------|------|---------|
| `elm-forge-postgres` | pgvector/pgvector:pg17 | 5433 | Embedding storage |
| `elm-forge-redis` | redis:7-alpine | 6380 | Hot cache + patterns |
| `elm-forge-ollama` | ollama/ollama:rocm | 11435 | Local LLM + embeddings |

### MCP Servers

| Server | File | Purpose |
|--------|------|---------|
| forge-offtoken | `mcp-servers/forge-offtoken.js` | Local model via MCP |
