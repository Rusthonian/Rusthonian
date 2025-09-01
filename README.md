# Rusthonian

A super project providing Python bindings for various Rust crates via PyO3.

## Structure

This is a super project that links to submodules for different Rust crate bindings:

- **UUID**: Python bindings for the Rust UUID crate (currently local, will be separate repository)
- **Future modules**: More bindings will be added as separate repositories

## Development Setup

### Prerequisites

- Rust (latest stable)
- Python 3.9+ (with PyO3 ABI3 compatibility for Python 3.13+)
- Git (optional, for submodule management)

### Setup

**IMPORTANT**: This project uses Git submodules. You must clone with the `--recursive` flag!

1. Clone the repository with submodules:
```bash
git clone --recursive https://github.com/Rusthonian/Rusthonian.git
cd Rusthonian
```

**If you already cloned without `--recursive`**, run this to get the submodules:
```bash
git submodule update --init --recursive
```

2. Set up submodules (if using Git):
```bash
./scripts/setup_submodules.sh
```

Or manually:
```bash
git submodule init
git submodule update --init --recursive
```

**Note**: If you're not using Git or this isn't a Git repository, the UUID module is available locally in the `UUID/` directory and can be built directly.

## Building

### Build without features (basic module only)
```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build
```

### Build with UUID support
```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --features uuid
```

### Build for development
```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --features uuid --release
```

## Usage

Once built, you can use the Python module:

```python
import Rusthonian

# Access UUID functionality (if built with uuid feature)
from Rusthonian.UUID import UUID, new_v4

# Generate a new UUID
uuid = new_v4()
print(uuid)
```

## Current Status

- ✅ **Main project**: Compiles successfully
- ✅ **UUID module**: Available locally, compiles with feature flag
- 🔄 **Git submodules**: Ready to be set up when repository is initialized
- 📋 **Future modules**: Structure ready for boost and other bindings

## CI/CD with GitHub Actions

This project includes GitHub Actions workflows for automated testing and publishing:

### Test Workflow
- **Triggers**: Push to main/master, pull requests, manual dispatch
- **Tests**: Builds against Python 3.9-3.13 with and without features
- **Matrix testing**: Tests different Python versions and feature combinations

### Publish Workflow
- **Trigger**: Manual dispatch only
- **Features**:
  - Builds and publishes to PyPI
  - Creates GitHub releases automatically
  - Configurable version, features, and Python version
  - Requires PyPI API token in repository secrets

### Setting up PyPI Publishing

1. Create a PyPI account and API token
2. Add the token to your GitHub repository secrets:
   - Go to Settings → Secrets and variables → Actions
   - Add a new secret named `PYPI_API_TOKEN`
   - Paste your PyPI API token

3. To publish a new version:
   - Go to Actions → Publish to PyPI
   - Click "Run workflow"
   - Fill in the version and features
   - Click "Run workflow"

## Troubleshooting

### "No such file or directory" error for UUID
If you get an error like:
```
failed to read `/path/to/Rusthonian/UUID/Cargo.toml`
No such file or directory (os error 2)
```

This means the submodules weren't downloaded. Run:
```bash
git submodule update --init --recursive
```

### Empty UUID directory
If the UUID directory is empty, the submodule wasn't initialized. Run:
```bash
git submodule update --init --recursive
```

## Adding New Submodules

1. Create a new repository for your binding (e.g., `rusthonian-uuid`)
2. Add it to `.gitmodules`:
```
[submodule "uuid"]
    path = uuid
    url = https://github.com/Rusthonian/uuid.git
    branch = main
```
3. Add the dependency to `Cargo.toml`:
```toml
[features]
boost = ["rusthonian-boost"]

[dependencies]
rusthonian-boost = { path = "boost", optional = true }
```
4. Update `src/lib.rs` to include the new module
5. Run `git submodule add <url> <path>`

## Notes

- The `PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1` environment variable is needed for Python 3.13+ compatibility
- Each submodule should be a separate repository that can be used independently
- The super project provides a unified interface to all bindings
- Currently working with local UUID module; will transition to Git submodules when repository is set up
