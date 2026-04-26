# 🏗️ Architecture

> ARKHEION Forge v0.9.0 — Internal Architecture & Design Decisions
>
> **83,572 LOC Rust** | **8 crates** | **65 MCP tools** | **~850 tests**

---

## Workspace Overview

ARKHEION Forge is a Cargo workspace (`resolver = "2"`, edition `"2024"`) with 8 specialized crates:

| Crate | LOC | Tests | Role |
|-------|-----|-------|------|
| **forge-core** | 14,748 | 209 | SSOT constants, Gene primitives, formats, ops |
| **forge-intel** | 10,692 | 120 | φ calculations, analysis, gene diagnostics |
| **forge-bank** | 4,050 | 9 | Persistent gene bank (sled), observatory |
| **forge-brain** | 20,362 | 352 | AI surgeon: diagnosis → plan → execute pipeline |
| **forge-gpu** | 7,316 | 71 | GPU rendering, HIP/ROCm integration |
| **forge-bridge** | 5,193 | 77 | Python ↔ Rust bridge (PyO3/pyo3-macros) |
| **forge-ui** | 14,294 | — | Native desktop UI (egui/eframe) |
| **forge-mcp** | 4,953 | 108 | MCP server — 65 tools over JSON-RPC 2.0 |
| *binary* | 348 | — | `main.rs` → `eframe::run_native()` |

## Crate Dependency Graph

```text
arkheion-forge (binary)
├── forge-ui
│   ├── forge-core
│   └── forge-gpu → forge-core
├── forge-mcp (standalone MCP server binary)
│   ├── forge-core
│   ├── forge-intel → forge-core
│   ├── forge-bank → forge-core
│   ├── forge-brain → forge-core, forge-intel
│   └── forge-gpu → forge-core
├── forge-bridge (PyO3)
│   ├── forge-core
│   ├── forge-intel → forge-core
│   └── forge-bank → forge-core
├── eframe (egui)
├── tracing / tracing-subscriber
└── anyhow
```

```text
┌─────────────────────────────────────────────────────────────┐
│                   arkheion-forge workspace                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────────┐    │
│  │  forge-mcp  │  │  forge-ui   │  │  forge-bridge    │    │
│  │  65 tools   │  │  egui app   │  │  PyO3 bindings   │    │
│  │  JSON-RPC   │  │  panels     │  │  numpy interop   │    │
│  └──────┬──────┘  └──────┬──────┘  └────────┬─────────┘    │
│         │                │                   │              │
│  ┌──────▼──────┐  ┌──────▼──────┐  ┌────────▼─────────┐   │
│  │ forge-brain │  │  forge-gpu  │  │   forge-bank     │   │
│  │ AI surgeon  │  │  renderer   │  │   sled + obs.    │   │
│  │ 14 modules  │  │  HIP/ROCm  │  │   persistent     │   │
│  └──────┬──────┘  └──────┬──────┘  └────────┬─────────┘   │
│         │                │                   │              │
│  ┌──────▼──────┐         │                   │              │
│  │ forge-intel │         │                   │              │
│  │ φ analysis  │         │                   │              │
│  └──────┬──────┘         │                   │              │
│         │                │                   │              │
│  ┌──────▼────────────────▼───────────────────▼──────────┐  │
│  │                    forge-core                         │  │
│  │  Gene, GenePool, ops, formats, constants (PHI SSOT)  │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Constants — Single Source of Truth (SSOT)

All φ-related constants are defined **once** in `forge-core/src/lib.rs` and
re-exported by downstream crates:

```rust
// forge-core/src/lib.rs (SSOT — never duplicate these)
pub const PHI:     f64 = 1.618_033_988_749_895;
pub const PHI_INV: f64 = 0.618_033_988_749_895;
```

| Consumer | Import |
|----------|--------|
| forge-intel | `pub use forge_core::PHI; pub use forge_core::PHI_INV as PHI_INVERSE;` |
| forge-bridge | `pub use forge_core::PHI;` |
| forge-brain/causal | `pub use forge_core::PHI_INV as PHI_INVERSE;` |

Derived constants (e.g. `PHI_SQUARED = PHI * PHI`) are computed from the SSOT.

## MCP Server — 65 Tools

The `forge-mcp` crate implements a **Model Context Protocol** server (JSON-RPC 2.0
over stdio, protocol version `2024-11-05`) exposing 65 tools organized into categories:

| Category | Tools | Examples |
|----------|-------|---------|
| Core Ops (34) | Pool loading, gene inspection, editing | `forge_load`, `forge_prune`, `forge_mutate`, `forge_transplant` |
| Brain (16) | AI-driven analysis and surgery | `forge_brain_diagnose`, `forge_brain_plan`, `forge_brain_execute`, `forge_brain_fuse` |
| Bank (12) | Persistent gene bank management | `forge_bank_import`, `forge_bank_query`, `forge_bank_evolve`, `forge_bank_observe` |
| GPU (3) | Hardware status and computation | `forge_gpu_status`, `forge_gpu_phi`, `forge_gpu_holographic` |

**Dispatch**: `handlers.rs` routes `tools/call` requests → handler functions. Each handler
extracts args from `serde_json::Value`, calls the appropriate crate API, and returns
MCP-formatted `Content` responses.

**Testing**: 108 passing tests validate tool definitions, JSON schemas, dispatch alignment,
and handler behavior. Bank tools that require the real 1.7GB gene bank and GPU tools
that may SIGABRT (HIP runtime) are skipped in the alignment test.

## Forge-Brain — AI Surgeon Pipeline

The brain (`20,362 LOC`, 14 modules, 352 tests) implements a **diagnose → plan → execute**
pipeline for autonomous model surgery:

```text
GenePool
  │
  ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Diagnose   │────▶│     Plan     │────▶│   Execute    │
