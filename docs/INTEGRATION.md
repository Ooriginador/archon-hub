# 🔗 Integration Guide

> Full pipeline: GGUF → .arktern → Forge → .nucleus, Python ecosystem, GPU modules

---

## End-to-End Pipeline

ARKHEION Forge is the **visual editor** stage in a larger model processing pipeline:

```text
┌──────────────┐     ┌──────────────────────┐     ┌───────────────┐
│  Source Model │     │  Python Pipeline     │     │  ARKHEION     │
│  (GGUF/HF)   │────▶│                      │────▶│  Forge        │
│              │     │  Ternary Converter   │     │  (Rust GUI)   │
│  Llama 3.2   │     │  Optimizer           │     │               │
│  Mistral     │     │  Neural Rewiring     │     │  Edit genes   │
│  Qwen        │     │                      │     │  Save .nucleus│
└──────────────┘     └──────────────────────┘     └───────┬───────┘
                                                          │
                              ┌────────────────────────────┘
                              ▼
                     ┌───────────────────────┐
                     │  Nucleus Ecosystem    │
                     │                      │
                     │  GPU Acceleration    │
                     │  Inference Engine    │
                     │  Model Serving       │
                     └───────────────────────┘
```

---

## Stage 1: Ternary Conversion (Python)

### Script: `llama_ternary_converter.py`

Converts standard model formats (GGUF, SafeTensors) into `.arktern`:

```bash
python llama_ternary_converter.py \
    --input models/llama-3.2-1b.gguf \
    --output /tmp/arktern_test/llama3.2-1b_ternary.arktern \
    --phi-optimization \
    --ads-cft-compression
```

### What It Does

1. **Load model** from GGUF/SafeTensors
2. **Quantize** each tensor to ternary `{-1, 0, +1}` using threshold-based quantization
3. **Pack** trits at 5-per-byte
4. **Optionally apply** zlib compression on high-sparsity tensors
5. **Optionally apply** AdS/CFT holographic compression (experimental)
6. **Write** `.arktern` file with 64-byte header + metadata + index + data

### Conversion Results (Llama 3.2 1B)

```text
Original GGUF:    770 MB
Output .arktern:  172 MB  (4.5× compression)
Round-trip accuracy: 100%
Tensors:
  zlib-compressed: 67
  AdS/CFT:          0  (smart selection — none qualified)
φ consciousness tracking:
  109 layers analyzed
  φ_max = 1.894
  φ_avg = 0.131
```

---

## Stage 2: Optimization (Python)

### Script: `llama_ternary_optimizer.py`

Post-conversion refinement:

```bash
python llama_ternary_optimizer.py \
    --input /tmp/arktern_test/llama3.2-1b_ternary.arktern \
    --output /tmp/arktern_test/llama3.2-1b_optimized.arktern
```

### Techniques

| Technique | Description |
| --------- | ----------- |
| φ-Enhanced Pruning | Prune using golden ratio thresholds |
| Bio-Synthetic Repair | Reconstruct damaged parameters |
| Structured FFN Pruning | Prune MLP rows/columns as units |

---

## Stage 3: Neural Rewiring (Python)

### Script: `llama_neural_rewiring.py`

Advanced model surgery in Python:

```bash
python llama_neural_rewiring.py \
    --input /tmp/arktern_test/llama3.2-1b_ternary.arktern
```

### Techniques

| Technique | Description |
| --------- | ----------- |
| Cross-Layer Transplant | Move attention patterns between layers |
| Spectral Redistribution | SVD-based weight redistribution |
| Neural Pathway Fusion | Merge redundant pathways |
| Adaptive Threshold Recalibration | Dynamic quantization thresholds |

---

## Stage 4: ARKHEION Forge (Rust)

The visual editor reads `.arktern` or `.nucleus` and provides interactive editing:

```bash
cd arkheion-forge
cargo run --release
# → Open file dialog → select .arktern
```

### Input Formats

| Format | Extension | Source |
| ------ | --------- | ------ |
| Arktern | `.arktern` | Python converter pipeline |
| Nucleus | `.nucleus` | Forge save / Nucleus tools |

### Output Format

| Format | Extension | Destination |
| ------ | --------- | ----------- |
| Nucleus | `*_edited.nucleus` | Nucleus ecosystem / inference |

---

## Stage 5: Nucleus Ecosystem (Python)

After editing in Forge, `.nucleus` files integrate with the broader ARKHEION nucleus pipeline.

### Key Python Modules

| Module | Path | Description |
| ------ | ---- | ----------- |
| `ModelSurgeon` | `src/nucleus/` | Programmatic gene editing |
| `NeuralTransplantor` | `src/nucleus/` | Cross-model gene transplantation |
| `NeuralEtcher` | `src/nucleus/` | Fine-grained parameter editing |
| `GeneNoiseDetector` | `src/nucleus/` | Detect noisy/damaged genes |
| `GeneEvolution` | `src/nucleus/` | Evolutionary gene optimization |
| `GeneticTransfusionV2` | `src/nucleus/` | Multi-model gene merging |
| `GeneSynthesizer` | `src/nucleus/` | Generate new genes from scratch |
| `ComputationGene` | `src/nucleus/` | Executable gene abstraction |
| `NucleusManager` | `src/nucleus/` | Gene pool management |

