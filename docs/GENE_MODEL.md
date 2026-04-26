# 🧬 Gene Data Model

> Core data structures: Gene, GenePool, GeneDomain, and Trit Codec

---

## Gene

A **Gene** is the atomic unit in ARKHEION Forge — one tensor layer of a neural network, stored in ternary form.

### Struct Definition

```rust
pub struct Gene {
    pub gene_id:     String,                           // SHA-256[:16] of packed_data
    pub layer_name:  String,                           // e.g., "blk.0.attn_q.weight"
    pub shape:       Vec<usize>,                       // e.g., [4096, 4096]
    pub packed_data: Vec<u8>,                          // Trit-packed bytes (5 trits/byte)
    pub n_elements:  usize,                            // Total params (product of shape)
    pub sources:     Vec<String>,                      // Source model names
    pub domain:      GeneDomain,                       // Auto-classified domain
    pub phi_quality: f64,                              // φ-based quality score
    pub metadata:    HashMap<String, serde_json::Value> // Extensible attributes
}
```

### Gene Identity

Every gene has a **content-derived ID**:

```text
gene_id = hex( SHA-256( packed_data ) )[:16]
```

This means:
- **Same data → same ID** — content-addressable storage
- **Editing → new ID** — identity reflects current state
- **Automatic deduplication** — identical tensors share one gene

### Construction

```rust
// From raw trit values
let gene = Gene::from_trits(
    "blk.0.attn_q.weight",     // layer name
    &[1, 0, -1, 1, -1, 0],    // trit values {-1, 0, +1}
    vec![2, 3],                 // shape
    vec!["llama-3.2-1b".into()] // source models
);
```

### Key Methods

| Method | Return | Description |
| ------ | ------ | ----------- |
| `from_trits()` | `Gene` | Construct from raw i8 trits, auto-packs and computes ID |
| `compute_id(data)` | `String` | SHA-256 → hex[:16] of arbitrary bytes |
| `unpack()` | `Vec<i8>` | Decode packed_data back to trit values |
| `packed_size()` | `usize` | Number of packed bytes |
| `sparsity()` | `f64` | Fraction of zeros (0.0 – 1.0) |
| `entropy()` | `f64` | Shannon entropy in bits |
| `distribution()` | `(neg, zero, pos)` | Count of each trit value |
| `is_fragile()` | `bool` | True if Norm or Embed domain |

### Fragile Detection

Certain gene domains are **dangerous to edit** because they control normalization or token mapping:

```rust
pub fn is_fragile(&self) -> bool {
    matches!(self.domain, GeneDomain::Norm | GeneDomain::Embed)
}
```

Fragile genes are marked with ⚠️ in the UI and the editor shows a warning banner.

---

## GeneDomain

Automatic classification of a gene's functional role based on its layer name.

### Variants

| Domain | Color | Detection Patterns |
| ------ | ----- | ------------------ |
| `Attention` | `#FF6B6B` (red) | `attn`, `attention`, `self_attn`, `q_proj`, `k_proj`, `v_proj`, `o_proj` |
| `Mlp` | `#4ECDC4` (teal) | `mlp`, `ffn`, `feed_forward`, `gate_proj`, `up_proj`, `down_proj`, `fc1`, `fc2`, `w1`, `w2`, `w3` |
| `Norm` | `#45B7D1` (blue) | `norm`, `ln_`, `layer_norm`, `rms` |
| `Embed` | `#96CEB4` (green) | `embed`, `embd`, `wte`, `wpe`, `tok_` |
| `Output` | `#FFEAA7` (yellow) | `output`, `lm_head`, `final` |
| `Bias` | `#DDA0DD` (plum) | `bias` |
| `Conv` | `#FF8C42` (orange) | `conv` |
| `Unknown` | `#888888` (gray) | Anything else |

### Priority Order

Classification is checked in this specific order to handle ambiguous names:

```text
1. Norm     ← "attn_norm" is Norm, not Attention
2. Bias     ← "attn_bias" is Bias, not Attention
3. Embed    ← checked before Attention/MLP
4. Output
5. Conv
6. Attention
7. Mlp
8. Unknown  ← fallback
```

> **Design note**: Norm is checked before Attention because layer names like
> `attn_norm`, `attn_layer_norm` are normalization layers, not attention weights.

### Color Mapping

Each domain has both hex (`color()`) and RGBA (`color_rgba()`) representations for UI rendering:

