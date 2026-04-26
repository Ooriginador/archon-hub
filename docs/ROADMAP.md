# 🚀 ARKHEION Forge — Roadmap Completo

> **Versão**: 2.2.0 | **Data**: 2026-02-09
> **De editor mecânico → sistema inteligente de rewriting neural**

---

## 📍 Estado Atual: v0.13.0 (Neural Repair Active)

### Forge Core (Rust) — 7.785 linhas, 92 testes ✅

| Módulo | Linhas | Função |
|--------|--------|--------|
| `gguf.rs` | 1.492 | Import/Export GGUF v3 (7 quant types) |
| `nucleus_v3.rs` | 1.243 | Formato .nucleus V3 mmap zero-copy |
| `ops.rs` | 999 | 21 operações cirúrgicas com undo |
| `inference.rs` | 691 | Perplexity estimation + A/B compare |
| `safetensors.rs` | 706 | Export SafeTensors (HuggingFace) |
| `versioning.rs` | 546 | Git-like delta snapshots |
| `gene.rs` | 319 | Gene struct + domain classification |
| `nucleus.rs` | 294 | Formato .nucleus V2 (ARKUNN02) |
| `trit.rs` | 180 | Codec ternário 5 trits/byte |
| `arktern.rs` | 181 | Formato .arktern legado |

### Forge Intel (Rust + PyO3) — 5.635 linhas, 53 testes, 25+ módulos Python ✅ 🆕

| Módulo Rust | Linhas | Função |
|-------------|--------|--------|
| `forge_cli.rs` | 1.724 | CLI 20 comandos (smart-prune, evolve, repair) |
| `neural_repair.rs` | 891 | 4 métodos de reparo neural post-surgery |
| `smart_evolve.rs` | 767 | Directed evolution — GA engine ternário |
| `noise_detector.rs` | 629 | 5 tipos de ruído, auto_clean, PyO3 |
| `smart_ops.rs` | 586 | Smart prune (4 steps pipeline) |
| `phi_calculator.rs` | 417 | IIT φ quality scoring (pure Rust fast path) |
| `taxonomy.rs` | 308 | Domain/SubType/Quality classification |
| `lib.rs` | 278 | IntelEngine (PyO3 bootstrap, wrapper API) |
| `error.rs` | 35 | IntelError enum (7 variantes) |

**Python Package (`arkheion_intel` v0.2.0)**: 18,654 LOC, 25+ módulos standalone, 62/62 testes ✅

### Forge GPU (Rust FFI) — 1.891 linhas

| Feature | Status |
|---------|--------|
| FFI bridge `libternary_forge_gpu.so` | ✅ 24 símbolos |
| Context-aware API (zero-alloc) | ✅ OPT-5 fused pipeline |
| Wave32 MatMul (FP32 + FP16) | ✅ via `libwave32_matmul_gpu.so` |
| φ quality scoring GPU | ✅ via `libconsciousness_phi_gpu.so` |
| Pinned memory DMA | ✅ `PinnedBuffer` |
| Benchmark suite | ✅ 8 seções |

### Forge UI (egui) — 3D viz, panels, cross-model transplant

| Feature | Status |
|---------|--------|
| 3D gene space (orbit, zoom, pick) | ✅ Phase 3 |
| Knowledge ops (inject, synthesis, transplant) | ✅ Phase 4 |
| Undo/redo, batch ops, compare | ✅ Phase 2 |
| Brain UX panels (dashboard, plan, exec, journal, diff) | ✅ Phase G |
| Multi-hypothesis + adaptive exec UI | ✅ Phase H |
| Memory Browser (experiences, domain insights) | ✅ Phase H |

> Detalhes completos das fases G/H: ver [docs/FORGE_UX_ROADMAP.md](FORGE_UX_ROADMAP.md).

### Pipeline Validado End-to-End

```
GGUF (770 MB) → convert → .nucleus (236 MB, 3.3×)
  → prune/mutate/transplant → inference → ab-compare
  → export-gguf (roundtrip) → export-safetensors
  → diagnose (noise) → taxonomy → phi (φ quality)
  168/168 testes ✅ | 4 crates | 20 CLI commands
```

---

## 🧠 O Gap: Inteligência Python Não Integrada

O Forge tem a **infraestrutura** (rápida, segura, testada).
O Python tem a **inteligência** (profunda, completa, 4.376 classes).

### Inventário Python Disponível (Verificado)

```
src/arkheion/
├── nucleus/          127 arquivos, 391 classes  ← CORE: cirurgia inteligente
├── consciousness/     21 arquivos               ← IIT φ calculator
├── compression/       12 arquivos               ← AdS/CFT holográfico
├── training/          28 arquivos               ← Directed evolution
├── sacred_geometry/    4 arquivos               ← φ optimization
├── bio_synthetic/      3 arquivos               ← Evolutionary engine
├── inference/          2 arquivos               ← Ternary inference
├── llm/               15 arquivos               ← LLM bridge
└── quantum/           15+ arquivos              ← Quantum processing
```

### GPU .so Compilados (Verificado)

