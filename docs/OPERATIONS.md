# 🛠️ Surgical Operations

> Prune, Mutate, Transplant, Amputate — gene editing reference

---

## Overview

ARKHEION Forge provides four surgical operations for editing neural genes. All operations:

1. **Work in-place** on the gene's `packed_data`
2. **Recompute gene_id** after modification (SHA-256)
3. **Return an `EditResult`** for history tracking
4. **Never panic** — all edge cases handled gracefully

```rust
pub struct EditResult {
    pub gene_id:        String,  // Gene that was edited
    pub operation:      String,  // Human-readable description
    pub before_nnz:     usize,   // Non-zero count before
    pub after_nnz:      usize,   // Non-zero count after
    pub before_entropy: f64,     // Shannon entropy before
    pub after_entropy:  f64,     // Shannon entropy after
}
```

---

## ✂️ Prune

**Purpose**: Remove weak connections by zeroing out a fraction of non-zero trits.

### Signature

```rust
pub fn prune(gene: &mut Gene, ratio: f64) -> EditResult
```

### Parameters

| Param | Type | Range | Description |
| ----- | ---- | ----- | ----------- |
| gene | &mut Gene | — | Gene to prune |
| ratio | f64 | 0.0 – 1.0 | Fraction of non-zero trits to zero out |

### Algorithm

```text
1. Unpack trits: packed_data → Vec<i8>
2. Collect indices of all non-zero trits
3. Calculate how many to prune: n = floor(nnz × ratio)
4. Deterministically select n indices (evenly spaced)
5. Set selected trits to 0
6. Repack and recompute gene_id
```

### Selection Strategy

Indices are selected with **even spacing**, not randomly:

```text
step = nnz / n_to_prune
selected = [floor(i × step) for i in 0..n_to_prune]
```

This ensures consistent, reproducible results across runs.

### Examples

```text
Before: [-1, 1, 0, 1, -1, 0, 1, 1, -1, 0]
         nnz = 7, sparsity = 30%

prune(ratio=0.3):  zero out 2 of 7 non-zero
After:  [-1, 1, 0, 0, -1, 0, 1, 0, -1, 0]
         nnz = 5, sparsity = 50%

prune(ratio=0.0):  no-op
prune(ratio=1.0):  equivalent to amputate
```

### UI Controls

- **Slider**: 0% – 50% (capped at 50% in UI for safety)
- **Button**: "✂️ Prune"
- **Status**: `"Pruned: 7 nnz → 5 nnz"`

---

## 🧬 Mutate

**Purpose**: Introduce random perturbations by flipping trit values.

### Signature

```rust
pub fn mutate(gene: &mut Gene, probability: f64, seed: u64) -> EditResult
```

### Parameters

| Param | Type | Range | Description |
| ----- | ---- | ----- | ----------- |
| gene | &mut Gene | — | Gene to mutate |
| probability | f64 | 0.001 – 0.1 | Per-trit mutation probability |
| seed | u64 | — | PRNG seed for determinism |

### Algorithm

```text
1. Unpack trits: packed_data → Vec<i8>
2. Initialize LCG PRNG with seed
3. For each trit:
   a. Generate pseudo-random number
   b. If (random % 1000) < (probability × 1000):
      Cycle the value: -1 → 0 → 1 → -1
4. Repack and recompute gene_id
```

### PRNG: Linear Congruential Generator

```rust
state = state.wrapping_mul(6364136223846793005)
             .wrapping_add(1442695040888963407);
// Use state >> 33 for value extraction
```

**Deterministic**: Same seed → same mutations. Enables reproducible experiments.

### Mutation Cycle

```text
-1  →  0  →  +1  →  -1  →  ...

In mapped form (after encoding):
 0  →  1  →   2  →   0  →  ...
```

This is a **cyclic permutation**, not random assignment. It preserves the balance of the distribution.

### Examples

```text
Before: [-1, 1, 0, 1, -1]
         entropy = 1.52 bits

mutate(prob=0.01, seed=42):
  ~1% of trits flip: [-1, 1, 0, 0, -1]  (one flip)
  entropy ≈ 1.48 bits

mutate(prob=0.1, seed=42):
  ~10% of trits flip: more changes
  entropy shifts more significantly
```

