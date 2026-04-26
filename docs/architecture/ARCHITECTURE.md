# ARKHEION FORGE — Master Architecture Document

> **Version**: 1.0.0-GENESIS  
> **Date**: 2026-04-23  
> **Classification**: Foundational — Architectural Truth  
> **Scope**: Full system design for autonomous AI coding engine

---

## 1. Vision: Coding in Parameter Space

### 1.1 The Paradigm Shift

Traditional coding operates in **text space**:
```
Human writes text → Parser tokenizes → Compiler transforms → Binary executes
```

Arkheion Forge operates in **parameter space**:
```
Intent defined → Semantic embedding computed → Similar patterns retrieved (RAG) →
Parameters synthesized in latent space → Code materialized → Crystal validated
```

The fundamental insight: **code IS data**. Every function, class, and module can be represented as a point in a high-dimensional embedding space. Editing code becomes navigating and transforming that space — not manipulating strings.

### 1.2 What "Parameter Space Coding" Means

When we say "code in parameter space," we mean:

1. **Every code file has an embedding** — a 768-dimensional vector that captures its semantic meaning
2. **Editing is a vector operation** — instead of "add line X after line Y," it's "move the embedding towards intent Z"
3. **Generation is retrieval + synthesis** — find the N closest existing implementations, then synthesize a new one
4. **Validation is coherence checking** — the new code must maintain Φ-coherence with the rest of the lattice

### 1.3 The Three Loops

