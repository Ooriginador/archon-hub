# Rust Crate Reference

> Detailed documentation of each Rust crate in the workspace.

---

## Workspace Layout

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.9.0"
edition = "2024"
```

## Dependency Graph

```
                    ┌──────────────┐
                    │ arkheion-forge│  (root binary)
                    │    main.rs   │
                    │    cli.rs    │
                    └──────┬───────┘
                           │
         ┌────────┬────────┼────────┬──────────┐
         │        │        │        │          │
    ┌────▼───┐ ┌──▼───┐ ┌─▼────┐ ┌▼────────┐ ┌▼──────┐
    │forge-ui│ │forge-│ │forge-│ │forge-   │ │forge- │
    │        │ │brain │ │bridge│ │intel    │ │gpu    │
    └────┬───┘ └──┬───┘ └──┬───┘ └───┬─────┘ └──┬────┘
         │        │        │         │           │
         └────────┴────────┼─────────┘           │
                           │                     │
                    ┌──────▼───────┐             │
                    │  forge-core  │◄────────────┘
                    │  (foundation)│
                    └──────────────┘
                           │
                    ┌──────▼───────┐
                    │  forge-bank  │
                    │  (gene store)│
                    └──────────────┘
```

---

## forge-core

**Purpose**: Computational foundation — all tensor operations, model formats, and core abstractions.

### Key Dependencies
```toml
serde = "1.0"           # Serialization
ndarray = "0.16"        # N-dimensional arrays
sha2 = "0.10"           # Cryptographic hashing
byteorder = "1.5"       # Endian-aware I/O
flate2 = "1.0"          # Deflate compression
zstd = "0.13"           # Zstd compression
half = "2.4"            # FP16 support
rayon = "1.10"          # Parallel iteration
memmap2 = "0.9"         # Memory-mapped files
rand = "0.8"            # Random number generation
sled = "0.34"           # Embedded database
```

### Architecture
```
forge-core/src/
├── lib.rs              # Public API: Gene, Genome, Ops, Formats, Codec
├── ops.rs              # Matrix ops: matmul, softmax, layer_norm, gelu
├── simd.rs             # SIMD-optimized inner loops (AVX2, NEON)
├── gene.rs             # Gene struct: represents one neural layer
├── genome.rs           # Genome struct: collection of genes = full model
├── lattice.rs          # Lattice: spatial organization of genes
├── backward.rs         # Gradient computation via chain rule
├── inference.rs        # Forward-pass inference engine
├── training.rs         # Training loop with optimizer
├── tokenizer.rs        # BPE tokenizer
├── codec/
│   ├── mod.rs          # Codec trait + registry
│   └── trit.rs         # Ternary (2-bit) codec
├── formats/
│   ├── mod.rs          # Format trait + auto-detection
│   ├── gguf.rs         # GGUF: read/write llama.cpp models
│   ├── safetensors.rs  # SafeTensors: HuggingFace format
│   ├── nucleus.rs      # Nucleus v1/v2: native format
│   ├── nucleus_v3.rs   # Nucleus v3: with Φ metadata
│   ├── arktern.rs      # ArkTern: ternary-optimized
│   ├── mobile.rs       # Mobile: quantized for edge
│   └── legacy.rs       # Legacy format (formerly Korus)
├── coherence.rs        # Φ-coherence calculation
├── mutation_strategy.rs # Mutation operators: gaussian, uniform, directed
├── mutation_log.rs     # Immutable log of all mutations
├── synthesizer.rs      # Gene synthesis from components
├── distillation.rs     # Knowledge distillation
├── curriculum.rs       # Curriculum learning
├── cross_training.rs   # Multi-model cross-training
├── dual_nucleus.rs     # Dual-nucleus architecture
├── empowerment.rs      # Empowerment metric
├── elm_gene.rs         # ELM-specific gene extension
├── resilience.rs       # Fault tolerance
├── runtime_learning.rs # Online learning
├── scale_calibration.rs # Scale calibration
├── frequency_bands.rs  # Frequency analysis
├── brain_predictor.rs  # Prediction module
├── corpus.rs           # Training corpus
├── versioning.rs       # Version management
├── metrics_export.rs   # Metrics export
├── marketplace.rs      # Gene marketplace
└── tracer.rs           # Execution tracing
```

---

## forge-gpu

**Purpose**: GPU acceleration via dynamic library loading (ROCm HIP, CUDA, OpenCL).

### Key Design
- Uses `libloading` for runtime GPU library discovery
- Falls back to CPU if no GPU available
- Supports ROCm (AMD) as primary target

### Architecture
```
forge-gpu/src/
├── lib.rs              # GPU engine selection + fallback
├── gpu_engine.rs       # Core dispatch: CPU ↔ GPU
├── inference.rs        # GPU inference kernels
├── training_gpu.rs     # GPU training with gradient accumulation
├── backward_gpu.rs     # GPU backpropagation
├── distill_gpu.rs      # GPU distillation
├── renderer.rs         # GPU visualization rendering
├── training_advisor.rs # Auto-tune batch size, learning rate
└── tests/
    ├── mod.rs
    └── test_gpu_lib_loading.rs
