# AI Core — Python Module Reference

> Detailed reference for the Python AI intelligence layer.

---

## Module Index

### Perception Layer (How the system sees code)

| Module | Files | LOC | Purpose |
|--------|-------|-----|---------|
| **RAG Pipeline** | 4 | ~51K | Retrieval-Augmented Generation — semantic search across codebase |
| **HDCache** | 15 | ~46K | 5-layer hierarchical cache (RAM→Redis→FP→pgvector→LLM) |
| **Neural Mesh** | 7 | ~190K | Cognitive lattice: crystals, embeddings, spheres, resonance |
| **Semantic Cache** | 1 | 36K | Similarity-based response caching |
| **Intelligent Cache** | 1 | 34K | AI-powered cache with auto-learning |

### Reasoning Layer (How the system thinks)

| Module | Files | LOC | Purpose |
|--------|-------|-----|---------|
| **Agent Core** | 1 | 83K | Central AI agent — intent classification, response generation |
| **Planning Engine** | 4 | ~89K | Strategic planning with task decomposition |
| **Autopilot** | 8 | ~205K | Autonomous AI sessions with tool registry |
| **Dream Engine** | 5 | ~32K | Counterfactual reasoning and consolidation |
| **Cognitive Thinking** | 1 | 35K | Extended thinking / chain-of-thought reasoning |
| **Streaming Inference** | 1 | 54K | Token-by-token LLM generation |

### Evolution Layer (How the system improves)

| Module | Files | LOC | Purpose |
|--------|-------|-----|---------|
| **Learning Loop** | 7 | ~46K | Self-improvement from outcomes |
| **Fine-Tuning** | 7 | ~97K | Model training with data collection |
| **System Crystallizer** | 8 | ~89K | Python → C → .so compilation |
| **Model Registry** | 1 | 36K | Model versioning and management |
| **Training Data Extractor** | 1 | 30K | Extract training data from operations |

### Kernel Layer (Mathematical foundations)

| Module | Files | LOC | Purpose |
|--------|-------|-----|---------|
| **Phi Kernel** | 1 | 12K | Golden ratio decay, Φ-coherence |
| **Graph Kernel** | 1 | 18K | Graph algorithms on dependency lattice |
| **Embedding Kernel** | 1 | 9K | Embedding operations and comparison |
| **Compression Kernel** | 1 | 10K | Data compression algorithms |
| **Resonance Kernel** | 1 | 9K | Resonance field calculations |
| **Fingerprint Kernel** | 1 | 12K | Content fingerprinting |
| **Clustering Kernel** | 1 | 5K | Semantic clustering |
| **Synthesis Kernel** | 1 | 7K | Crystal synthesis operations |
| **iGPU Cache** | 1 | 13K | Zero-copy GPU memory cache |

### Temporal Layer (How the system remembers)

| Module | Files | LOC | Purpose |
|--------|-------|-----|---------|
| **WAL** | 1 | 7K | Write-Ahead Log for causal traces |
| **Chain** | 1 | 6K | Temporal snapshot chain |
| **Snapshot** | 1 | 7K | Point-in-time lattice state |
| **Store** | 1 | 8K | Persistent temporal storage |
| **Invariant** | 1 | 8K | Temporal invariant enforcement |

### Intelligence Layer (Evolutionary algorithms)

| Module | Files | LOC | Purpose |
|--------|-------|-----|---------|
| **Forge Intel** | 30 | ~200K+ | Gene evolution, sacred geometry, IIT, compression |
| **Model Surgeon** | 1 | - | Surgical model editing |
| **Neural Transplant** | 1 | - | Cross-model gene transfer |
| **Gene Pool** | 1 | - | Population of gene variants |
| **Mutation** | 1 | - | Mutation operators |

---

## Key Abstractions

### Crystal (SkillCrystal)
```python
class SkillCrystal:
    """A crystallized unit of knowledge or skill.
    
    Crystals are the atomic units in the Neural Mesh.
    Each crystal represents a learned capability that can be:
    - Composed with other crystals
    - Evaluated for coherence (Φ)
    - Transferred between contexts
    - Evolved through mutation
    """
    embedding: ndarray     # 768-dim semantic representation
    capabilities: list     # What this crystal can do
    connections: list      # Links to other crystals
    phi_score: float       # Coherence score (0.0 - 1.0)
    metadata: dict         # Source, version, lineage
```

