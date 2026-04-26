# 📦 Binary Format Specifications

> .nucleus (ARKUNN02) and .arktern format documentation

---

## .nucleus Format (ARKUNN02)

The **Nucleus** format is ARKHEION Forge's native file format for storing gene pools. It uses a length-prefixed binary layout with zstd outer compression.

### Overview

```text
┌────────────────────────────────────────────┐
│          ZSTD Compressed Envelope          │
│  ┌──────────────────────────────────────┐  │
│  │ Magic: ARKUNN02         (8 bytes)    │  │
│  │ Version: u16 LE         (2 bytes)    │  │
│  │ Gene Count: u32 LE      (4 bytes)    │  │
│  ├──────────────────────────────────────┤  │
│  │ Gene 0                               │  │
│  │ Gene 1                               │  │
│  │ ...                                  │  │
│  │ Gene N-1                             │  │
│  ├──────────────────────────────────────┤  │
│  │ Architectures (zlib JSON)            │  │
│  │ Absorption History (zlib JSON)       │  │
│  └──────────────────────────────────────┘  │
└────────────────────────────────────────────┘
```

### Header

| Offset | Size | Type | Description |
| ------ | ---- | ---- | ----------- |
| 0 | 8 | bytes | Magic: `ARKUNN02` (ASCII) |
| 8 | 2 | u16 LE | Format version (current: 2) |
| 10 | 4 | u32 LE | Number of genes |

### Per-Gene Record

Each gene is stored sequentially after the header:

| Field | Encoding | Description |
| ----- | -------- | ----------- |
| gene_id | LP-string | SHA-256 based identifier (16 hex chars) |
| layer_name | LP-string | Original layer path (e.g., `blk.0.attn_q.weight`) |
| n_dims | u8 | Number of shape dimensions |
| shape | u32 × n_dims | Shape array (e.g., `[4096, 4096]`) |
| packed_data_len | u32 LE | Length of packed trit data in bytes |
| packed_data | bytes | Trit-packed tensor (5 trits per byte) |
| sources | LP-string | JSON array of source model names |
| metadata | LP-string | JSON object with extra attributes |

**LP-string** = Length-Prefixed string: `u32 LE` byte count followed by UTF-8 bytes.

### Trailer

| Field | Encoding | Description |
| ----- | -------- | ----------- |
| arch_compressed_len | u32 LE | Size of compressed architectures |
| architectures | zlib bytes | Compressed JSON: `{model_name: {layer → gene_id}}` |
| history_compressed_len | u32 LE | Size of compressed history |
| absorption_history | zlib bytes | Compressed JSON array (reserved) |

### Compression

- **Outer layer**: Zstandard (level 3) — entire payload
- **Architectures/History**: Zlib (level 6) — internal metadata
- **Fallback on load**: If zstd decompression fails, try reading raw bytes

### Example: Size Budget

For Llama 3.2 1B (67 tensors, ~1.2B params):

```text
Packed trits:    ~240 MB  (1.2B / 5 trits per byte)
Zstd compressed: ~170 MB  (level 3)
Metadata:        ~  1 KB  (JSON)
Total .nucleus:  ~170 MB
```

---

## .arktern Format (ARKT)

The **Arktern** format is produced by the Python ternary converter pipeline (`llama_ternary_converter.py`). It stores ternary-quantized tensors with metadata.

### Overview

```text
┌────────────────────────────────────────────┐
│ Header (64 bytes fixed)                    │
├────────────────────────────────────────────┤
│ Metadata (zlib-compressed JSON)            │
├────────────────────────────────────────────┤
│ Tensor Index (zlib-compressed JSON array)  │
├────────────────────────────────────────────┤
│ Tensor Data (concatenated packed bytes)    │
└────────────────────────────────────────────┘
```

### Header (64 bytes)

| Offset | Size | Type | Description |
| ------ | ---- | ---- | ----------- |
| 0 | 4 | bytes | Magic: `ARKT` (ASCII) |
| 4 | 4 | u32 LE | Format version |
| 8 | 8 | u64 LE | Number of tensors |
| 16 | 8 | u64 LE | Metadata section size (compressed) |
| 24 | 8 | u64 LE | Tensor index section size (compressed) |
| 32 | 8 | u64 LE | Total parameter count |
| 40 | 24 | bytes | Reserved (zeros) |

### Metadata Section

Zlib-compressed JSON object containing model information:

```json
{
    "model_name": "llama-3.2-1b",
    "source_format": "gguf",
    "total_params": 1235814400,
    "conversion_date": "2025-02-15T10:30:00",
    "converter_version": "2.0.0",
    "phi_optimization": true,
    "ads_cft_compression": false
}
```

### Tensor Index Section

Zlib-compressed JSON array. Each entry describes one tensor:

```json
{
    "name": "blk.0.attn_q.weight",
    "shape": [4096, 4096],
    "n_elements": 16777216,
    "packed_size": 3355444,
    "offset": 0,
    "scale": 0.0234375,
    "compression": "none",
    "phi_quality": 0.834
}
```

| Field | Type | Description |
| ----- | ---- | ----------- |
| name | string | Layer name |
| shape | int[] | Tensor dimensions |
| n_elements | int | Total number of parameters |
| packed_size | int | Size in bytes after trit packing |
| offset | int | Byte offset into tensor data section |
| scale | float | Original quantization scale (for reconstruction) |
| compression | string | `"none"`, `"zlib"`, or `"holographic"` |
| phi_quality | float | φ-based quality score (0.0 – 2.0) |

### Tensor Data Section

Raw concatenated packed bytes. Each tensor's data starts at `offset` and spans `packed_size` bytes. Data is trit-packed (5 trits per byte, base-3 encoding).

### Compression Types Per Tensor

| Type | Description |
| ---- | ----------- |
| `none` | Raw trit-packed bytes (most tensors) |
| `zlib` | Zlib-compressed packed bytes (high-sparsity tensors) |
| `holographic` | AdS/CFT holographic compression (experimental) |

---

## Format Comparison

| Feature | .arktern | .nucleus |
| ------- | -------- | -------- |
| Magic | `ARKT` (4B) | `ARKUNN02` (8B) |
| Header | Fixed 64B | Variable |
| Outer compression | None | Zstd level 3 |
| Metadata compression | Zlib | Zlib |
| Gene IDs | Not stored (computed on load) | Stored explicitly |
| Architecture map | Not stored | Stored (model → gene mapping) |
| Absorption history | Not stored | Stored (reserved) |
| Multi-model | Single model | Multiple models (gene merging) |
| Write support | Python only | Rust (save) |
| Primary use | Conversion output | Editing / storage |

---

## Trit Packing Reference

See [GENE_MODEL.md](GENE_MODEL.md#trit-codec) for the detailed trit packing algorithm.

Quick reference:

```text
Values:     -1  →  0
             0  →  1
            +1  →  2

Packing:    5 trits per byte
            byte = t₀ + 3·t₁ + 9·t₂ + 27·t₃ + 81·t₄
            Max byte value: 2+6+18+54+162 = 242

Efficiency: 8 bits per 5 trits = 1.6 bits/trit
            vs. theoretical: log₂(3) = 1.585 bits/trit
            Overhead: 0.95%
```

---

*Next: [GENE_MODEL.md](GENE_MODEL.md) — Gene data model*