```
arkheion_unified_gpu/build/
├── libwave32_matmul_gpu.so           6 símbolos  (matmul GPU)
├── libconsciousness_phi_gpu.so       9 símbolos  (φ IIT GPU)
├── libholographic_compression_gpu.so 6 símbolos  (AdS/CFT GPU)
├── libquantum_gates_gpu.so          25 símbolos  (quantum GPU)
├── libunified_memory_manager.so     24 símbolos  (memory GPU)
└── _gpu_core.cpython-312.so         13 símbolos  (Python bindings)

src/
├── arkheion_ternary_ops.so          20 símbolos  (ternary GPU)
├── arkheion_sacred_native.so         C++         (sacred geometry)
├── arkheion_nucleus_bridge.so        pybind11    (nucleus C++ bridge)
└── (+ 26 outros módulos compilados)
```

---

## 🗺️ ROADMAP DE INTEGRAÇÃO

### Arquitetura de Integração

```
┌─────────────────────────────────────────────────────────────┐
│                    FORGE CLI / GUI                           │
│                    (Rust, egui)                              │
├───────────┬───────────┬─────────────┬───────────────────────┤
│ forge-core│ forge-gpu │ forge-intel │ forge-ui              │
│ (formats) │ (FFI .so) │ (PyO3 🆕)  │ (visualization)       │
├───────────┴───────────┴─────────────┴───────────────────────┤
│                    INTEGRATION LAYER                         │
├──────────────────┬──────────────────┬───────────────────────┤
│  C/HIP .so       │  Python (PyO3)   │  Standalone scripts   │
│  ─────────       │  ──────────      │  ──────────────       │
│  wave32_matmul   │  GeneNoiseDetect │  llama_ternary_conv   │
│  phi_gpu         │  GeneEvolution   │  llama_neural_rewire  │
│  holographic     │  GeneSynthesizer │  llama_optimizer      │
│  ternary_ops     │  GeneTaxonomy    │  finetune_ternary     │
│  quantum_gates   │  DeltaCompressor │                       │
│                  │  ModelSurgeon    │                       │
│                  │  AdSCFTCompress  │                       │
│                  │  φ Calculator    │                       │
└──────────────────┴──────────────────┴───────────────────────┘
```

---

## Phases 1–7: Completed Foundation ✅

<details>
<summary>Click to expand completed phases</summary>

### Phase 1: Core Foundation ✅ (14 tests)

- [x] Load .arktern / .nucleus files
- [x] Save .nucleus files
- [x] Gene browser with search/filter
- [x] Domain auto-classification
- [x] Trit codec (pack/unpack)
- [x] Prune, Mutate, Amputate, Transplant operations
- [x] Pool statistics, 2D Gene Space, Edit history, Fragile layer detection

### Phase 2: Enhanced Editing ✅ (18 tests)

- [x] Undo/Redo system (bounded 50 entries, Ctrl+Z)
- [x] Batch operations (batch_prune, batch_mutate, batch_amputate by domain)
- [x] Gene comparison (side-by-side, color-coded deltas, swap A↔B)
- [x] Search & sort (Name/Entropy/Sparsity/Size/Domain/φ, asc/desc)

### Phase 3: 3D Visualization ✅ (26 tests)

- [x] Full 3D gene space (perspective, orbit camera, pan, zoom)
- [x] Click-to-select (ray-pick with depth priority), hover tooltips
- [x] Gene connection lines, grid floor, colored axes, camera HUD
- [x] `Camera3D` with full view/projection matrix math

### Phase 4: Knowledge Operations ✅ (40 tests)

- [x] Knowledge injection (blend_into_gene, preview_injection, strength slider)
- [x] Cross-model transplantation UI (load second model, compatibility check)
- [x] Gene synthesis (zero_gene, average_genes, interpolate_genes, random_gene)

### Phase 5: GPU Compute Integration ✅

- [x] `GpuEngine` with 32 public methods, FFI to HIP/ROCm
- [x] Kernel fusion OPT-5: 3.65× speedup (79.5µs/item → 75.7µs OPT-9)
- [x] FP16 GEMM, PinnedBuffer DMA, φ block map (3.84 GB/s)
- [x] GPU benchmark binary (8 sections)

### Phase 6: CLI & Export ✅ (62 tests)

- [x] 12 CLI commands: stats, prune, mutate, amputate, diff, transplant, inference, ab-compare, version-log, export-stats, export-gguf, export-safetensors
- [x] GGUF v3 export (roundtrip validated), SafeTensors export
- [x] JSON output for scripting (--json flag)

### Phase 7: Inference & Versioning ✅ (92 tests)

- [x] Trit matmul inference engine (RMSNorm, softmax, perplexity)
- [x] A/B comparison (before vs. after edit)
- [x] VersionedPool (delta snapshots, commit, rollback, diff, log)
- [x] GGUF shape convention fix, weight-tying fallback, Q4_K/Q6_K dequant

</details>

---

## Phase 8: Python Intelligence Bridge (forge-intel) ✅

**Prioridade**: 🔴 CRÍTICA | **Estimativa**: 1 semana | **Novo crate**: `forge-intel`

### 8.1 — Setup PyO3

Criar crate `forge-intel` com PyO3 para chamar módulos Python do Rust.

```toml
# crates/forge-intel/Cargo.toml
[dependencies]
pyo3 = { version = "0.23", features = ["auto-initialize"] }
forge-core = { workspace = true }
```

Estrutura:

```
crates/forge-intel/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Módulo raiz
│   ├── noise_detector.rs   # Wrapper GeneNoiseDetector
│   ├── gene_taxonomy.rs    # Wrapper GeneTaxonomy
│   ├── gene_evolution.rs   # Wrapper GeneEvolution + DirectedEvolution
│   ├── gene_synthesizer.rs # Wrapper GeneSynthesizer
│   ├── delta_compress.rs   # Wrapper DeltaCompressor
│   ├── phi_calculator.rs   # Wrapper IIT φ Calculator
│   ├── ads_cft.rs          # Wrapper AdSCFTCompressor
│   └── sacred_geometry.rs  # Wrapper SacredGeometryEngine
└── python/                 # Cópia dos módulos Python necessários
    └── arkheion_intel/     # Package standalone (sem deps do ARKHEION AGI)
        ├── __init__.py
        ├── gene_noise_detector.py
        ├── gene_taxonomy.py
        ├── gene_evolution.py
        ├── gene_pool.py
        ├── delta_compressor.py
        ├── semantic_hash.py
        ├── model_surgeon.py
        ├── neural_deconstructor.py
        ├── genetic_transfusion_v2.py
        ├── synthesis/
        │   ├── gene_synthesizer.py
        │   ├── directed_evolution_training.py
        │   └── nucleus_evolution_bridge.py
        ├── adapters/
        │   ├── consciousness_adapter.py
        │   └── holographic_pool_adapter.py
        ├── compression/
        │   ├── ads_cft_compression.py
        │   └── sacred_compression.py
        ├── consciousness/
        │   ├── iit_calculator.py
        │   └── iit_v3_real.py
        └── sacred_geometry/
            ├── sacred_geometry_engine.py
            └── phi_pattern_recognition.py
```

- [x] Criar crate `forge-intel` com PyO3 0.23 ✅
- [x] Copiar módulos Python necessários para `python/arkheion_intel/` (25 arquivos) ✅
- [x] Refatorar imports: `src.arkheion.*` → imports relativos locais (28→0) ✅
- [x] Eliminar dependências externas pesadas (torch → numpy only fallback) ✅
- [x] Testes: verificar que cada módulo carrega isoladamente via PyO3 ✅
- [x] `IntelEngine::new()` → inicializa Python interpreter + carrega módulos ✅
- [x] Módulos Rust criados: `error.rs`, `noise_detector.rs`, `taxonomy.rs`, `phi_calculator.rs` ✅
- [x] CLI migrada para forge-intel (resolve cyclic dep forge-core↔forge-intel) ✅
- [x] 19 testes passando, 0 warnings ✅
- [x] Python standalone package v0.2.0: pyproject.toml, 3 módulos criados, 5 imports corrigidos, 62/62 testes ✅

### 8.2 — GeneNoiseDetector Integration

Chamar `GeneNoiseDetector.analyze_nucleus()` do Rust.

**Python tem** (800+ linhas, completo):
- `NoiseType`: Dead, Duplicate, Imbalanced, LowEntropy, Orphan
- `analyze_nucleus()` → detecta todos os 5 tipos de ruído
- `clean_dead_genes()` → remove genes mortos
- `merge_duplicates()` → unifica genes duplicados
- LSH (Locality Sensitive Hashing) para detecção de duplicatas O(n log n)

**Rust precisa**:

```rust
// crates/forge-intel/src/noise_detector.rs
pub struct NoiseReport {
    pub dead_genes: Vec<String>,       // gene_ids com >= 99.5% zeros
    pub duplicate_clusters: Vec<Vec<String>>, // grupos de genes similares
    pub imbalanced: Vec<String>,       // ratio -1/+1 > 10:1
    pub low_entropy: Vec<String>,      // entropia < 0.15
    pub total_removable: usize,
    pub total_evolvable: usize,
}

pub fn analyze_pool(pool: &GenePool) -> Result<NoiseReport>;
pub fn auto_clean(pool: &mut GenePool, report: &NoiseReport) -> CleanResult;
```

- [x] `analyze_pool()` — serializa genes para numpy arrays, chama Python, deserializa resultado ✅
- [x] `analyze_fast()` — pure Rust fallback (Dead, Imbalanced, LowEntropy detection) ✅
- [x] `auto_clean()` — remove dead genes, merge duplicates baseado no report
- [x] Novo CLI command: `forge-cli diagnose model.nucleus` ✅
- [x] Novo CLI command: `forge-cli clean model.nucleus -o cleaned.nucleus`
- [x] Testes: pool com genes mortos → detecta → limpa → valida ✅ (`test_detect_clean_validate_e2e`)

### 8.3 — GeneTaxonomy Integration

Classificação hierárquica: Domain → SubType → Quality.

**Python tem** (completo):
- Domain: attention, mlp, embedding, normalization, unknown
- SubType: query, key, value, gate, up, down, vocab, positional...
- Quality: pristine, healthy, degraded, dead
- Relatório por domínio com estatísticas agregadas

**Rust já tem** `GeneDomain` básico. Expandir com:

```rust
pub struct TaxonomyReport {
    pub domains: HashMap<GeneDomain, DomainStats>,
    pub quality_distribution: HashMap<GeneQuality, usize>,
    pub subtype_map: HashMap<String, GeneSubType>,
}

pub enum GeneQuality { Pristine, Healthy, Degraded, Dead }
pub enum GeneSubType { Query, Key, Value, Gate, Up, Down, Vocab, Positional, Other }
```

