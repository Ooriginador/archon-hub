# ARKHEION FORGE — Autonomous Intelligence Platform

> **Version**: 1.0.0-GENESIS  
> **Date**: 2026-04-23  
> **Status**: Architecture Defined — Pre-Construction  
> **License**: MIT  
> **Identity**: Standalone project

---

## What Is This?

Arkheion Forge is an **autonomous intelligence platform** where AI and code are unified into a single cognitive architecture. Instead of treating AI as a tool that writes code, or code as something separate from intelligence — the Forge treats them as **the same thing**: neural representations that can be understood, evolved, and synthesized in parameter space.

Every file is a neuron in a cognitive lattice. Every build is a synthesis cycle. Every test is an immune checkpoint. The AI doesn't *assist* coding — it **is** the coding.

The system fuses three paradigms:
1. **Evolutionary Lattice Modalities (ELM)** — Cognitive build system with awareness, evolution, synthesis, validation
2. **Neural Forge** — GPU-accelerated model surgery (prune, mutate, transplant, distill)
3. **Parametric Intelligence** — Perception, reasoning, and synthesis in embedding/parameter space via RAG, fine-tuning, and semantic inference

---

## Documentation Map

| Document | Purpose |
|----------|---------|
| [ARCHITECTURE.md](architecture/ARCHITECTURE.md) | **Master architecture** — full system design |
| [SUBSYSTEM_MAP.md](architecture/SUBSYSTEM_MAP.md) | Detailed map of every subsystem and module |
| [PARAMETER_SPACE_CODING.md](architecture/PARAMETER_SPACE_CODING.md) | The parametric coding paradigm explained |
| [ELM_ARCHITECTURE.md](architecture/ELM_ARCHITECTURE.md) | Evolutionary Lattice Modalities — build system |
| [CRATE_REFERENCE.md](subsystems/CRATE_REFERENCE.md) | Rust crate documentation |
| [AI_CORE_REFERENCE.md](subsystems/AI_CORE_REFERENCE.md) | Python AI core module reference |
| [INFRASTRUCTURE.md](guides/INFRASTRUCTURE.md) | Docker, GPU, DevBrain setup |
| [GETTING_STARTED.md](guides/GETTING_STARTED.md) | Build and run instructions |

---

## Project Inventory

```
arkheion-forge/                        # ROOT
├── crates/                            # Rust workspace (138 .rs files)
│   ├── forge-core/     (42 files)     # Tensor ops, codecs, formats, training
│   ├── forge-gpu/      (11 files)     # ROCm/HIP GPU acceleration
│   ├── forge-brain/    (15 files)     # AI reasoning, planning, surgery
│   ├── forge-intel/    (17 files)     # Evolutionary intelligence + Python FFI
│   ├── forge-bridge/   (14 files)     # D-Bus, DevBrain, resonance bridges
│   ├── forge-editor/   (8 files)      # Code editor with AI chat
│   ├── forge-ui/       (13 files)     # egui native UI
│   ├── forge-bank/     (10 files)     # Gene bank, similarity, synthesis
│   ├── forge-mcp/      (5 files)      # MCP protocol integration
│   └── forge-python/   (3 files)      # PyO3 Python bindings
│
├── ai-core/                           # Python AI intelligence (530 .py files)
│   ├── arkheion/       (45 files)     # Core: kernels, temporal, condensation
│   ├── hdcache/        (15 files)     # 5-layer hierarchical cache
│   ├── neural_mesh/    (7 files)      # Crystals, embeddings, spheres, resonance
│   ├── dream/          (5 files)      # Counterfactual engine, consolidation
│   ├── rag/            (4 files)      # RAG pipeline + auto-reindex
│   ├── autopilot/      (8 files)      # Autonomous AI agent sessions
│   ├── planning/       (4 files)      # Strategic plan decomposer + engine
│   ├── learning/       (7 files)      # Evolution, fine-tuning, outcomes
│   ├── fine_tuning/    (7 files)      # Data collector, trainer, evaluator
│   ├── memory/         (8 files)      # Causal reconstructor, memory search
│   ├── tools/          (27 files)     # AI tool registry
│   ├── system_crystallizer/ (8 files) # Cython crystallization orchestrator
│   └── ...             (385+ more)    # Agent, inference, cache, etc.
│
├── tests/                             # Mirror Lattice (401 test files)
│   ├── mirrors/ai/                    # Direct mirror of ai-core
│   ├── mirrors/mcp_server/            # MCP bridge tests
│   └── unit/                          # Unit tests
│
├── mcp-servers/                       # MCP protocol servers
│   └── forge-offtoken.js              # Local model MCP server
│
├── docs/                              # Documentation
│   ├── architecture/                  # Architecture documents
│   ├── subsystems/                    # Module references
│   └── guides/                        # Setup and usage guides
│
├── src/                               # Rust binaries
│   ├── main.rs                        # GUI launcher (egui)
│   ├── cli.rs                         # CLI interface
│   └── nucleus_phi_demo.rs            # Φ-coherence demo
│
├── Cargo.toml                         # Workspace manifest
└── docker-compose.elm-forge.yml       # Intelligence substrate
```

---

## The Three Pillars

### Pillar 1: ELM (Build as Cognition)

```
Awareness → Evolution → Synthesis → Validation → Observation
(Sense)    (Analyze)   (Create)    (Verify)      (Observe)
```

### Pillar 2: Neural Forge (Model Surgery)

```
Load Model → Decompose Genes → Visualize → Mutate/Prune → Resynthesize → Validate
```

### Pillar 3: Parametric Coding (Code in Parameter Space)

```
Intent → Semantic Embedding → RAG Retrieval → Parameter Synthesis → Crystal Output
```

---

## Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Native Core** | Rust + egui | Tensor ops, GPU, UI, codecs |
| **GPU Acceleration** | ROCm/HIP (AMD) | Training, inference, backprop |
| **AI Intelligence** | Python 3.12 | RAG, agent, planning, learning |
| **Embeddings** | Ollama (nomic-embed-text) | Semantic search and similarity |
| **Vector Store** | PostgreSQL + pgvector | Embedding persistence |
| **Hot Cache** | Redis 7 | Pattern cache, decision store |
| **Model Formats** | GGUF, SafeTensors, Nucleus | Neural network file I/O |
| **IPC** | D-Bus + MCP + PyO3 | Cross-process communication |