```
┌────────────────────────────────────────────────────────────────────────┐
│                    PARAMETRIC CODING ENGINE                            │
│                                                                        │
│   ┌──────────────────────────────────────────────────────────────┐    │
│   │  LOOP 1: UNDERSTAND (Read → Embed → Index)                  │    │
│   │                                                              │    │
│   │  Source Code → Chunking → Embedding (Ollama) → pgvector     │    │
│   │  DevBrain indexes 67K+ symbols with semantic embeddings     │    │
│   │  HDCache provides 5-layer retrieval (RAM→Redis→FP→PGV→LLM) │    │
│   └──────────────────────────────────────────────────────────────┘    │
│                          │                                            │
│                          ▼                                            │
│   ┌──────────────────────────────────────────────────────────────┐    │
│   │  LOOP 2: SYNTHESIZE (Intent → Retrieve → Generate → Forge)  │    │
│   │                                                              │    │
│   │  Intent → RAG Pipeline retrieves similar code chunks         │    │
│   │  Neural Model Router selects optimal model                   │    │
│   │  Streaming Inference generates candidate code                │    │
│   │  System Crystallizer compiles to native binary               │    │
│   └──────────────────────────────────────────────────────────────┘    │
│                          │                                            │
│                          ▼                                            │
│   ┌──────────────────────────────────────────────────────────────┐    │
│   │  LOOP 3: VALIDATE (Test → Cohere → Seal → Learn)            │    │
│   │                                                              │    │
│   │  Mirror Lattice runs immune checks (401 test files)          │    │
│   │  Φ-Kernel calculates coherence score                         │    │
│   │  Manifest seals the lattice state                            │    │
│   │  Learning loop feeds results back for self-improvement       │    │
│   └──────────────────────────────────────────────────────────────┘    │
│                          │                                            │
│                          └───────► Loop 1 (continuous evolution)      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. System Architecture

### 2.1 Layer Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         LAYER 0: NATIVE CORE (Rust)                        │
│                                                                             │
│  forge-core        forge-gpu         forge-intel        forge-bank          │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐      ┌──────────┐         │
│  │ Tensor   │     │ ROCm/HIP │     │ Evo Pipe │      │ Gene     │         │
│  │ Ops      │     │ Training │     │ Sacred   │      │ Bank     │         │
│  │ Codecs   │     │ Inference│     │ Geometry │      │ Catalog  │         │
│  │ Formats  │     │ Backprop │     │ IIT Φ    │      │ Query    │         │
│  │ SIMD     │     │ Distill  │     │ Compress │      │ Synth    │         │
│  └──────────┘     └──────────┘     └──────────┘      └──────────┘         │
│                                                                             │
│  forge-brain       forge-bridge      forge-editor     forge-ui             │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐          │
│  │ Analyzer │     │ D-Bus    │     │ Code Ed  │     │ egui App │          │
│  │ Planner  │     │ DevBrain │     │ AI Chat  │     │ Panels   │          │
│  │ Surgery  │     │ Resonance│     │ File Exp │     │ Gene Viz │          │
│  │ Executor │     │ MCP      │     │ Terminal │     │ Controls │          │
│  │ Memory   │     │ Security │     │ Tabs     │     │ History  │          │
│  └──────────┘     └──────────┘     └──────────┘     └──────────┘          │
│                                                                             │
│  forge-mcp         forge-python                                            │
│  ┌──────────┐     ┌──────────┐                                             │
│  │ Protocol │     │ PyO3     │                                             │
│  │ Handler  │     │ Bindings │                                             │
│  └──────────┘     └──────────┘                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                     LAYER 1: AI INTELLIGENCE (Python)                       │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  PERCEPTION LAYER                                                   │   │
│  │  ┌────────┐ ┌──────────┐ ┌───────────┐ ┌──────────────┐           │   │
│  │  │RAG     │ │Semantic  │ │HDCache    │ │Neural Mesh   │           │   │
│  │  │Pipeline│ │Cache     │ │(5-layer)  │ │(Crystals +   │           │   │
│  │  │        │ │          │ │           │ │ Embeddings + │           │   │
│  │  │51K LOC │ │36K LOC   │ │46K LOC    │ │ Spheres)     │           │   │
│  │  └────────┘ └──────────┘ └───────────┘ └──────────────┘           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  REASONING LAYER                                                    │   │
│  │  ┌────────┐ ┌──────────┐ ┌───────────┐ ┌──────────────┐           │   │
│  │  │Agent   │ │Planning  │ │Autopilot  │ │Dream Engine  │           │   │
│  │  │Core    │ │Engine    │ │Session    │ │(Counterfact  │           │   │
│  │  │        │ │          │ │           │ │ + Consolid)  │           │   │
│  │  │83K LOC │ │68K LOC   │ │85K LOC    │ │              │           │   │
│  │  └────────┘ └──────────┘ └───────────┘ └──────────────┘           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  EVOLUTION LAYER                                                    │   │
│  │  ┌────────┐ ┌──────────┐ ┌───────────┐ ┌──────────────┐           │   │
│  │  │Learning│ │Fine-     │ │System     │ │Model         │           │   │
│  │  │Loop    │ │Tuning    │ │Crystalliz.│ │Registry      │           │   │
│  │  │        │ │Pipeline  │ │           │ │              │           │   │
│  │  └────────┘ └──────────┘ └───────────┘ └──────────────┘           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────────────────┤
│                    LAYER 2: INTELLIGENCE SUBSTRATE                          │
│                                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                     │
│  │  PostgreSQL   │  │    Redis     │  │   Ollama     │                     │
│  │  + pgvector   │  │   7-Alpine   │  │  ROCm/GPU   │                     │
│  │               │  │              │  │              │                     │
│  │  Embeddings   │  │  Hot Cache   │  │  Embeddings  │                     │
│  │  Symbol Index │  │  Patterns    │  │  Generation  │                     │
│  │  Dependencies │  │  Decisions   │  │  Inference   │                     │
│  └──────────────┘  └──────────────┘  └──────────────┘                     │
│       :5433              :6380             :11435                           │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Data Flow: From Intent to Crystal

```
                    ┌─────────────┐
                    │  USER INTENT │
                    │  "implement  │
                    │   auth flow" │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  EMBEDDING  │  Ollama: nomic-embed-text
                    │  COMPUTATION│  → 768-dim vector
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
       ┌──────▼──────┐ ┌──▼───────┐ ┌──▼──────────┐
       │ RAG PIPELINE│ │ HDCACHE  │ │ NEURAL MESH │
       │ k-NN search │ │ 5-layer  │ │ Crystal     │
       │ Top-N chunks│ │ lookup   │ │ matching    │
       └──────┬──────┘ └──┬───────┘ └──┬──────────┘
              │            │            │
              └────────────┼────────────┘
                           │
                    ┌──────▼──────┐
                    │  SYNTHESIS  │  Merge retrieved chunks
                    │  ENGINE     │  + model generation
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  CRYSTAL    │  Cython → C → .so
                    │  FORGE      │  (or direct code output)
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  VALIDATION │  Φ-Coherence check
                    │  Φ ≥ 0.6    │  + Mirror Lattice tests
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │  MANIFEST   │  lattice_manifest.json
                    │  SEAL       │  SHA-256 compound hash
                    └─────────────┘