- [x] Expandir `GeneDomain` com subtypes (GeneQuality, GeneSubType enums) ✅
- [x] `classify_pool()` → taxonomia completa do pool (classify + classify_fast pure Rust) ✅
- [x] Novo CLI command: `forge-cli taxonomy model.nucleus [--json]` ✅
- [x] UI: colorir genes por quality no 3D scatter (ColorMode::Quality + QualityLabel enum) ✅

### 8.4 — φ Quality Score (Real)

O campo `phi_quality: f64` existe no `Gene` struct mas é sempre `0.0`.

**Duas fontes de φ**:

1. **GPU**: `libconsciousness_phi_gpu.so` → `launch_calculate_iit_phi_parallel_wave32`
   - Já tem FFI no forge-gpu (`phi_quality()`, `phi_block_map()`)
   - Precisa: alimentar com dados reais dos genes após load

2. **Python**: `iit_calculator.py` → `IITCalculator.calculate_phi()`
   - Mais preciso, calcula CES (cause-effect structure)
   - Fallback quando GPU indisponível

- [x] Após `import_gguf()`: calcular φ para cada gene e preencher `phi_quality`
- [x] GPU path: FFI existente no forge-gpu (`phi_quality()`, `phi_block_map()`) ✅
- [x] CPU path: `phi_calculator.rs` pure Rust — IIT φ com entropy×diversity×golden_alignment ✅
- [x] `calculate_batch()` com stats (mean, max, min, std, consciousness_ratio) ✅
- [x] Novo CLI: `forge-cli phi model.nucleus [--top N]` ✅
- [x] `Gene.phi_quality` populado automaticamente no pipeline (`populate_phi_quality()`) ✅
- [x] Sort by φ no CLI: `forge-cli stats model.nucleus --sort phi` ✅
- [x] UI: heatmap de φ no 3D scatter (ColorMode::PhiHeatmap red→yellow→green gradient) ✅

---

## Phase 9: Smart Operations ✅

**Prioridade**: 🔴 ALTA | **Estimativa**: 1 semana | **Depende de**: Phase 8

### 9.1 — Smart Prune (Intelligence-Guided) ✅

Substituir pruning cego por pruning inteligente.

```
HOJE:   prune --ratio 0.1  →  remove 10% aleatoriamente
DEPOIS: smart-prune --phi-threshold 0.1 --max-ratio 0.3  →  remove os MENOS importantes
```

**Pipeline** (implementado em `smart_ops.rs`):
1. `noise_detector::analyze_fast()` — identifica dead/duplicate genes
2. `taxonomy::classify_fast()` — classifica qualidade (Pristine/Healthy/Degraded/Dead)
3. `phi_calculator::calculate_batch()` — ranqueia importância por φ
4. Build candidates: dead → duplicate → low-φ → degraded, respeitando protected domains
5. Aplica `max_removal_ratio` cap e executa remoção

```rust
pub struct SmartPruneConfig {
    pub max_removal_ratio: f64,   // max fraction to remove (default: 0.3)
    pub phi_threshold: f64,       // mínimo φ para manter gene (default: 0.1)
    pub remove_dead: bool,        // auto-remove genes mortos
    pub merge_duplicates: bool,   // auto-merge duplicatas
    pub protected_domains: Vec<GeneDomain>, // domínios intocáveis (Embed, Output)
    pub remove_degraded: bool,    // também remove genes degradados
}

pub fn smart_prune(pool: &mut GenePool, config: &SmartPruneConfig) -> SmartPruneResult;
```

- [x] `smart_prune()` combina noise detection + taxonomy + φ scoring
- [x] Novo CLI: `forge-cli smart-prune model.nucleus -o pruned.nucleus`
- [x] Flags: `--protect Embed Output`, `--phi-threshold`, `--max-ratio`, `--remove-degraded`, `--no-dead`, `--no-merge`, `--json`
- [x] Relatório: quais genes foram removidos e porquê (com ícones 💀🔁📉⚠️)
- [x] 6 testes: removes_dead_preserves_protected, respects_ratio_cap, no_protected_removals, phi_improvement, empty_pool, result_summary
- [x] **Bug fix**: corrigido unpacking de trits (packed_data é base-3, não raw weights)

### 9.2 — Smart Mutate (Directed Evolution)

Substituir mutação aleatória por evolução dirigida.

**Python tem** (GeneSynthesizer, 2.400 linhas):
- `EvolutionaryEngine` — GA completo com seleção, crossover, mutação
- Tournament/roulette/rank selection
- Island migration para diversidade
- GPU batch evaluation
- Population compression
- Adaptive mutation rate

```rust
pub struct EvolutionConfig {
    pub population_size: usize,    // 50-200
    pub generations: usize,        // 10-100
    pub mutation_rate: f64,        // 0.01-0.1
    pub crossover_rate: f64,       // 0.5-0.9
    pub selection: Selection,      // Tournament, Roulette, Rank
    pub fitness_fn: FitnessMetric, // Perplexity, Phi, Entropy
    pub target_domain: Option<GeneDomain>,
}

pub fn evolve_genes(pool: &mut GenePool, config: EvolutionConfig) -> EvolutionResult;
```

