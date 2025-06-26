# 🦀 Rust WebAssembly + 🔄 HTMX Demo

A modern web application demonstrating the integration of Rust WebAssembly with HTMX for dynamic interactions.

## 🚀 Features

- **Rust WebAssembly**: High-performance calculations compiled to WASM
- **HTMX Integration**: Dynamic content updates without complex JavaScript
- **Express Server**: Simple Node.js server for API endpoints
- **Modern UI**: Beautiful, responsive design with CSS gradients and animations

## 📋 Prerequisites

Make sure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18+ recommended)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Installing wasm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Or via npm:
```bash
npm install -g wasm-pack
```

## 🛠️ Setup & Installation

1. **Install JavaScript dependencies**:
   ```bash
   npm install
   ```

2. **Start the development server** (builds WASM and starts server):
   ```bash
   npm start
   ```

   Or manually:
   ```bash
   npm run build:rust
   npm run server
   ```

## 🏗️ Project Structure

```
rust-frame-host/
├── src/
│   └── lib.rs                 # Rust WASM source code
├── public/
│   └── manifest.json          # PWA manifest
├── pkg/                       # Generated WASM output (after build)
├── index.html                 # Main HTML page
├── server.js                  # Express server with API endpoints
├── Cargo.toml                 # Rust dependencies
├── package.json               # Node.js dependencies
└── vite.config.ts             # Vite configuration
```

## 🎯 Demo Features

### WebAssembly Functions
- **Simple Functions**: `greet()`, `add()`, `factorial()`
- **Complex Types**: Calculator class with methods
- **Memory Management**: Automatic cleanup via wasm-bindgen

### HTMX Interactions
- **GET Requests**: Fetch server time dynamically
- **POST Requests**: Echo form data processing
- **Integration**: Combine WASM calculations with server requests

### Express Server
- **Simple**: Lightweight Node.js web server
- **API Endpoints**: RESTful endpoints for HTMX integration
- **Static Files**: Serves HTML, CSS, and WASM files

## 🔧 Available Scripts

- `npm run build:rust` - Build Rust to WebAssembly
- `npm run server` - Start Express server
- `npm start` - Build Rust and start server

## 🧪 Testing the Integration

1. **WASM Functions**: Use the calculator and math functions
2. **HTMX Requests**: Click "Get Server Time" and "Echo Text" buttons
3. **Combined Usage**: Try the "Calculate Random Sum & Send to Server" feature
4. **Browser Console**: Check for WASM loading logs and calculation results

## 🔗 Technology Stack

- **[Rust](https://rust-lang.org/)** - Systems programming language
- **[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)** - Rust-WebAssembly bindings
- **[Express.js](https://expressjs.com/)** - Fast web framework for Node.js
- **[HTMX](https://htmx.org/)** - Dynamic HTML with minimal JavaScript
- **[Node.js](https://nodejs.org/)** - JavaScript runtime environment

## 🚨 Troubleshooting

### WASM Module Not Loading
- Ensure `wasm-pack` is installed and `pkg/` directory exists
- Run `npm run build:rust` to regenerate WASM files
- Check browser console for loading errors

### Build Errors
- Verify Rust toolchain is up to date: `rustup update`
- Clear build cache: `rm -rf target/ pkg/ node_modules/`
- Reinstall dependencies: `npm install`

### Development Server Issues
- Check if port 3000 is available
- Try `npm run build:rust && npm run server`
- Ensure all dependencies are installed

## 📚 Learning Resources

- [Rust WebAssembly Book](https://rustwasm.github.io/book/)
- [HTMX Documentation](https://htmx.org/docs/)
- [Express.js Guide](https://expressjs.com/en/guide/routing.html)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

## 🤝 Contributing

Feel free to open issues or submit pull requests for improvements!

## 📄 License

MIT License - feel free to use this code for learning and projects. 