# 🏛️ Archon Hub

> **The Integration Layer for Arkheion Sovereign AI.**

Archon Hub is the official distribution point for the **Archon SDK** and integration tools. It allows developers to connect their applications to the local Arkheion network and leverage high-performance ternary models.

## 📦 What's inside?

- **Archon SDK**: Professional Rust library to communicate with local Arkheion Daemons.
- **Archon CLI (Binary)**: Universal tool to compress GGUF models into HTCV4.
- **Archon Daemon (Binary)**: Local server for model management and inference.
- **Integration Manuals**: Complete guides on how to embed sovereign AI into your software.
- **API Reference**: Detailed technical specs for the Archon local endpoints.

## ⚙️ Binary Installation

The tools are available in the `bin/` directory for Linux x64:

```bash
# Give execution permissions
chmod +x bin/archon bin/archon-daemon

# Run the packager
./bin/archon pack --interactive
```

## 🚀 Getting Started

### 1. Install the SDK
Add `archon-sdk` to your project:

```toml
[dependencies]
archon-sdk = "0.9"
```

### 2. Connect to a Local Daemon
Ensure you have the `archon-daemon` running:

```bash
./bin/archon-daemon
```

Then, in your Rust code:

```rust
use archon_sdk::ArchonClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ArchonClient::default();
    let response = client.generate("Hello, Arkheion!").await?;
    println!("Response: {}", response.text);
    Ok(())
}
```

## 📚 Resources
- [Full Integration Guide](docs/integration_manual.md)
- [API Reference](docs/api_reference.md)

## 🛡️ License
Licensed under the MIT License.

---
**Powered by Arkheion Forge**