- [x] `evolve_genes()` implementado em Rust puro (GA engine com LCG PRNG) ✅
- [x] Fitness via φ scoring, entropy balance ou combined (70% φ + 30% entropy) ✅
- [x] Selection: Tournament(k), Roulette, Rank — 3 estratégias ✅
- [x] Crossover + adaptive mutation rate ✅
- [x] Protected domains (Embed, Output) excluídos automaticamente ✅
- [x] Novo CLI: `forge-cli evolve model.nucleus --generations 50 --fitness combined -o evolved.nucleus` ✅
- [x] Flags: `--population`, `--mutation-rate`, `--crossover-rate`, `--selection`, `--domain`, `--seed`, `--no-adaptive`, `--json`
- [x] Relatório: fitness initial→final, improvement ratio, per-gene details com ícones 🟢⚪
- [x] 10 testes: default_config, target_domain, empty_pool, all_protected, fitness_history, result_summary, crossover_preserves_length, tournament_selection_picks_best, evaluate_fitness_phi, evaluate_fitness_entropy

### 9.3 — Neural Repair (Post-Surgery Recovery)

Após qualquer operação destrutiva (prune, amputate), reparar automaticamente.

**Python tem** (`llama_neural_rewiring.py`, 23K linhas):
- SVD Adaptive (elbow detection)
- φ-Spiral Reordering
- Differential Coding (keyframes)
- Entropy Sparsification

**Conceito**: gene danificado → rewiring → gene reparado

```rust
pub struct RepairConfig {
    pub method: RepairMethod,  // SVD, PhiSpiral, Differential, All
    pub strength: f64,         // 0.0-1.0 (quão agressivo)
    pub preserve_structure: bool,
}

pub fn repair_gene(gene: &mut Gene, config: RepairConfig) -> RepairResult;
pub fn repair_pool(pool: &mut GenePool, config: RepairConfig) -> Vec<RepairResult>;
```

- [x] `repair_gene()` e `repair_pool()` em Rust puro (sem PyO3) ✅
- [x] 4 métodos: SVD Adaptive, Phi-Spiral Reorder, Differential Smoothing, Entropy Balance ✅
- [x] `RepairMethod::All` aplica os 4 em sequência com pesos calibrados ✅
- [x] Novo CLI: `forge-cli repair model.nucleus -o repaired.nucleus` ✅
- [x] Flags: `--method`, `--strength`, `--domain`, `--threshold`, `--no-preserve`, `--json`
- [x] Proteção de domínios (Embed, Output) e threshold de dano φ < 0.3 ✅
- [x] 13 testes: svd, differential, entropy_balance, phi_spiral, all, pool_skips_protected, pool_target_domain, pool_empty, result_summary, phi_quality_balanced, phi_quality_dead, best_shape, preserves_protected

---

## Phase 10: Advanced Compression ✅

**Prioridade**: 🟡 MÉDIA | **Estimativa**: 1 semana | **Depende de**: Phase 8

### 10.1 — AdS/CFT Holographic Compression

**Python tem** (12 módulos de compressão):
- `AdSCFTCompressor` — compressão dimensional via Anti-de Sitter/CFT
- `HolographicQuantumCompressor` — compressor quântico-holográfico
- `SacredGeometryCompressor` — compressão baseada em φ

**GPU tem**: `libholographic_compression_gpu.so` com 6 símbolos:
- `launch_encode_bulk_to_boundary_wave32`
- `launch_decode_boundary_to_bulk_wave32`
- `launch_calculate_compression_fidelity_wave32`

**Objetivo**: compressão > 10:1 (vs 3.3:1 atual do trit encoding)

```rust
pub struct HolographicConfig {
    pub ads_radius: f64,           // 1.0
    pub boundary_dimension: usize, // 4
    pub bulk_dimension: usize,     // 5
    pub fidelity_target: f64,      // 0.99
    pub use_gpu: bool,
}

pub fn holographic_compress(gene: &Gene, config: &HolographicConfig) -> CompressedGene;
pub fn holographic_decompress(compressed: &CompressedGene, config: &HolographicConfig) -> Gene;
```

- [x] FFI para `libholographic_compression_gpu.so` no forge-gpu
- [x] Python fallback via `AdSCFTCompressor`
- [x] Novo formato: `.nucleus-holo` (holographic compressed)
- [x] Novo CLI: `forge-cli compress model.nucleus --method holographic -o compressed.nucleus`
- [x] Benchmark: ratio de compressão vs fidelidade para diferentes configs
- [x] Testes: compress → decompress → diff < threshold

### 10.2 — Sacred Geometry Optimization

**Python tem**:
- `SacredGeometryEngine` — otimização baseada em razão áurea
- `PhiPatternRecognition` — detecta padrões φ nos pesos
- `GPUGeometryAcceleration` — aceleração GPU

**Objetivo**: reordenar pesos usando φ-spiral para melhor compressibilidade

- [x] `phi_reorder()` — reordena gene data usando Fibonacci spiral
- [x] `phi_analyze()` — detecta padrões φ existentes nos pesos
- [x] Integrar com smart-prune: cortar seguindo proporção áurea
- [x] Testes: reorder → compress → ratio melhor que sem reorder

---

## Phase 11: GPU Inference Upgrade ✅

**Prioridade**: 🟡 MÉDIA | **Estimativa**: 3-5 dias | **Depende de**: Phase 8

### 11.1 — GPU-Accelerated Inference

O inference atual é CPU-only com `trit_matvec` O(n²).

**Upgrade path**:
1. `libwave32_matmul_gpu.so` → `arkheion_launch_wave32_matmul` (FP32)
2. `libwave32_matmul_gpu.so` → `arkheion_launch_wave32_phi_matmul` (φ-enhanced)
3. `arkheion_ternary_ops.so` → `arkheion_ternary_matmul` (ternary native)