### UI Controls

- **Slider**: 0.1% – 10%
- **Button**: "🧬 Mutate"
- **Seed**: Auto-generated from system clock (`SystemTime::now()`)
- **Status**: `"Mutated: entropy 1.520 → 1.483"`

---

## 🔄 Transplant

**Purpose**: Copy a gene from one GenePool to another.

### Signature

```rust
pub fn transplant(
    source: &GenePool,
    gene_id: &str,
    target: &mut GenePool
) -> Option<EditResult>
```

### Parameters

| Param | Type | Description |
| ----- | ---- | ----------- |
| source | &GenePool | Pool to copy from |
| gene_id | &str | ID of gene to transplant |
| target | &mut GenePool | Pool to insert into |

### Algorithm

```text
1. Look up gene_id in source pool
2. Clone the gene
3. Insert clone into target pool
4. Return EditResult with gene stats
```

### Use Cases

- **Knowledge transfer**: Copy attention heads from a larger model
- **Layer replacement**: Swap out a damaged or undertrained layer
- **Multi-model fusion**: Build chimeric models from parts of different models

### Returns

- `Some(EditResult)` if the gene was found and transplanted
- `None` if `gene_id` not found in source

---

## 🗑️ Amputate

**Purpose**: Completely zero out a gene (remove all learned information).

### Signature

```rust
pub fn amputate(gene: &mut Gene) -> EditResult
```

### Algorithm

```text
1. Record current nnz and entropy
2. Unpack trits
3. Set ALL trits to 0
4. Repack (all bytes become value 121 = 1+3+9+27+81)
5. Recompute gene_id
6. Return EditResult
```

### After Amputation

```text
sparsity:    100%
entropy:     0.0 bits
distribution: (0, n_elements, 0)
nnz:         0
```

### Warning

Amputating critical layers (Norm, Embed) will likely **destroy model functionality**. The UI shows these as ⚠️ fragile and uses red-colored button.

### UI Controls

- **Button**: "🗑️ Amputate (zero all)" (red text)
- **Status**: `"Amputated: 16777216 → 0 nnz"`

---

## 📊 Pool Statistics

The `pool_stats()` function provides aggregate metrics across all genes:

### Signature

```rust
pub fn pool_stats(pool: &GenePool) -> PoolStats
```

### PoolStats Struct

```rust
pub struct PoolStats {
    pub gene_count:    usize,                      // Total genes
    pub total_params:  usize,                      // Sum of n_elements
    pub total_nnz:     usize,                      // Sum of non-zero trits
    pub avg_sparsity:  f64,                        // Mean sparsity
    pub avg_entropy:   f64,                        // Mean entropy
    pub domain_counts: HashMap<GeneDomain, usize>, // Count per domain
}
```

### Displayed On Load

When a file is opened, pool_stats populates the status bar:

```text
"Loaded 67 genes | 1235814400 params | 21.3% sparse | 1.42 bits entropy"
```

---

## Operation Comparison

| Operation | Changes nnz | Changes entropy | Reversible | Risk |
| --------- | ----------- | --------------- | ---------- | ---- |
| Prune | ↓ decreases | ↓ usually decreases | No* | Low |
| Mutate | ≈ similar | ≈ shifts slightly | No* | Medium |
| Transplant | — (additive) | — | Remove gene | Low |
| Amputate | ↓↓ to zero | ↓↓ to zero | No* | **High** |

\* Operations are tracked in `edit_history` but undo is not yet implemented (Phase 2).

---

## Test Coverage

All operations have unit tests (14/14 passing):

| Test | Operation | Validates |
| ---- | --------- | --------- |
| `prune_50_percent` | Prune | nnz reduced by ~50% |
| `prune_zero_is_noop` | Prune | ratio=0 changes nothing |
| `mutate_deterministic` | Mutate | Same seed → same result |
| `amputate_zeros_all` | Amputate | All trits become 0 |

---

*Next: [UI_GUIDE.md](UI_GUIDE.md) — User interface guide*