```

---

## 3. The Seven Subsystems

### 3.1 Arkheion Core (Kernel Layer)

**Purpose**: The computational kernel — phi calculations, graph operations, compression, embeddings.

```
arkheion/
├── kernels/
│   ├── phi_kernel.py           # Golden ratio decay, coherence scoring
│   ├── graph_kernel.py         # Graph operations on dependency lattice
│   ├── embedding_kernel.py     # Embedding computation and comparison
│   ├── compression_kernel.py   # Data compression algorithms
│   ├── fingerprint_kernel.py   # Content fingerprinting
│   ├── resonance_kernel.py     # Resonance field calculations
│   ├── synthesis_kernel.py     # Crystal synthesis operations
│   ├── clustering_kernel.py    # Semantic clustering
│   └── igpu_cache.py           # Integrated GPU cache (zero-copy)
│
├── temporal/
│   ├── wal.py                  # Write-Ahead Log for causal traces
│   ├── chain.py                # Temporal chain of snapshots
│   ├── snapshot.py             # Point-in-time lattice state
│   ├── store.py                # Persistent temporal storage
│   └── invariant.py            # Temporal invariant enforcement
│
├── condensation/
│   ├── engine.py               # Memory condensation engine
│   ├── kernel.py               # Condensation kernel ops
│   └── consensus.py            # Multi-node consensus protocol
│
├── graph/                      # CSR graph structures
├── holographic/                # Holographic projection/observation
├── synthesis/                  # Binary synthesis pipeline
└── ternary/                    # Ternary quantization support
```

### 3.2 HDCache (5-Layer Hierarchical Cache)

**Purpose**: Ultra-fast retrieval across 5 cache levels with automatic promotion/eviction.

```
L0a: RAM (iGPU zero-copy buffer)     ← < 0.01ms latency
L0b: Redis (hot cache + BlobPool)     ← < 1ms latency
L1:  Fingerprint + HDGraph            ← < 5ms (structural match)
L2:  pgvector (semantic similarity)   ← < 20ms (embedding search)
L3:  LLM (generative fallback)        ← < 2000ms (model inference)
```

Key files:
- `_cache.py` — Core cache orchestrator (46K LOC)
- `_graph.py` — HDGraph structure for structural matching
- `_backbone.py` — Cache backbone infrastructure
- `_blob_pool.py` — Deduplicated blob storage
- `_fingerprint.py` — Content fingerprinting for L1
- `_pulse.py` — Cache health monitoring
- `_causal.py` — Causal cache invalidation

### 3.3 Neural Mesh (Cognitive Lattice)

**Purpose**: The living neural network that connects all code entities.

```
_crystals.py    — SkillCrystals: crystallized knowledge units (56K LOC)
_embeddings.py  — CapabilityEmbeddings: semantic representations
_spheres.py     — AccelerationSpheres: performance optimization zones
_resonance.py   — AdaptiveResonanceField: pattern reinforcement
_mesh.py        — NeuralMesh: the connecting fabric
_sales_nexus.py — Sales-specific neural connections
```

### 3.4 RAG Pipeline (Retrieval-Augmented Generation)

**Purpose**: Find and retrieve relevant code/knowledge for any query.

```
rag_pipeline.py          — Main RAG orchestrator (51K LOC)
rag_auto_reindex.py      — Automatic re-indexing on changes
rag_tenant_resolution.py — Multi-tenant RAG isolation
graph_rag_bridge.py      — Bridge between RAG and graph structures
```

### 3.5 Dream Engine (Counterfactual Reasoning)

**Purpose**: Generate "what if?" scenarios and consolidate insights.

```
counterfactual_engine.py  — Simulate alternative outcomes
consolidator.py           — Promote dreams into permanent knowledge
consolidation_lock.py     — Concurrency control for consolidation
gates.py                  — Gating mechanisms for dream quality
```

### 3.6 System Crystallizer (Build Engine)

**Purpose**: Transform Python source into optimized native binaries.

```
_orchestrator.py  — Build orchestration
_graph.py         — Dependency graph for build order
_synthesizer.py   — Cython synthesis pipeline
_populator.py     — __init__.py population
_static_data.py   — Static analysis data
_stress_tester.py — Build stress testing
_types.py         — Type definitions
```

### 3.7 Forge Intel (Evolutionary Intelligence)

**Purpose**: Gene-level neural network manipulation — the DNA of AI models.

```
Python layer (forge-intel-python/):
├── compression/
│   ├── ads_cft_compression.py        # AdS/CFT-inspired compression
│   ├── ads_cft_gpu_accelerator.py    # GPU-accelerated compression
│   └── sacred_compression.py         # Sacred geometry compression
├── consciousness/
│   ├── iit_calculator.py             # IIT Φ calculation
│   └── iit_v3_real.py                # IIT v3 full implementation
├── sacred_geometry/
│   ├── phi_pattern_recognition.py    # Golden ratio pattern detection
│   └── sacred_geometry_engine.py     # Geometry-based optimization
├── synthesis/
│   ├── directed_evolution_training.py # Directed evolution for training
│   ├── gene_synthesizer.py           # Gene synthesis operations
│   └── nucleus_evolution_bridge.py   # Bridge to Nucleus format
├── gene_evolution.py                 # Evolutionary gene algorithms
├── gene_pool.py                      # Population of gene variants
├── mutation.py                       # Mutation operators
├── neural_transplant.py              # Cross-model gene transfer
└── model_surgeon.py                  # Surgical model editing

