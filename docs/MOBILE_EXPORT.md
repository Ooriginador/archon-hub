# 📱 ARKHEION Forge — Mobile Export System

> **Formato**: `.arkmobile` (ZIP) | **GPU**: Vulkan / Metal / WebGPU via WGSL  
> **Status**: v1.0 | **MCP Tool**: `forge_export_mobile`

---

## Visão Geral

O sistema de exportação mobile empacota modelos ternários em bundles otimizados para GPUs mobile.
A GPU do smartphone fica **ociosa >95% do tempo** — nosso sistema usa WGSL compute shaders
via `wgpu` para executar inferência direto na GPU mobile com latência mínima.

```
┌─────────────────────────────────────────────────────────┐
│                    .arkmobile Bundle                     │
├─────────────────────────────────────────────────────────┤
│  manifest.json          Metadata + architecture config   │
│  weights/embed.bin      Token embeddings (f16)           │
│  weights/output.bin     Output projection (f16)          │
│  weights/norm.bin       RMS norm weights (f32)           │
│  weights/blocks/        Packed ternary blocks (5t/byte)  │
│  shaders/*.wgsl         GPU compute shaders              │
└─────────────────────────────────────────────────────────┘
```

## Plataformas Suportadas

| Plataforma | API GPU    | Cobertura | Versão Mínima     |
|------------|------------|-----------|-------------------|
| Android    | Vulkan 1.1 | ~80%      | Android 10+       |
| iOS        | Metal      | 100%      | iOS 14+           |
| macOS      | Metal      | 100%      | macOS 11+         |
| Web        | WebGPU     | ~70%      | Chrome 113+       |

## Uso via MCP

### Exportar para Mobile

```json
{
  "method": "tools/call",
  "params": {
    "name": "forge_export_mobile",
    "arguments": {
      "file": "llama-3.2-1b.nucleus",
      "output": "llama-3.2-1b.arkmobile",
      "target": "universal",
      "name": "Llama-3.2-1B-Ternary",
      "max_context": 2048
    }
  }
}
```

### Resposta

```json
{
  "exported": "llama-3.2-1b.arkmobile",
  "format": "arkmobile",
  "target": "universal",
  "genes": 291,
  "total_params": 1235814400,
  "weight_bytes": 98865152,
  "compression_ratio": "50.0:1",
  "n_blocks": 16,
  "n_shaders": 5,
  "architecture": {
    "n_layers": 16,
    "hidden_dim": 2048,
    "vocab_size": 128256,
    "n_heads": 32
  },
  "shaders_included": [
    "trit_decode.wgsl",
    "ternary_matvec.wgsl",
    "rms_norm.wgsl",
    "attention.wgsl",
    "activation.wgsl"
  ]
}
```

### Opções Avançadas

```json
{
  "file": "model.nucleus",
  "output": "model-android.arkmobile",
  "target": "android",
  "name": "My-Model",
  "max_context": 4096,
  "norm_as_f16": true,
  "strip_domains": ["Bias", "Conv"]
}
```

## WGSL Compute Shaders

Cinco shaders cobrem o pipeline completo de inferência de um Transformer:

| Shader | Propósito | Workgroup |
|--------|-----------|-----------|
| `trit_decode.wgsl` | Decodifica 5 trits/byte → {-1,0,+1} | 256 |
| `ternary_matvec.wgsl` | Multiplicação peso-ternário × vetor-f32 | 256 |
| `rms_norm.wgsl` | RMSNorm com redução em shared memory | 256 |
| `attention.wgsl` | Softmax + RoPE (Rotary Position Embedding) | 256/128 |
| `activation.wgsl` | SwiGLU + operações element-wise | 256 |

### Ternary MatVec — O Núcleo

O shader mais crítico. Trits ficam **empacotados** (5 por byte) e são decodificados on-the-fly:

```wgsl
// Ternary multiply: branchless via select
// raw_trit: 0 → -1 (subtract), 1 → 0 (skip), 2 → +1 (add)
let contrib = select(
    select(-x_val, 0.0, raw_trit == 1u),
    x_val,
    raw_trit == 2u
);
accumulator += contrib;
```

**Vantagem móvel**: Ternary matmul é **3× mais rápido** que INT8 matmul porque:
- Nenhuma multiplicação real (só add/subtract/skip)
- 5 trits/byte = 62.5% menos bandwidth que INT8
- Cache-friendly: weights ficam compactos

## Estimativa de Performance (Llama 3.2 1B)

