# Infrastructure Guide

> Setup, build, and run instructions for Arkheion Forge.

---

## 1. Prerequisites

### Hardware
- **CPU**: x86_64 with AVX2 support (any modern AMD/Intel)
- **GPU** (optional but recommended): AMD GPU with ROCm support
  - Verified: RX 6700 XT, RX 7900 XTX, MI100, MI250
  - Requires: ROCm 6.x, `/dev/kfd` + `/dev/dri` access
- **RAM**: 16GB minimum, 32GB recommended
- **Storage**: 20GB for project + models

### Software
```bash
# Required
rustc 1.82+        # Rust toolchain
cargo               # Rust package manager
python 3.12+        # Python runtime
docker + compose    # Container runtime
gcc                 # C compiler (for Cython)

# Optional
rocm-dev            # AMD GPU acceleration
cython              # Python → C compilation
```

---

## 2. Intelligence Substrate (Docker)

The Forge requires three infrastructure services:

### 2.1 Start Services

```bash
cd arkheion-forge
docker compose -f docker-compose.elm-forge.yml up -d
```

### 2.2 Service Details

| Service | Image | Port | Purpose |
|---------|-------|------|---------|
| **elm-forge-postgres** | pgvector/pgvector:pg17 | 5433 | Vector embeddings + symbol index |
| **elm-forge-redis** | redis:7-alpine | 6380 | Hot cache + pattern store |
| **elm-forge-ollama** | ollama/ollama:rocm | 11435 | Embedding generation + LLM |

### 2.3 First-Time Setup

```bash
# Pull the embedding model
curl http://localhost:11435/api/pull -d '{"name": "nomic-embed-text"}'

# Verify services
curl http://localhost:11435/api/tags        # Ollama models
redis-cli -p 6380 ping                      # Redis
psql -h localhost -p 5433 -U devbrain devbrain -c "SELECT 1"  # Postgres
```

### 2.4 Environment Variables

```bash
# Postgres
POSTGRES_DB=devbrain
POSTGRES_USER=devbrain
POSTGRES_PASSWORD=devbrain_secret
DATABASE_URL=postgresql://devbrain:devbrain_secret@localhost:5433/devbrain

# Redis
REDIS_URL=redis://localhost:6380

# Ollama
OLLAMA_HOST=http://localhost:11435
EMBEDDING_MODEL=nomic-embed-text

# GPU (optional)
ROCR_VISIBLE_DEVICES=0
HSA_OVERRIDE_GFX_VERSION=10.3.0  # For RDNA2
```

---

## 3. Build the Rust Core

### 3.1 Standard Build

```bash
cd arkheion-forge
cargo build --release
```

### 3.2 With GPU Support

```bash
cargo build --release --features nucleus
```

### 3.3 Binaries Produced

| Binary | Description |
|--------|------------|
| `arkheion-forge` | GUI application (egui) |
| `forge-cli` | Command-line interface |
| `nucleus-phi-demo` | Φ-coherence demonstration |
| `runtime-feedback-server` | Feedback collection server |

---

## 4. Python AI Core

### 4.1 Setup Virtual Environment

```bash
cd arkheion-forge
python -m venv .venv
source .venv/bin/activate

pip install -r requirements.txt  # When created
# Or install key dependencies:
pip install fastapi pydantic redis asyncpg ollama xxhash
pip install cython numpy scipy scikit-learn
```

### 4.2 Index the Codebase

```bash
# Initialize DevBrain index (if using DevBrain MCP)
# This creates embeddings for all 530 Python files
python -c "
from ai_core.rag.rag_pipeline import RAGPipeline
pipeline = RAGPipeline()
pipeline.index_directory('ai-core/')
"
```

---

## 5. GPU Setup (AMD ROCm)

### 5.1 Verify Hardware

```bash
# Check for AMD GPU
lspci | grep -i amd
ls /dev/kfd /dev/dri/renderD128

# Check ROCm installation
rocminfo | head -20
```

### 5.2 Docker GPU Access

The `docker-compose.elm-forge.yml` already configures GPU passthrough:

```yaml
elm-forge-ollama:
  devices:
    - /dev/kfd
    - /dev/dri
```

### 5.3 Troubleshooting

```bash
# Permission denied on /dev/kfd
sudo usermod -aG render,video $USER
# Log out and back in

# Wrong GFX version
export HSA_OVERRIDE_GFX_VERSION=10.3.0  # RDNA2
export HSA_OVERRIDE_GFX_VERSION=11.0.0  # RDNA3
```

---

## 6. Project Structure After Setup

```
arkheion-forge/
├── .venv/                     # Python virtual environment
├── target/                    # Rust build output
│   └── release/
│       ├── arkheion-forge     # GUI binary
│       └── forge-cli          # CLI binary
├── crates/                    # Rust source
├── ai-core/                   # Python AI source
├── tests/                     # Test mirror lattice
├── docs/                      # Documentation
├── mcp-servers/               # MCP protocol servers
├── docker-compose.elm-forge.yml
└── Cargo.toml
```

---

## 7. Running

### 7.1 Launch GUI

```bash
./target/release/arkheion-forge
```

### 7.2 CLI Usage

```bash
# Load and inspect a model
./target/release/forge-cli load model.gguf

# Analyze model health
./target/release/forge-cli analyze model.gguf

# Prune low-importance genes
./target/release/forge-cli prune model.gguf --threshold 0.1 --output pruned.gguf

# Convert between formats
./target/release/forge-cli convert model.gguf --format safetensors --output model.safetensors
```

### 7.3 MCP Server

```bash
# Start the offtoken MCP server for VS Code integration
node mcp-servers/forge-offtoken.js
```