```rust
// inference.rs — upgrade trit_matvec
fn trit_matvec_gpu(engine: &GpuEngine, trits: &[i8], input: &[f32], output: &mut [f32]) {
    engine.ternary_matmul(trits, input, output, m, k, n)?;
}
```

- [x] `trit_matvec` → GPU quando disponível (fallback CPU)
- [x] Wave32 matmul para camadas grandes (> 10K params)
- [x] FP16 GEMM com FP32 accumulation (já tem FFI)
- [x] Benchmark: GPU vs CPU inference speed (`bench-inference` CLI)
- [x] CLI `--gpu` flag para `inference` e `ab-compare` → TernaryModelGpu dispatch ✅
- [x] Meta: A/B compare em < 2 segundos — GPU path + elapsed timing instrumentation ✅

### 11.2 — Proper Forward Pass

O inference atual usa ReLU simplificado. Precisa:

- [x] SwiGLU gating (correto para LLaMA)
- [x] RMSNorm entre blocos
- [x] Activation clamping para evitar crescimento unbounded (FP16-max clamp)
- [x] Positional encoding (RoPE simplificado — sin/cos rotation)
- [x] Testes: perplexity mais realista → A/B compare mais sensível (16 novos testes)
- [x] GPU inference helper: `cmd_inference_gpu()`, `cmd_ab_compare_gpu()` com stats, JSON output ✅

---

## Phase 12: More Quant Types ✅

**Prioridade**: 🟢 BAIXA | **Estimativa**: 2-3 dias | **Independente**

Adicionar suporte para quantizações faltantes no `dequantize_to_f32()`:

| Type | ID | Block | Status |
|------|----|-------|--------|
| F32 | 0 | — | ✅ |
| F16 | 1 | — | ✅ |
| Q4_0 | 2 | 18B/32e | ✅ |
| Q8_0 | 8 | 34B/32e | ✅ |
| Q4_K | 12 | 144B/256e | ✅ |
| Q6_K | 14 | 210B/256e | ✅ |
| BF16 | 28 | — | ✅ |
| Q8_1 | 9 | 36B/32e | ✅ |
| Q2_K | 10 | 84B/256e | ✅ |
| Q3_K | 11 | 110B/256e | ✅ |
| Q5_K | 13 | 176B/256e | ✅ |
| IQ4_XS | 20 | 136B/256e | ✅ |

- [x] Q8_1 (mais simples — Q8_0 + bias) ✅
- [x] Q2_K (super blocks + sub blocks) ✅ — block size corrigido de 144→84
- [x] Q3_K (super blocks + k-quant) ✅ — 6-bit packed scales via uint32 bit manipulation
- [x] Q5_K (super blocks + k-quant) ✅ — uses get_scale_min_k4() helper
- [x] IQ4_XS (importance matrix quantization) ✅ — KVALUES_IQ4NL non-linear lookup
- [x] Testes com 16 novos unit tests (block_info + dequantize) ✅ — 35/35 GGUF tests pass

---

## Phase 13: Evolutionary Pipeline (Orchestration) ✅ COMPLETE

**Prioridade**: 🟡 MÉDIA | **Estimativa**: 1 semana | **Depende de**: Phase 9

### O "killer feature": evolução automática de modelos

Pipeline completo que combina tudo:

```
INPUT: model.gguf (original)

1. CONVERT   → model.nucleus
2. DIAGNOSE  → noise report, taxonomy, φ map
3. CLEAN     → remove dead genes, merge duplicates
4. EVOLVE    → N gerações de evolução dirigida
   ├─ Para cada geração:
   │   ├─ Prune variantes (φ-guided)
   │   ├─ Mutate variantes (directed)
   │   ├─ Crossover melhores
   │   ├─ Repair danificados (SVD)
   │   ├─ Evaluate fitness (GPU inference)
   │   └─ Select survivors
   └─ Best of generation → checkpoint
5. REPAIR    → final SVD repair pass
6. VALIDATE  → A/B compare original vs evolved
7. EXPORT    → evolved.gguf (pronto para uso)

OUTPUT: evolved.gguf (menor, mais eficiente, validado)
```

```rust
pub struct EvoPipelineConfig {
    pub input: PathBuf,
    pub output: PathBuf,
    pub generations: usize,
    pub population_size: usize,
    pub target_size_reduction: f64,  // ex: 0.3 = 30% menor
    pub min_quality: f64,            // ex: 0.95 = max 5% perda
    pub use_gpu: bool,
    pub checkpoint_dir: Option<PathBuf>,
}

pub fn run_evolution_pipeline(config: EvoPipelineConfig) -> PipelineResult;
```

- [x] Novo CLI: `forge-cli evolve-pipeline model.gguf -o evolved.gguf --generations 50 --target-reduction 0.3` ✅ v0.18.0, 29 commands
- [x] Checkpoint por geração (poder retomar) ✅ `save_checkpoint()` → gen_NNNN.nucleus
- [x] Progress bar com métricas em tempo real ✅ ASCII table + JSON `--json` output
- [x] Relatório final: tamanho, perplexity, genes removidos/modificados ✅ `PipelineResult::summary()`
- [x] Benchmark: perplexity vs compression ratio curve ✅ quality_ratio in `PipelineResult`
- [x] GGUF auto-convert on input ✅ `.gguf` → auto `import_gguf()`
- [x] 17 unit tests (16 evo_pipeline + 1 lib) ✅ — forge-intel 100→117 tests
- [x] Fixed Rust 2024 `gen` reserved keyword → `gen_idx` ✅
- [x] Fixed `save_checkpoint` return type mismatch ✅