│              │     │              │     │              │
│ phi analysis │     │ step builder │     │ versioned    │
│ weak genes   │     │ brain decide │     │ snap/rollback│
│ entropy map  │     │ cost/benefit │     │ GA optimize  │
└──────────────┘     └──────────────┘     └──────────────┘
                                              │
                                              ▼
                                         ┌──────────┐
                                         │ Validate │
                                         │ phi_after│
                                         │ compare  │
                                         └──────────┘
```

Key modules: `diagnosis.rs`, `planner.rs`, `executor.rs`, `evolution.rs`,
`surgery.rs` (transplant/fuse), `causal.rs`, `phi.rs`, `observatory.rs`.

### Multi-Model Fusion

Brain supports fusing multiple gene pools with strategies:
- **BestOfEach** — select highest-quality gene per layer
- **Interpolate { alpha }** — weighted average in trit space
- **BrainDecided** — AI chooses per-gene strategy

## Data Flow

### Loading a Model

```text
File on disk (.arktern / .nucleus / .gguf)
        │
        ▼
  ┌─────────────────────┐
  │  Format Parser       │
  │  nucleus.rs          │  ← native format
  │  arktern.rs          │  ← legacy format
  │  (gguf via convert)  │  ← import path
  │                      │
  │  Decompress (zstd    │
  │  or zlib) → Parse    │
  │  header → Read       │
  │  tensor index →      │
  │  Extract packed data │
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  GenePool            │
  │                      │
  │  HashMap<gene_id,    │
  │          Gene>       │
  │                      │
  │  Each Gene has:      │
  │  - packed_data       │
  │  - shape, domain     │
  │  - phi_quality       │
  │  - SHA-256 gene_id   │
  └──────────┬──────────┘
             │
      ┌──────┼──────────────┐
      ▼      ▼              ▼
┌─────────┐ ┌────────┐ ┌──────────┐
│forge-ui │ │forge-  │ │forge-mcp │
│ panels  │ │ gpu    │ │ 65 tools │
│ egui    │ │ render │ │ JSON-RPC │
└─────────┘ └────────┘ └──────────┘
```

### Editing a Gene

```text
User clicks "✂️ Prune" (ratio=0.1)   ── or ──   MCP: forge_prune(file, domain)
        │                                              │
        └──────────────────┬───────────────────────────┘
                           ▼
                 ┌──────────────────┐
                 │  ops::prune()    │
                 │                  │
                 │  1. Unpack trits │
                 │  2. Count nnz    │
                 │  3. Zero out 10% │
                 │     of non-zero  │
                 │  4. Repack       │
                 │  5. Recompute ID │
                 └────────┬─────────┘
                          │
                          ▼
                 ┌──────────────────┐
                 │  EditResult      │
                 │  gene_id         │
                 │  before_nnz      │
                 │  after_nnz       │
                 │  before/after_   │
                 │  entropy         │
                 └──────────────────┘
```

### Gene Bank Pipeline

```text
.nucleus files on disk
        │
        ▼
  ┌──────────────────┐
  │  forge_bank_     │
  │  import()        │
  │  scan()          │
  └────────┬─────────┘
           │
           ▼
  ┌──────────────────┐     ┌──────────────────┐
  │  Gene Bank       │────▶│  Observatory     │
  │  (sled index)    │     │  probe_all()     │
  │  ~/.arkheion/    │     │  health reports  │
  │  gene_bank/      │     │  taxonomy        │
  └────────┬─────────┘     └──────────────────┘
           │
    ┌──────┼──────┐
    ▼      ▼      ▼
 query  evolve  synthesize
 export  tag    recombine
```

### Saving

```text
GenePool in memory
        │
        ▼
  ┌──────────────────────┐
  │  nucleus::save()     │
  │                      │
  │  1. Write ARKUNN02   │
  │  2. Version (u16)    │
  │  3. Gene count       │
  │  4. For each gene:   │
  │     - len-prefixed   │
  │       strings        │
  │     - shape array    │
  │     - packed bytes   │
  │     - JSON sources   │
  │     - JSON metadata  │
  │  5. Zlib-compress    │
  │     architectures    │
  │  6. Zstd outer wrap  │
  └──────────┬───────────┘
             │
             ▼
       .nucleus file
