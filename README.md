# 🏛️ Archon Hub

> **The Integration Layer for Arkheion Sovereign AI.**

Archon Hub is the official distribution point for the **Archon SDK** and integration tools. It enables developers and organizations to leverage high-performance **Ternary Neural Networks** locally, without compromising data sovereignty or performance.

---

## 🧬 What is Archon?

Archon is the bridge between the complex world of **Arkheion Forge** and your application. It utilizes **HTCV4 (Holographic Ternary Compression)** to run large language models on consumer-grade hardware with extreme efficiency.

### 🌟 Key Advantages
*   **Extreme Compression**: Models that traditionally require 24GB of VRAM are compressed to run in less than 8GB.
*   **Total Sovereignty**: No API calls to external clouds. Your intelligence is local.
*   **Native Speed**: Written in pure Rust for bare-metal performance.

---

## 📦 What's inside this Repository?

-   **Archon SDK**: Professional Rust library for seamless integration.
-   **Archon CLI (Binaries)**: Interactive tool to compress standard GGUF models into the high-performance `.htcv4` format.
-   **Archon Daemon (Binaries)**: The local "Intelligent Router" that serves models via a high-speed REST API.
-   **Documentation**: Comprehensive guides and technical references.

---

## ⚙️ Installation & Usage

### 1. Ready the Tools (Linux x64)
The binaries are pre-compiled and located in the `bin/` folder.

```bash
# Set permissions
chmod +x bin/archon bin/archon-daemon

# Start the compression engine (Interactive mode)
./bin/archon pack --interactive
```

### 2. Launch the Intelligence Engine
Start the daemon to begin serving your local models:

```bash
./bin/archon-daemon
```

### 3. Integrate via SDK (Rust)
Add the SDK to your `Cargo.toml`:
```toml
[dependencies]
archon-sdk = "0.9"
```

Implement in your code:
```rust
use archon_sdk::ArchonClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ArchonClient::default();
    
    // The Archon Daemon automatically routes your request to the best specialist model.
    let response = client.generate("fn main() { }").await?;
    
    println!("Model: {}", response.model_used);
    println!("Output: {}", response.text);
    Ok(())
}
```

---

## 📚 Technical Documentation

Explore our deep-dive manuals to master the Archon ecosystem:
*   [**Integration Manual**](docs/integration_manual.md): Comprehensive guide for developers.
*   [**API Reference**](docs/api_reference.md): Technical specs for REST integration (Python, JS, etc).

---

## 🛡️ License
This project is licensed under the **MIT License**.

Built with 🧬 **Arkheion Forge** - *Democratizing Sovereign Intelligence.*