```rust
pub fn color(&self) -> &'static str {
    match self {
        Attention => "#FF6B6B",
        Mlp       => "#4ECDC4",
        Norm      => "#45B7D1",
        Embed     => "#96CEB4",
        Output    => "#FFEAA7",
        Bias      => "#DDA0DD",
        Conv      => "#FF8C42",
        Unknown   => "#888888",
    }
}
```

---

## GenePool

A **GenePool** is a complete model — a collection of genes with architecture metadata.

### Struct Definition

```rust
pub struct GenePool {
    pub genes:         HashMap<String, Gene>,            // gene_id → Gene
    pub architectures: HashMap<String, HashMap<String, String>>, // model → layer → gene_id
    pub version:       u16,
}
```

### Operations

| Method | Description |
| ------ | ----------- |
| `insert(gene)` | Add gene to pool (by gene_id) |
| `remove(id)` | Remove gene by ID |
| `get(id)` | Immutable reference to gene |
| `get_mut(id)` | Mutable reference (for editing) |
| `gene_count()` | Number of genes |
| `by_domain(domain)` | Filter genes by domain |
| `domain_stats()` | Count per domain: `HashMap<GeneDomain, usize>` |

### Multi-Model Support

The `architectures` map allows a single GenePool to represent genes from **multiple models**:

```text
architectures = {
    "llama-3.2-1b": {
        "blk.0.attn_q.weight": "a1b2c3d4e5f60718",
        "blk.0.attn_k.weight": "f8e7d6c5b4a39281",
        ...
    },
    "mistral-7b": {
        "blk.0.attn_q.weight": "1122334455667788",
        ...
    }
}
```

This enables **gene transplantation** — copying a gene from one model's architecture into another.

---

## Trit Codec

The trit codec packs ternary values `{-1, 0, +1}` into bytes at 5 trits per byte.

### Encoding

```text
Input:  Trit values ∈ {-1, 0, +1}
Map:    -1 → 0,  0 → 1,  +1 → 2

Pack 5 trits into one byte:
  byte = t₀ × 1  +  t₁ × 3  +  t₂ × 9  +  t₃ × 27  +  t₄ × 81

Powers: [1, 3, 9, 27, 81]
Max value: 2 + 6 + 18 + 54 + 162 = 242 (fits in u8)
```

### Decoding

Uses a **pre-computed decode table** (243 entries) for O(1) per-byte decoding:

```text
For each byte value 0..=242:
  t₀ = byte % 3;       byte /= 3;
  t₁ = byte % 3;       byte /= 3;
  t₂ = byte % 3;       byte /= 3;
  t₃ = byte % 3;       byte /= 3;
  t₄ = byte % 3;

Map back: 0 → -1,  1 → 0,  2 → +1
```

### Efficiency Analysis

```text
Information content:  log₂(3) = 1.585 bits per trit
Actual usage:         8 bits per 5 trits = 1.600 bits per trit
Overhead:             0.95% (near-optimal)

Compression ratio vs float32:
  32 bits / 1.6 bits = 20× per parameter

Compression ratio vs float16:
  16 bits / 1.6 bits = 10× per parameter
```

### Statistical Functions

| Function | Description | Formula |
| -------- | ----------- | ------- |
| `sparsity()` | Fraction of zero values | `count(0) / n_elements` |
| `entropy()` | Shannon entropy (bits) | $H = -\sum p_i \log_2(p_i)$ |
| `distribution()` | Trit value counts | `(count(-1), count(0), count(+1))` |

**Entropy interpretation**:
- 0.0 bits — all same value (constant tensor)
- 1.585 bits — uniform distribution (maximum for ternary)
- Typical range: 1.0 – 1.5 bits for real model layers

---

## Worked Example

Loading a Llama 3.2 1B `.arktern` file:

```text
Input: llama3.2-1b_ternary.arktern (173 MB)

Parsing:
  Header:  ARKT, 67 tensors, 1.2B params
  Index:   67 TensorEntry records
  Data:    Packed trit bytes

Conversion to GenePool:
  67 genes created
  gene_id computed for each (SHA-256[:16])
  Domains auto-classified:
    Attention: ~24 genes (attn_q, attn_k, attn_v, attn_o per block)
    MLP:       ~24 genes (gate, up, down per block)
    Norm:      ~16 genes (rms_norm per block + final)
    Embed:      1 gene  (token_embd)
    Output:     1 gene  (output/lm_head)

Statistics:
  Total params:    1,235,814,400
  Average sparsity: ~21%
  Average entropy:  ~1.42 bits
  Packed size:      ~247 MB (uncompressed trits)
  File size:        ~173 MB (with zlib on some tensors)
```

---

*Next: [OPERATIONS.md](OPERATIONS.md) — Surgical operations*