```

## Design Decisions

### 1. Why Ternary?

Ternary quantization (`{-1, 0, +1}`) is the most extreme useful quantization. It enables:

- **5 trits per byte** — near-optimal packing (log₂(3⁵) = 7.92 bits, using 8)
- **Discrete editing** — no floating-point precision issues
- **Gene identity** — SHA-256 of packed bytes is stable and deterministic
- **Surgical tools** — prune/mutate operations are simple bit-level ops

### 2. Why Gene Metaphor?

Biological metaphor maps cleanly to model surgery:

| Biology | Forge | Meaning |
| ------- | ----- | ------- |
| Gene | Tensor layer | Discrete unit of "knowledge" |
| Genome | GenePool | Complete model |
| Mutation | ops::mutate() | Random perturbation |
| Pruning | ops::prune() | Remove weak connections |
| Transplant | ops::transplant() | Move gene between models |
| Fusion | brain::plan_fusion() | Merge multiple models |
| Amputation | ops::amputate() | Remove entire layer |
| Evolution | bank::evolve_from_bank() | GA-based improvement |
| Fragile | is_fragile() | Norm/Embed — dangerous to edit |

### 3. Why 8 Crates?

```text
forge-core    ← Zero dependencies on upper layers. Library-only.
forge-intel   ← φ analysis, diagnostics. Depends only on core.
forge-bank    ← Persistent storage + observatory. Depends on core.
forge-brain   ← AI surgeon. Depends on core + intel.
forge-gpu     ← GPU rendering + HIP compute. Depends on core.
forge-bridge  ← Python interop (PyO3). Depends on core + intel + bank.
forge-ui      ← egui/eframe desktop app. Depends on core + gpu.
forge-mcp     ← MCP JSON-RPC server. Depends on all except ui/bridge.
```

This separation enables:
- CLI tools using only `forge-core`
- MCP server for AI agent integration (no UI)
- Python library via `forge-bridge` (no UI)
- Web UI via `forge-core` + `forge-gpu` + WebAssembly
- Testing without UI dependencies

### 4. Why eframe/egui over Tauri/GTK?

- **Pure Rust** — no JavaScript, no web runtime
- **Immediate mode** — no retained widget state complexity
- **Cross-platform** — same code for Linux, macOS, Windows
- **Lightweight** — ~5MB binary, instant startup
- **glow backend** — OpenGL ES, works on AMD ROCm systems

### 5. Why φ (Golden Ratio)?

The golden ratio φ = 1.618033988749895 is used throughout ARKHEION as:

- **Quality metric** — gene `phi_quality` score
- **Distribution analysis** — optimal weight distributions follow φ patterns
- **Architecture constant** — SSOT in `forge-core/src/lib.rs`
- **Consciousness threshold** — `Φ > 0.5` for integrated states (IIT)

```rust
pub const PHI: f64 = 1.618033988749895;     // forge-core SSOT
pub const PHI_INV: f64 = 0.618033988749895; // forge-core SSOT
```

### 6. Gene ID = SHA-256

Each gene's identity is derived from its content:

```text
gene_id = SHA-256(packed_data)[:16 hex chars]
```

This means:
- Same data → same ID (content-addressable)
- Editing data → new ID (immutable identity)
- Deduplication is automatic (shared genes detected by ID collision)

### 7. Why MCP?

Model Context Protocol enables AI agents (Claude, Copilot) to **directly operate
on gene pools** without custom integration:

- Standard JSON-RPC 2.0 over stdio — universal transport
- 65 tools with JSON schemas — fully discoverable
- Brain tools enable autonomous model surgery from natural language
- Bank tools enable persistent model management across sessions

## Error Handling Strategy

```text
forge-core:    thiserror → Typed errors (NucleusError, ArkternError)
forge-brain:   ForgeError → Unified error enum across brain operations
forge-mcp:     JSON-RPC error codes → -32602 (invalid params), -32603 (internal)
forge-ui:      Match on Result → Display in status bar
main.rs:       anyhow → Catch-all for startup failures
```

Errors never panic. All operations return `Result<T, Error>`.

## Resilience Infrastructure (v0.9.1)

- **Memory pre-flight**: validates RAM before model loading
- **Circuit breakers**: I/O (3 failures/30s), GPU (5/10s), Python (3/60s)
- **Thermal monitoring**: polls `rocm-smi`, auto-pauses at GPU ≥90°C
- **Operation degradation**: Full → Reduced → Minimal → Emergency
- **PhiDebouncer**: stabilizes φ oscillations during thermal events

## Thread Safety

Desktop UI is **single-threaded** (egui event loop on main thread).
MCP server runs on **tokio async runtime** (single worker for stdio).

Background operations:
- `rayon` for parallel gene operations in forge-core
- Async channels for UI ↔ background communication
- GPU compute via HIP runtime (AMD ROCm)

---

*Next: [FORMATS.md](FORMATS.md) — Binary format specifications*