### Python Module Counts

```text
Total Python modules:      221 scripts
  Nucleus modules:          91
  Ternary modules:          19
  Compression modules:      35
  Quantum modules:         111
  Consciousness modules:    26
  Bio-synthetic modules:    63
  LLM modules:              17
```

---

## GPU Acceleration

ARKHEION includes compiled GPU kernels that accelerate the pipeline:

### Compiled GPU Libraries (5 .so files)

| Library | HIP Kernels | Purpose |
| ------- | ----------- | ------- |
| `libwave32_matmul_gpu.so` | wave32_matmul, ternary_matmul | Matrix operations |
| `libconsciousness_phi_gpu.so` | phi_calculator, iit_integration | φ computation |
| `libholographic_compression_gpu.so` | ads_cft_bulk_encoding, boundary_extraction | Holographic compression |
| `libquantum_gates_gpu.so` | quantum_gates_rdna2, circuit_simulation | Quantum gates |
| `libunified_memory_manager.so` | memory_pool_alloc, cache_management | Memory management |

### GPU Hardware

```text
GPU:       AMD Radeon RX 6600M (RDNA2)
Backend:   ROCm / HIP
Framework: PyTorch 2.4.1+rocm6.0
Kernels:   20 HIP active, 8 CUDA (compatibility)
```

### Total Compiled Modules

```text
38 unique .so files across the project
Including: pybind11 bindings, GPU kernels, Python C extensions
```

---

## Data Flow Diagram

```text
                    Python Ecosystem
                    ════════════════

GGUF ─────┐
SafeTensors┤
HuggingFace┘
     │
     ▼
┌─────────────────────┐
│ llama_ternary_       │
│ converter.py         │
│                      │
│ Quantize → Pack →   │
│ Compress → Write    │
└──────────┬──────────┘
           │ .arktern
           ▼
┌─────────────────────┐     ┌─────────────────────┐
│ llama_ternary_       │     │ llama_neural_        │
│ optimizer.py         │────▶│ rewiring.py          │
│                      │     │                      │
│ φ-Prune             │     │ Transplant           │
│ Bio-Repair          │     │ Spectral             │
│ FFN-Prune           │     │ Pathway Fusion       │
└──────────┬──────────┘     └──────────┬──────────┘
           │ .arktern                   │ .arktern
           └─────────┬─────────────────┘
                     │
                     ▼
              ════════════════
              Rust Ecosystem
              ════════════════
                     │
           ┌─────────▼──────────┐
           │  ARKHEION Forge    │
           │  (Rust / egui)     │
           │                    │
           │  Visual editing    │
           │  Prune / Mutate    │
           │  Transplant        │
           │  Gene Space viz    │
           └─────────┬──────────┘
                     │ .nucleus
                     ▼
              ════════════════
              GPU Ecosystem
              ════════════════
                     │
           ┌─────────▼──────────┐
           │  Nucleus Pipeline  │
           │  + GPU Kernels     │
           │                    │
           │  Inference         │
           │  Fine-tuning       │
           │  Model serving     │
           └────────────────────┘
```

---

## API for Programmatic Use

`forge-core` can be used as a Rust library without the UI:

```rust
use forge_core::formats::{arktern, nucleus};
use forge_core::gene::GenePool;
use forge_core::ops;

// Load .arktern
let file = arktern::load("model.arktern").unwrap();
let mut pool = arktern::to_gene_pool(&file);

// Get stats
let stats = ops::pool_stats(&pool);
println!("Genes: {}, Params: {}", stats.gene_count, stats.total_params);

// Prune all MLP layers by 10%
for gene in pool.by_domain(GeneDomain::Mlp) {
    if let Some(g) = pool.get_mut(&gene.gene_id) {
        ops::prune(g, 0.1);
    }
}

// Save as .nucleus
nucleus::save(&pool, "model_pruned.nucleus").unwrap();
```

---

## File Locations in Workspace

```text
ARKHEION_AGI_2.0/
├── arkheion-forge/              # This project (Rust)
├── src/nucleus/                 # Nucleus Python modules (91)
├── src/core/quantum/            # Quantum processing (111 modules)
├── src/core/consciousness/      # IIT consciousness (26 modules)
├── src/advanced/bio_synthetic/  # Bio-synthetic intelligence (63 modules)
├── data/models/                 # Saved models
├── checkpoints/                 # Training checkpoints
│   ├── arkheion_ternary/        # Ternary model checkpoints
│   └── ternary_nucleus/         # Nucleus-format checkpoints
└── arkheion_unified_gpu/        # GPU kernel builds
```

---

*Next: [ROADMAP.md](ROADMAP.md) — Future phases*
