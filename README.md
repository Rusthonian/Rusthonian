# 🦀 Rusthonian

**High-performance Python bindings for Rust crates**

[![CI](https://github.com/Rusthonian/Rusthonian/workflows/CI/badge.svg)](https://github.com/Rusthonian/Rusthonian/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/Rusthonian/Rusthonian)

Rusthonian is an umbrella project that provides Python bindings for high-quality Rust crates through PyO3.

## 🚀 Quick Start

### Install from PyPI (Easiest)

```bash
pip install Rusthonian
```

### Or Build from Source

```bash
git clone https://github.com/Rusthonian/Rusthonian.git
cd Rusthonian
pip install maturin
maturin develop --release
```

### Use It

```python
from Rusthonian import uuid, chrono

print(uuid.uuid4())

now = chrono.DateTime.now()
print(now.to_rfc3339())
```

## 📦 Included Modules

### UUID
Complete Python bindings for Rust's [`uuid` crate](https://docs.rs/uuid/).

- All UUID versions (v1-v8)
- **9.6+ million UUIDs/second** performance
- See [`uuid/README.md`](uuid/README.md) for full documentation

**Example:**
```python
from Rusthonian import uuid

u = uuid.uuid4()
print(u)
```

### Chrono
Complete Python bindings for Rust's [`chrono` crate](https://docs.rs/chrono/).

- DateTime with timezone support (UTC, Local, FixedOffset)
- Naive types (NaiveDateTime, NaiveDate, NaiveTime)
- Duration arithmetic and conversions
- **10-50x faster** than Python's datetime
- RFC3339 and RFC2822 parsing/formatting
- See [`chrono/README.md`](chrono/README.md) for full documentation

**Example:**
```python
from Rusthonian import chrono

now = chrono.DateTime.now()
tomorrow = now + chrono.Duration.days(1)
print(tomorrow.format("%Y-%m-%d %H:%M:%S"))
```

## 📥 Installation

```bash
pip install Rusthonian
```

That's it! No need to install Rust or build anything. Pre-built wheels are available for:
- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)  
- Windows (x64)

## 🛠️ Building

### Requirements
- Python 3.9+
- Rust (latest stable)
- Maturin (`pip install maturin`)

### Build Commands

```bash
# Development build (editable install)
maturin develop --release

# Production build (wheel)
maturin build --release

# Quick test
python -c "from Rusthonian import uuid; print(uuid.uuid4())"
```

### Python 3.13+

```bash
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
maturin develop --release
```

## 📖 Documentation

- **Quick Start**: See [`QUICKSTART.md`](QUICKSTART.md)
- **UUID Module**: See [`uuid/README.md`](uuid/README.md)
- **Chrono Module**: See [`chrono/README.md`](chrono/README.md)
- **Examples**: Check module-specific example directories

## 🧪 Testing

```bash
# Run all tests
./test_all.sh

# Or test individual modules
python uuid/test_comprehensive.py
python chrono/examples/chrono_example.py
```

## 🏗️ Project Structure

```
Rusthonian/
├── src/              # Main umbrella project
│   └── lib.rs
├── uuid/             # UUID module
│   ├── src/
│   ├── examples/
│   └── README.md
├── chrono/           # Chrono module  
│   ├── src/
│   ├── examples/
│   └── README.md
├── Rusthonian/       # Python package wrapper
│   └── __init__.py
├── Cargo.toml        # Rust config
└── pyproject.toml    # Python packaging
```

## 📄 License

Dual-licensed under MIT OR Apache-2.0

## 🤝 Contributing

Contributions welcome! Please:
1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Run tests: `./test_all.sh`
5. Submit a PR

## 📮 Links

- [GitHub](https://github.com/Rusthonian/Rusthonian)
- [Issues](https://github.com/Rusthonian/Rusthonian/issues)

---

Built with [PyO3](https://pyo3.rs/) ❤️