Rust layer (crates/forge-intel/):
├── evo_pipeline.rs                   # Rust evolutionary pipeline
├── holographic_compress.rs           # Holographic compression
├── island_model.rs                   # Island model for evolution
├── neural_repair.rs                  # Neural network repair
├── noise_detector.rs                 # Gene noise detection
└── pareto.rs                         # Pareto-optimal selection
```

---

## 4. The ELM Build System

### 4.1 Modal Architecture

| Modal | Name | Responsibility |
|-------|------|---------------|
| **A** | Awareness | Detect hardware, verify substrate health, auto-recover |
| **E** | Evolution | Hash diff + semantic impact analysis → what changed? |
| **S** | Synthesis | Cython → C → .so compilation with GPU acceleration |
| **V** | Validation | Φ-coherence scoring, manifest sealing |
| **T** | Testing | Mirror Lattice execution, immune checkpoint |
| **O** | Observation | Real-time UI heartbeat, IPC telemetry |

### 4.2 ELM Flow

```
             ┌──────┐
             │ A    │ ← GPU? Docker? Ollama?
             │AWARE │
             └──┬───┘
                │
             ┌──▼───┐
             │ E    │ ← xxHash + DevBrain impact
             │EVOLVE│
             └──┬───┘
                │
             ┌──▼───┐
             │ S    │ ← Cython → GCC → .so (8 threads)
             │SYNTH │
             └──┬───┘
                │
           ┌────┴────┐
           │         │
        ┌──▼───┐  ┌──▼───┐
        │ V    │  │ T    │ ← Run mirror tests
        │VALID │◄─│TEST  │   Calculate Φ from results
        └──┬───┘  └──────┘
           │
        ┌──▼───┐
        │ O    │ ← Update UI heartbeat
        │OBSRV │
        └──────┘