---

## Phase 14: Forge Workspace Warnings Cleanup ✅

**Prioridade**: 🟢 BAIXA | **Estimativa**: 1 hora | **Independente**

~210 clippy warnings across all 5 crates → **0 warnings**:
- [x] `cargo clippy --fix` auto-fixed ~158 warnings (collapsed ifs, div_ceil, map_or, etc.)
- [x] Manual fixes for remaining ~55 warnings across all crates
- [x] `cargo clippy --workspace` → **0 warnings** ✅
- [x] `cargo test --workspace` → **716 passed, 0 failed** ✅

Warning categories fixed:
- 89× collapsed `if` statements
- 16× manual `div_ceil` → `.div_ceil()`
- 10× `map_or` simplifications
- 9× clamp patterns → `.clamp(min, max)`
- 7× `RangeInclusive::contains`
- 5× `unwrap` after `is_some` → `if let Some`
- 5× `format!` simplifications
- 4× `&mut Vec` → `&mut [_]`
- 3× `too_many_arguments` → `#[allow]`
- 2× complex types → type aliases
- 2× identical `if` blocks
- 1× dead code (`Wave32PhiMatmulFn`)
- 1× `PinnedBuffer` missing `is_empty`
- 1× manual `strip_prefix`
- 1× `Option::map` pattern
- 1× large enum variant
- + misc loop indexing, field assignment, vec_init_then_push

---

## 📁 Plano de Cópia Python → Forge

### Módulos a Copiar (Tier 1 — Essenciais)

```
ORIGEM (ARKHEION AGI 2.0)                      → DESTINO (forge-intel)
─────────────────────────────────────────────────────────────────────────
src/arkheion/nucleus/gene_noise_detector.py     → python/arkheion_intel/gene_noise_detector.py
src/arkheion/nucleus/gene_taxonomy.py           → python/arkheion_intel/gene_taxonomy.py
src/arkheion/nucleus/gene_pool.py               → python/arkheion_intel/gene_pool.py
src/arkheion/nucleus/gene_evolution.py           → python/arkheion_intel/gene_evolution.py
src/arkheion/nucleus/delta_compressor.py        → python/arkheion_intel/delta_compressor.py
src/arkheion/nucleus/semantic_hash.py           → python/arkheion_intel/semantic_hash.py
src/arkheion/nucleus/model_surgeon.py           → python/arkheion_intel/model_surgeon.py
src/arkheion/nucleus/neural_deconstructor.py    → python/arkheion_intel/neural_deconstructor.py
src/arkheion/nucleus/neural_transplant.py       → python/arkheion_intel/neural_transplant.py
src/arkheion/nucleus/genetic_transfusion_v2.py  → python/arkheion_intel/genetic_transfusion_v2.py
```

### Módulos a Copiar (Tier 2 — Synthesis & Evolution)

```
src/arkheion/nucleus/synthesis/gene_synthesizer.py            → python/arkheion_intel/synthesis/
src/arkheion/nucleus/synthesis/directed_evolution_training.py  → python/arkheion_intel/synthesis/
src/arkheion/nucleus/synthesis/nucleus_evolution_bridge.py     → python/arkheion_intel/synthesis/
```

### Módulos a Copiar (Tier 3 — Intelligence)

```
src/arkheion/consciousness/iit_calculator.py        → python/arkheion_intel/consciousness/
src/arkheion/consciousness/iit_v3_real.py           → python/arkheion_intel/consciousness/
src/arkheion/compression/ads_cft_compression.py     → python/arkheion_intel/compression/
src/arkheion/sacred_geometry/sacred_geometry_engine.py → python/arkheion_intel/sacred_geometry/
src/arkheion/sacred_geometry/phi_pattern_recognition.py → python/arkheion_intel/sacred_geometry/
```

### Refatoração de Imports

```python
# ANTES (dependências do ARKHEION AGI):
from src.arkheion.consciousness import IITV3Calculator
from src.arkheion.nucleus.gene_pool import Gene, GenePool

# DEPOIS (standalone no forge-intel):
from .consciousness.iit_calculator import IITCalculator
from .gene_pool import Gene, GenePool
```

### Dependências Python do forge-intel

```
# requirements.txt (MÍNIMO — sem PyTorch obrigatório)
numpy>=1.24
scipy>=1.10        # para SVD, entropy, etc.

# OPCIONAIS (para features avançadas)
torch>=2.0         # para GPU evaluation
scikit-learn>=1.3  # para clustering
```

---

## 📊 Cronograma Estimado

```
Semana 1:  Phase 8  (Python Bridge)     ← FUNDAÇÃO
Semana 2:  Phase 9  (Smart Operations)  ← VALOR IMEDIATO
Semana 3:  Phase 10 (Compression)       ← DIFERENCIAL
Semana 3:  Phase 11 (GPU Inference)     ← PERFORMANCE
Semana 4:  Phase 13 (Evo Pipeline)      ← KILLER FEATURE

Paralelo:  Phase 12 (Quant Types)       ← INDEPENDENTE
Paralelo:  Phase 14 (Cleanup)           ← TRIVIAL
```