### HDGraph Node
```python
class HDGraphNode:
    """A node in the hierarchical directed graph.
    
    HDGraph is the structural backbone of HDCache.
    Each node represents a cached computation with
    connections to its inputs and dependents.
    """
    key: str               # Unique identifier
    value: Any             # Cached result
    embedding: ndarray     # Semantic embedding
    fingerprint: str       # Content hash
    dependencies: list     # Input nodes
    dependents: list       # Output nodes
    access_count: int      # For eviction policy
    last_access: datetime  # For LRU
```

### Gene (Rust-side)
```rust
pub struct Gene {
    pub name: String,           // e.g. "blk.0.attn.q"
    pub layer_type: LayerType,  // ATTN, MLP, NORM, EMBED
    pub weights: Vec<f32>,      // Parameter tensor
    pub shape: Vec<usize>,      // Tensor dimensions
    pub metadata: GeneMetadata, // Source, version, phi
}
```

### Φ-Coherence
```python
def calculate_phi(module: str, test_results: dict) -> float:
    """
    Φ = (pass_rate) × (coverage) × min(assertion_density / 10.0, 1.0)
    
    Where:
    - pass_rate: tests passed / total tests
    - coverage: % of source lines covered by tests
    - assertion_density: assertions / test files
    """
    pass_rate = test_results["passed"] / max(test_results["total"], 1)
    coverage = test_results.get("coverage", 0.78)
    density = test_results.get("assertion_density", 7.76)
    return round(pass_rate * coverage * min(density / 10.0, 1.0), 4)
```

---

## Inter-Module Communication

```
┌─────────────┐     embed      ┌──────────────┐
│ RAG Pipeline │◄──────────────│ Ollama       │
│              │     query      │ (Embeddings) │
└──────┬───────┘               └──────────────┘
       │ chunks
       ▼
┌──────────────┐     cache      ┌──────────────┐
│ HDCache      │◄──────────────│ Redis        │
│ (5-layer)    │               │ (L0b)        │
└──────┬───────┘               └──────────────┘
       │ results                      │
       ▼                              │ patterns
┌──────────────┐                      │
│ Neural Mesh  │◄─────────────────────┘
│ (connectivity)│
└──────┬───────┘
       │ context
       ▼
┌──────────────┐     model      ┌──────────────┐
│ Agent Core   │◄──────────────│ Model Router │
│              │               │              │
└──────┬───────┘               └──────────────┘
       │ candidate
       ▼
┌──────────────┐     forge      ┌──────────────┐
│ Crystallizer │───────────────│ Phi Kernel   │
│              │               │ (validation) │
└──────────────┘               └──────────────┘
```

---

## Adaptation Notes

### Removing Arkheion Forge Dependencies

The AI core was extracted from the Arkheion Forge backend. To make it fully standalone:

1. **Database models** — Some modules reference SQLAlchemy/Pydantic models from `backend/src/models/`. These need to be replaced with standalone schemas.

2. **Tenant isolation** — Multi-tenant logic (`tenant_id` everywhere) can be simplified for single-user use.

3. **Sales-specific logic** — Modules like `objection_cache.py`, `seller_bot/`, `sales_response_boundary.py` are sales-domain specific. These can be removed or generalized.

4. **API routers** — The modules were designed to be served via FastAPI routers. The standalone version can expose them via MCP or CLI instead.

5. **Config files** — References to `config/` YAML files need to be redirected to local config.

### Priority Modules for Standalone

1. `rag_pipeline.py` — Core retrieval
2. `hdcache/_cache.py` — Core caching
3. `neural_mesh/` — Core connectivity
4. `arkheion/kernels/` — Core computation
5. `streaming_inference.py` — Core generation
6. `system_crystallizer/` — Core build
7. `learning/` — Core self-improvement