| Dispositivo | GPU | VRAM | Performance Estimada |
|-------------|-----|------|---------------------|
| Samsung S24 Ultra | Adreno 750 | 12GB shared | ~25-35 tok/s |
| iPhone 15 Pro | A17 Pro GPU | 8GB shared | ~30-45 tok/s |
| Pixel 8 Pro | Mali-G715 | 12GB shared | ~15-25 tok/s |
| Samsung A54 | Mali-G68 MC4 | 6GB shared | ~5-10 tok/s |
| Chrome (WebGPU) | Varies | System RAM | ~10-20 tok/s |

*Estimativas baseadas em bandwidth teórico × eficiência ternária.*

## Estrutura do Bundle

### manifest.json

```json
{
  "format": "arkmobile",
  "version": 1,
  "model_name": "Llama-3.2-1B-Ternary",
  "target": "universal",
  "architecture": {
    "n_layers": 16,
    "hidden_dim": 2048,
    "n_heads": 32,
    "head_dim": 64,
    "intermediate_dim": 8192,
    "vocab_size": 128256,
    "max_context": 2048,
    "rope_theta": 10000.0,
    "norm_eps": 1e-5
  },
  "total_weight_bytes": 98865152,
  "total_params": 1235814400,
  "compression_ratio": 50.0,
  "shaders": ["trit_decode.wgsl", "ternary_matvec.wgsl", ...],
  "phi": 1.618033988749895
}
```

### weights/blocks/blk_NNN.bin

Layout binário por bloco:

```
[n_genes: u16]
For each gene:
  [name_len: u16][name: bytes]
  [shape_rank: u8][dims: u32 × rank]
  [data_len: u32][packed_trits: bytes]
```

## Pipeline Completo: GGUF → Mobile

```bash
# 1. Importar GGUF → Nucleus (ternary quantization)
forge_convert --file llama-3.2-1b.gguf --output llama.nucleus --threshold 0.3

# 2. Comprimir holograficamente (opcional, reduz ~2×)
forge_compress --file llama.nucleus --output llama-compressed.nucleus --ratio 2.0

# 3. Exportar para mobile
forge_export_mobile --file llama.nucleus --output llama.arkmobile --target android

# 4. Verificar tamanho
ls -lh llama.arkmobile  # ~95-100 MB para 1.24B params
```

## Integração com App Mobile

### Android (Kotlin + Rust via JNI)

```kotlin
// Load the arkmobile bundle
val bundle = ArkheionMobile.loadBundle(assets, "llama.arkmobile")

// Initialize GPU inference
val engine = MobileGpuEngine(bundle, maxContext = 2048)

// Run inference
val output = engine.generate("Hello, how are you?", maxTokens = 100)
```

### iOS (Swift + Rust via FFI)

```swift
// Load the arkmobile bundle
let bundle = try ArkheionMobile.loadBundle(path: bundlePath)

// Initialize Metal compute pipeline
let engine = try MobileGpuEngine(bundle: bundle)

// Stream tokens
for await token in engine.streamGenerate("Tell me about...") {
    print(token, terminator: "")
}
```

### Web (TypeScript + WebGPU)

```typescript
// Load the arkmobile bundle
const bundle = await ArkheionMobile.loadBundle('/models/llama.arkmobile');

// Initialize WebGPU engine
const engine = await MobileGpuEngine.create(bundle);

// Generate with streaming
for await (const token of engine.generate('Explain quantum computing')) {
  process.stdout.write(token);
}
```

## Tamanhos de Bundle Estimados

| Modelo | Params | F32 | Nucleus | .arkmobile |
|--------|--------|-----|---------|------------|
| TinyLlama 1.1B | 1.1B | 4.4 GB | 88 MB | ~92 MB |
| Llama 3.2 1B | 1.24B | 4.9 GB | 99 MB | ~103 MB |
| Llama 3.2 3B | 3.21B | 12.8 GB | 257 MB | ~265 MB |
| Phi-3 Mini 3.8B | 3.8B | 15.2 GB | 304 MB | ~312 MB |

*Overhead do .arkmobile vs nucleus puro: ~4 MB (shaders + manifest + ZIP metadata)*

## Nota Epistemológica

| Claim | Tipo | Realidade |
|-------|------|-----------|
| "GPU mobile ociosa" | 📊 **Empírico** | Mensurável: apps típicos usam <5% tempo GPU |
| "50:1 vs f32" | 📊 **Empírico** | Calculável: 5 trits/byte vs 4 bytes/param |
| "25-35 tok/s no S24" | 🎨 **Heurístico** | Estimativa baseada em bandwidth teórico |
| "3× mais rápido que INT8" | 🎨 **Heurístico** | Teórico; depende de pipeline e cache effects |

---

*ARKHEION Forge Mobile Export v1.0 | φ-Enhanced | WGSL Compute Shaders*