```

---

## forge-brain

**Purpose**: High-level AI reasoning — analysis, planning, surgery, execution.

### Architecture
```
forge-brain/src/
├── lib.rs           # Brain API
├── analyzer.rs      # Deep model analysis (entropy, sparsity, dead neurons)
├── planner.rs       # Multi-step intervention planning
├── executor.rs      # Plan execution with rollback
├── surgery.rs       # Surgical model editing (prune, graft, transplant)
├── think.rs         # Extended thinking / chain-of-thought
├── diagnosis.rs     # Problem diagnosis
├── causal.rs        # Causal reasoning
├── evolution.rs     # Evolutionary optimization
├── genesis.rs       # Create models from scratch
├── memory.rs        # Working memory for reasoning
├── phi.rs           # Φ-integration with coherence
├── empowerment.rs   # Empowerment-based decisions
└── prompter.rs      # Dynamic prompt construction
```

---

## forge-bridge

**Purpose**: Inter-process communication and system integration.

### Architecture
```
forge-bridge/src/
├── lib.rs              # Bridge API
├── bridge.rs           # Core bridge abstraction
├── dbus_client.rs      # D-Bus client for system integration
├── dbus_server.rs      # D-Bus server exposing forge
├── devbrain.rs         # DevBrain MCP client
├── nucleus.rs          # Nucleus format bridge
├── resonance.rs        # Resonance field bridge
├── runtime_feedback.rs # Runtime performance feedback
├── training_stream.rs  # Streaming training data
├── security.rs         # Security layer (encryption, auth)
├── notify.rs           # Desktop notifications
├── kernel.rs           # Kernel-level bridge
├── huam_client.rs      # HUAM client
└── types.rs            # Shared types
```

---

## forge-editor

**Purpose**: Native code editor with AI-powered features.

### Features
- Syntax highlighting for Python, Rust, TypeScript, YAML, TOML
- AI chat panel connected to local LLM
- File explorer with semantic search
- Multi-tab editing
- Embedded terminal
- Gene recombinator UI

---

## forge-ui

**Purpose**: egui-based native desktop application.

### Features
- 3D gene space visualization
- Model loading/saving
- Gene browser with layer inspection
- Entropy/sparsity/Φ metrics display
- Surgery controls (prune, mutate, amputate)
- Mutation history with undo
- File explorer integration

---

## forge-bank

**Purpose**: Gene storage, cataloging, and retrieval system.

### Capabilities
- Store individual genes extracted from models
- Catalog with metadata (source model, layer, performance metrics)
- Similarity search across gene bank
- Lineage tracking (where did this gene come from?)
- Gene synthesis (combine genes to create new ones)

---

## forge-mcp

**Purpose**: MCP protocol implementation.

### Tools Exposed
- Model loading and inspection
- Gene extraction and manipulation
- Coherence checking
- Format conversion

---

## forge-python

**Purpose**: PyO3 bindings for Python integration.

### Exposed Functions
- Tensor operations callable from Python
- Model format I/O
- Gene manipulation
- Coherence calculation
