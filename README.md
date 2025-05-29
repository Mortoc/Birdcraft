# 🐦 Birdcraft

A cross-platform mobile app built with Rust and Dioxus.

## Features

- ⚡ Fast and responsive UI built with Dioxus
- 🎨 Modern styling with inline CSS
- 📱 Mobile-first design
- 🦀 Written in Rust for performance and safety
- 🌐 Deployable as WASM for web

## Live Demo

The latest version is automatically deployed to GitHub Pages: [View Live Demo](https://[username].github.io/Birdcraft/)

## Development

### Prerequisites

- Rust (latest stable)
- wasm-pack for WASM builds

### Building for Web

```bash
# Install dependencies
cargo install wasm-pack

# Build WASM package
wasm-pack build --target web --out-dir pkg

# Serve locally
python3 -m http.server 8000
```

### Building for Mobile

```bash
# For Android (requires Android SDK)
cargo build --target aarch64-linux-android

# For iOS (requires Xcode)
cargo build --target aarch64-apple-ios
```

## Project Structure

```
src/
├── lib.rs          # WASM entry point
├── main.rs         # Native entry point
Cargo.toml          # Rust dependencies
index.html          # Web app entry point
.github/workflows/  # CI/CD automation
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally
5. Submit a pull request

## License

MIT License - see LICENSE file for details.