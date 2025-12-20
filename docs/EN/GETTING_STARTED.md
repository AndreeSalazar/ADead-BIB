# 🚀 Getting Started with ADead-BIB

> **Author:** Eddi Andreé Salazar Matos | **Made with ❤️ in Peru** 🇵🇪

---

## Prerequisites

### 1. Install Rust

```bash
# Windows (PowerShell)
winget install Rustlang.Rust.MSVC

# Or download from https://rustup.rs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install Python 3.8+

```bash
# Windows
winget install Python.Python.3.11

# Or download from https://python.org
```

### 3. Install Python Dependencies

```bash
pip install numpy
```

### 4. (Optional) Install Ollama for AI Demos

```bash
# Install Ollama
winget install Ollama.Ollama

# Download a small model
ollama pull tinyllama

# Start Ollama server (in a separate terminal)
ollama serve
```

---

## Quick Start

### Step 1: Build the Compiler

```powershell
cd ADead-BIB
cargo build --release
```

### Step 2: Write Your First Program

Create a file `examples/my_first.adB`:

```python
def main():
    print("Hello from ADead-BIB!")
    x = 10
    y = 20
    print(x + y)
```

### Step 3: Compile and Run

```powershell
cargo run --release examples/my_first.adB
.\my_first.exe
```

**Output:**
```
Hello from ADead-BIB!
30
```

---

## AI Integration

### Basic AI Demo (0.19 MB RAM)

```powershell
cd python
python ai_complete.py
```

### Scalable AI with BPE (0.82 MB RAM)

```powershell
python ai_scalable.py
```

### With Ollama (Requires Ollama Running)

```powershell
# In terminal 1: Start Ollama
ollama serve

# In terminal 2: Run demo
python ollama_integration.py
```

---

## Project Structure

```
ADead-BIB/
├── src/rust/           # Compiler source
├── examples/           # Example .adB files
├── python/             # Python AI integration
├── build/              # Compiled executables
└── docs/               # Documentation
    ├── EN/             # English docs
    └── ES/             # Spanish docs
```

---

## Next Steps

1. **Learn the syntax:** See `SYNTAX.md`
2. **Explore examples:** Check `examples/` folder
3. **Try AI features:** Run Python demos
4. **Read the roadmap:** See `docs/IDEAS/`

---

**Created by Eddi Andreé Salazar Matos** | eddi.salazar.dev@gmail.com | 🇵🇪