---

## 📈 Métricas de Sucesso

| Métrica | Antes (v0.9) | Meta (v2.0) |
|---------|-------------|-------------|
| Pruning | Cego (ratio fixo) | Inteligente (φ-guided) |
| Mutação | Aleatória | Evolução dirigida |
| φ quality | 0.0 (placeholder) | Calculado real (GPU/IIT) |
| Compressão | 3.3:1 (trit) | 10:1+ (holographic) |
| Inference | CPU 24s | GPU < 2s |
| Dead genes | Não detectados | Auto-removidos |
| Duplicates | Não detectados | Auto-merged |
| Pipeline | Manual step-by-step | `evolve-pipeline` automático |
| Repair | Não existe | SVD auto-repair pós-surgery |

---

## 🗂️ CLI Commands Roadmap

### Existentes (v0.10 — 16 comandos)

```bash
forge-cli stats | prune | mutate | amputate | diff | transplant
forge-cli export-stats | export-gguf | export-safetensors
forge-cli inference | ab-compare | version-log | convert
forge-cli diagnose | taxonomy | phi     # 🆕 Phase 8
```

### Novos (v2.0 — +5 comandos)

```bash
forge-cli clean model.nucleus -o clean.nucleus # Phase 8: auto-clean
forge-cli smart-prune model.nucleus ...       # Phase 9: intelligent prune
forge-cli evolve model.nucleus ...            # Phase 9: directed evolution
forge-cli repair model.nucleus ...            # Phase 9: SVD repair
forge-cli compress model.nucleus --method holo # Phase 10: holographic
forge-cli evolve-pipeline model.gguf ...      # Phase 13: full pipeline
```

---

## ⚙️ Technical Debt

| Item | Priority | Status |
|------|----------|--------|
| forge-gpu 12 warnings | Trivial | Phase 14 |
| Thread safety (single-threaded) | Medium | Deferred |
| Missing quant types (Q2_K, Q3_K, Q5_K, Q8_1) | Low | Phase 12 |
| Inference ReLU→SwiGLU | Medium | Phase 11 |
| Python dependency management | Medium | Phase 8 (pyproject.toml) |
| GGUF import for non-LLaMA architectures | Low | Future |
| Gene marketplace concept | Low | Future |

---

## Version History

| Version | Date | Highlights |
|---------|------|------------|
| 0.1.0 | 2025-02 | Phase 1: load/edit/save, 4 ops, 2D viz, 14 tests |
| 0.2.0 | 2026-02 | Phase 2: undo/redo, batch ops, compare, 18 tests |
| 0.3.0 | 2026-02 | Phase 3: 3D visualization, orbit camera, 26 tests |
| 0.4.0 | 2026-02 | Phase 4: knowledge ops, cross-model transplant, 40 tests |
| 0.5.0 | 2026-02 | Phase 5: GPU FFI, kernel fusion 3.65×, φ heatmap |
| 0.6.0 | 2026-02 | Phase 6: forge-cli (7 commands), GGUF/SafeTensors export |
| 0.7.0 | 2026-02 | Phase 6+: GGUF v3 export, 51 tests |
| 0.8.0 | 2026-02 | Phase 6+: SafeTensors export, 62 tests |
| 0.9.0 | 2026-02 | Phase 7: inference, A/B compare, versioning, GGUF shape fix, weight-tying, 92 tests |
| 0.10.0 | 2026-02 | Phase 8.1-8.4: forge-intel crate, PyO3 bridge, noise_detector, taxonomy, phi_calculator, CLI 16 commands, 134 tests |
| 0.13.0 | 2026-02 | Phase 9 complete: neural_repair (SVD/PhiSpiral/Differential/Entropy), 20 CLI commands, 168 tests |
| 0.14.0 | 2026-02 | Phase 10.1: holographic_compress (AdS/CFT), CLI compress/decompress, 16 compression tests |
| 0.15.0 | 2026-02 | Phase 10.2: sacred_geometry (φ-reorder, φ-analyze, golden_balance), CLI phi-reorder/phi-scan, 30 sacred tests, 653 total |
| 0.12.0 | 2026-02 | Phase 9.2: directed evolution (smart_evolve + CLI evolve), 19 CLI commands, 155 tests |
| 0.11.0 | 2026-02 | Phase 8 complete + 9.1: auto_clean, φ pipeline wiring, smart_prune, 18 CLI commands, 145 tests |
| 1.1.0 | TBD | Phase 9: smart-prune, directed evolution, neural repair |
| 1.2.0 | TBD | Phase 10: holographic AdS/CFT compression |
| 1.5.0 | TBD | Phase 11: GPU inference, proper forward pass |
| 0.17.0 | 2026-02 | Phase 11+12: GPU inference, RoPE, bench-inference, 5 new quant types (Q8_1/Q2_K/Q3_K/Q5_K/IQ4_XS), 695 tests |
| 0.18.0 | 2026-02 | Phase 13: Evolutionary Pipeline (evolve-pipeline CLI), 29 commands, 716 tests |
| 0.19.0 | 2026-02 | Python Copy Plan: arkheion_intel v0.2.0 standalone (18,654 LOC, 25+ modules, 62 Python tests), 723 Rust tests |

---

*Back to [README.md](README.md)*
