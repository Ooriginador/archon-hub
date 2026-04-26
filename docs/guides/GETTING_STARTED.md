# Getting Started — Arkheion Forge

> From zero to parametric coding engine.

---

## Quick Start (5 minutes)

### 1. Start Intelligence Substrate

```bash
cd arkheion-forge
docker compose -f docker-compose.elm-forge.yml up -d
```

### 2. Build Rust Core

```bash
cargo build --release
```

### 3. Launch GUI

```bash
./target/release/arkheion-forge
```

---

## Full Setup (30 minutes)

### Step 1: Clone and Enter

```bash
# The project is already in arkheion-forge/
cd arkheion-forge
```

### Step 2: Start Docker Services

```bash
docker compose -f docker-compose.elm-forge.yml up -d

# Wait for services to stabilize
sleep 10

# Pull embedding model
curl http://localhost:11435/api/pull -d '{"name": "nomic-embed-text"}'
```

### Step 3: Setup Python Environment

```bash
python3.12 -m venv .venv
source .venv/bin/activate

# Install core dependencies
pip install \
  pydantic>=2.0 \
  redis>=5.0 \
  asyncpg>=0.29 \
  httpx>=0.25 \
  ollama>=0.1 \
  xxhash>=3.0 \
  numpy>=1.26 \
  scipy>=1.11 \
  scikit-learn>=1.3
```

### Step 4: Build Rust

```bash
# Standard build
cargo build --release

# With GPU support (requires ROCm)
cargo build --release --features nucleus
```

### Step 5: Verify Everything

```bash
# Check Docker services
docker compose -f docker-compose.elm-forge.yml ps

# Check Ollama
curl http://localhost:11435/api/tags

# Check Redis
redis-cli -p 6380 ping

# Check Postgres
psql -h localhost -p 5433 -U devbrain devbrain -c "SELECT 1"

# Check Rust binaries
./target/release/forge-cli --help
```

---

## Project Layout

```
arkheion-forge/
│
├── crates/                    # 🦀 Rust workspace
│   ├── forge-core/            # Tensor ops, formats, codecs
│   ├── forge-gpu/             # GPU acceleration
│   ├── forge-brain/           # AI reasoning
│   ├── forge-intel/           # Evolutionary intelligence
│   ├── forge-bridge/          # IPC bridges
│   ├── forge-editor/          # Code editor
│   ├── forge-ui/              # Native GUI
│   ├── forge-bank/            # Gene storage
│   ├── forge-mcp/             # MCP protocol
│   └── forge-python/          # Python bindings
│
├── ai-core/                   # 🐍 Python AI
│   ├── arkheion/              # Kernels + temporal + condensation
│   ├── hdcache/               # 5-layer cache
│   ├── neural_mesh/           # Cognitive lattice
│   ├── rag/                   # RAG pipeline
│   ├── dream/                 # Counterfactual engine
│   ├── autopilot/             # Autonomous sessions
│   ├── planning/              # Strategic planning
│   ├── learning/              # Self-improvement
│   ├── fine_tuning/           # Model training
│   ├── memory/                # Persistent memory
│   ├── system_crystallizer/   # Build engine
│   └── [many more modules]
│
├── tests/                     # 🧪 Mirror Lattice (401 files)
│   ├── mirrors/ai/            # AI module mirrors
│   └── unit/                  # Unit tests
│
├── mcp-servers/               # 🔌 MCP integration
│   └── forge-offtoken.js
│
├── docs/                      # 📚 Documentation
│   ├── architecture/          # System architecture
│   ├── subsystems/            # Module references
│   └── guides/                # How-to guides
│
├── src/                       # Rust binaries
├── Cargo.toml                 # Workspace manifest
└── docker-compose.elm-forge.yml
```

---

## What to Read Next

1. **[ARCHITECTURE.md](../architecture/ARCHITECTURE.md)** — Understand the full system
2. **[PARAMETER_SPACE_CODING.md](../architecture/PARAMETER_SPACE_CODING.md)** — The paradigm
3. **[SUBSYSTEM_MAP.md](../architecture/SUBSYSTEM_MAP.md)** — Module details
4. **[CRATE_REFERENCE.md](../subsystems/CRATE_REFERENCE.md)** — Rust crate docs
5. **[AI_CORE_REFERENCE.md](../subsystems/AI_CORE_REFERENCE.md)** — Python module docs

---

## Development Workflow

### Making Changes

```
1. Understand: Read the relevant module docs
2. Assess:    Check impact of changes
3. Implement: Write code + tests
4. Validate:  Run mirror lattice tests
5. Build:     Compile crystals
6. Verify:    Check Φ-coherence
```

### Running Tests

```bash
# Python tests
source .venv/bin/activate
pytest tests/ -v

# Rust tests
cargo test --workspace

# Specific subsystem
pytest tests/mirrors/ai/arkheion/ -v
cargo test -p forge-core
```

### Building Crystals (Cython)

```bash
# Compile Python to native binaries
python ai-core/system_crystallizer/_orchestrator.py
```
