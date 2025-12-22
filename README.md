# Daisy Days

Zed extension for DaisyUI. Provides documentation search, design concept references, and HTML layout generation.

## Version

0.3.0

## Requirements

- Rust 1.85+ (edition 2024)
- `wasm32-wasip1` target (`rustup target add wasm32-wasip1`)

## Build

### Windows

```powershell
.\scripts\build.ps1 build     # Build everything
.\scripts\build.ps1 server    # MCP server only
.\scripts\build.ps1 wasm      # WASM extension only
.\scripts\build.ps1 package   # Create target/dist/
.\scripts\build.ps1 install   # Install to local Zed
.\scripts\build.ps1 clean     # Clean artifacts
```

### Unix / macOS

```bash
./scripts/build.sh build      # Build everything
./scripts/build.sh server     # MCP server only
./scripts/build.sh wasm       # WASM extension only
./scripts/build.sh package    # Create target/dist/
./scripts/build.sh install    # Install to local Zed
./scripts/build.sh clean      # Clean artifacts
```

### With just

```bash
just -f scripts/justfile build
just -f scripts/justfile package
just -f scripts/justfile install-local
```

### Manual

```bash
# MCP server
cargo build --release -p daisy_days_mcp

# WASM extension
DAISY_SKIP_SERVER=1 cargo build --release --target wasm32-wasip1
```

## Output

| Artifact | Path |
|----------|------|
| MCP server (Windows) | `target/release/daisy_days.exe` |
| MCP server (Unix) | `target/release/daisy_days` |
| WASM extension | `target/wasm32-wasip1/release/daisy_days_extension.wasm` |
| Distribution package | `target/dist/` |

## Slash Commands

| Command | Description |
|---------|-------------|
| `/daisy-search <query>` | Search DaisyUI documentation |
| `/daisy-doc <name>` | Get documentation for a component |
| `/daisy-components` | List all components |
| `/daisy-concept <name>` | Get a design concept |
| `/daisy-concepts` | List all design concepts |
| `/daisy-layout <type> [title]` | Generate an HTML layout |
| `/daisy-layouts` | List layout types |

### Layout Types

`saas`, `blog`, `social`, `kanban`, `inbox`, `profile`, `docs`, `dashboard`, `auth`, `store`

### Design Concepts

`glassmorphism`, `neumorphism`, `darkmode`, `gradient`, `skeleton`, `responsive`

## Project Structure

```
daisy-days/
├── src/
│   ├── lib.rs          # Extension entry point
│   └── llms.txt        # DaisyUI documentation
├── mcp-server/
│   └── src/
│       ├── main.rs     # MCP server
│       └── llms.txt    # DaisyUI documentation
├── scripts/
│   ├── build.ps1       # Windows build script
│   ├── build.sh        # Unix build script
│   └── justfile        # just task runner
├── build.rs            # Cargo build script
├── extension.toml      # Zed extension manifest
└── Cargo.toml
```

## License

See LICENSE file.