# Daisy Days

A high-performance MCP server and Zed extension for DaisyUI with 15 tools that generates production-ready UI layouts.

## Project Structure

This repository contains two main components:

### 1. MCP Server (`mcp-server/`)
A standalone Model Context Protocol server that provides DaisyUI component generation tools.

**Build:**
```bash
cd mcp-server
cargo build --release
```

**Run:**
```bash
./mcp-server/target/release/daisy_days
```

### 2. Zed Extension (Root & `zed-extensions/`)
A Zed editor extension that integrates DaisyUI tooling into the editor.

**Development:**
- Root directory contains the extension source code
- `zed-extensions/extensions/daisy-days/` is used by Zed for compilation

**Build:**
```bash
cargo build --target wasm32-wasip1 --release
```

**Install in Zed:**
1. Open Zed
2. Go to Extensions
3. Click "Install Dev Extension"
4. Select the `zed-extensions/extensions/daisy-days` directory

## Features

- 15 DaisyUI generation tools
- Production-ready UI layouts
- Support for multiple layout types: SaaS, Blog, Social, Kanban, and more
- Offline documentation access
- Design concept generation

## Version

Current version: 0.2.0

## Author

Ahmad Hamdi

## Repository

https://github.com/theHamdiz/daisy-days