```

---

## 5. Model Surgery Pipeline

The Rust core provides native-speed neural network editing:

### 5.1 Operations

| Operation | Description | Crate |
|-----------|------------|-------|
| **Decompose** | Break model into genes (attention heads, MLP blocks) | forge-core |
| **Visualize** | 3D gene space visualization | forge-ui |
| **Prune** | Remove low-importance parameters | forge-core |
| **Mutate** | Apply random or directed mutations | forge-core |
| **Transplant** | Transfer genes between models | forge-intel |
| **Distill** | Compress model while preserving behavior | forge-gpu |
| **Quantize** | Reduce precision (ternary, FP16, INT8) | forge-core |
| **Fuse** | Merge compatible model layers | forge-core |

### 5.2 Supported Formats

| Format | Read | Write | Purpose |
|--------|------|-------|---------|
| **GGUF** | ✅ | ✅ | llama.cpp ecosystem |
| **SafeTensors** | ✅ | ✅ | HuggingFace ecosystem |
| **Nucleus** | ✅ | ✅ | Arkheion native format |
| **Nucleus v3** | ✅ | ✅ | Enhanced with Φ metadata |
| **ArkTern** | ✅ | ✅ | Ternary-optimized format |
| **Mobile** | ✅ | ✅ | Mobile-optimized format |
| **Korus** | ✅ | ✅ | Legacy format support |

---

## 6. IPC and Integration

### 6.1 Communication Channels

```
┌──────────────┐         ┌──────────────┐
│  Rust Core   │◄──────►│  Python AI   │
│  (forge-*)   │  PyO3   │  (ai-core)   │
└──────┬───────┘         └──────┬───────┘
       │                        │
       │ D-Bus                  │ HTTP
       │                        │
┌──────▼───────┐         ┌──────▼───────┐
│  VS Code     │         │  Ollama      │
│  (MCP)       │         │  (LLM)       │
└──────────────┘         └──────────────┘
```

### 6.2 MCP Protocol

The Forge exposes tools via MCP for integration with AI assistants:

- `generate-text` — Generate code using local model
- `list-models` — Available models on Ollama
- `health-check` — Backend health status

---

## 7. Φ-Coherence System

### 7.1 What is Φ?

Φ (Phi) is the **Integrated Information Theory** metric adapted for code coherence. It measures how well-integrated a code module is with the rest of the system.

```python
Φ = (test_pass_rate) × (code_coverage) × min(assertion_density / 10.0, 1.0)
```

### 7.2 Φ Thresholds

| Φ Value | State | Action |
|---------|-------|--------|
| ≥ 0.9 | **STABLE** | Crystal sealed, no action needed |
| 0.6–0.9 | **EVOLVING** | Crystal valid but improving |
| 0.2–0.6 | **DECAYED** | Needs attention, tests failing |
| < 0.2 | **BLIND** | No tests, critical gap |
| 0.0 | **DEAD** | Module has no mirror, reject |

### 7.3 Golden Ratio Decay

The eviction policy uses the golden ratio for natural decay:

```
score = base_confidence × φ^(-age_days) × log₂(1 + hits) × feedback
```

Where φ = 1.618... (golden ratio). This ensures that frequently-used, recently-validated crystals persist while stale ones naturally decay.

---

## 8. Construction Roadmap

### Phase 1: Foundation (Current)
- [x] Copy and isolate all AI subsystems
- [x] Document architecture
- [ ] Strip Arkheion Forge-specific dependencies
- [ ] Create standalone pyproject.toml
- [ ] Verify Rust crates compile independently

### Phase 2: Core Engine
- [ ] Implement parametric coding pipeline
- [ ] Unify RAG + HDCache + Neural Mesh into single retrieval API
- [ ] Build code-to-embedding pipeline
- [ ] Implement code generation from embeddings

### Phase 3: Editor Integration
- [ ] Connect forge-editor to AI pipeline
- [ ] Implement AI chat with local model
- [ ] Build file explorer with semantic search
- [ ] Add inline code suggestions

### Phase 4: Self-Evolution
- [ ] Implement the full ELM cycle
- [ ] Add dream phase (counterfactual code generation)
- [ ] Build learning loop (improve from own outputs)
- [ ] Enable autonomous code repair

---

## 9. Design Principles

1. **Zero Cloud Dependency** — All AI runs locally (Ollama, ROCm)
2. **Parameter Space First** — Embeddings are the primary representation
3. **Self-Validating** — Every crystal contains its own immune system
4. **Incrementally Evolved** — Never rebuild what hasn't changed
5. **Hardware-Aware** — Automatically detects and uses GPU when available
6. **Format-Agnostic** — Read/write any model format
7. **Privacy-First** — No data leaves the machine
