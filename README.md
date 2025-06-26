# 🦀 Rust WebAssembly + 🔄 HTMX Demo

A modern web application demonstrating the integration of Rust WebAssembly with HTMX for dynamic interactions.

## 🚀 Features

- **Rust WebAssembly**: High-performance calculations compiled to WASM
- **HTMX Integration**: Dynamic content updates without complex JavaScript
- **WebSocket Server**: Real-time communication built with Rust + Tokio
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

2. **Start the development server** (builds WASM and starts Express server):
   ```bash
   npm start
   ```

3. **Start the WebSocket server** (in a separate terminal):
   ```bash
   npm run websocket
   ```

   **Or start both servers simultaneously**:
   ```bash
   npm run dev:full
   ```

   Manual commands:
   ```bash
   npm run build:rust
   npm run server          # Express server on port 3000
   npm run websocket       # WebSocket server on port 8080
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

### WebSocket Real-time Communication
- **Bidirectional**: Real-time communication between clients
- **Message Broadcasting**: Messages sent to all connected clients
- **WASM Integration**: Send calculation results via WebSocket
- **Chat System**: Simple real-time chat functionality
- **Auto-reconnect**: Automatically reconnects on connection loss

### Express Server
- **Simple**: Lightweight Node.js web server
- **API Endpoints**: RESTful endpoints for HTMX integration
- **Static Files**: Serves HTML, CSS, and WASM files

## 🔧 Available Scripts

- `npm run build:rust` - Build Rust to WebAssembly
- `npm run server` - Start Express server (port 3000)
- `npm run websocket` - Start WebSocket server (port 8080)
- `npm run dev:full` - Start both Express and WebSocket servers
- `npm start` - Build Rust and start Express server

## 🧪 Testing the Integration

1. **WASM Functions**: Use the calculator and math functions
2. **HTMX Requests**: Click "Get Server Time" and "Echo Text" buttons
3. **Combined Usage**: Try the "Calculate Random Sum & Send to Server" feature
4. **WebSocket Real-time**: 
   - Send calculations via WebSocket to broadcast to all clients
   - Use the chat system to send messages in real-time
   - Open multiple browser tabs to see real-time updates
5. **Browser Console**: Check for WASM loading logs and WebSocket connection status

## 🔗 Technology Stack

- **[Rust](https://rust-lang.org/)** - Systems programming language
- **[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)** - Rust-WebAssembly bindings
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime for Rust
- **[tokio-tungstenite](https://github.com/snapview/tokio-tungstenite)** - WebSocket implementation for Tokio
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
- Check if ports 3000 (Express) and 8080 (WebSocket) are available
- Try `npm run build:rust && npm run dev:full`
- Ensure all dependencies are installed

### WebSocket Connection Issues
- Ensure the WebSocket server is running: `npm run websocket`
- Check if port 8080 is available and not blocked by firewall
- WebSocket connects to `ws://localhost:8080` - ensure no proxy interference
- Check browser console for WebSocket connection errors

## 📚 Learning Resources

- [Rust WebAssembly Book](https://rustwasm.github.io/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [WebSocket with Rust](https://blog.logrocket.com/websockets-tutorial-rust/)
- [HTMX Documentation](https://htmx.org/docs/)
- [Express.js Guide](https://expressjs.com/en/guide/routing.html)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

## 🚀 Railway Deployment

Deploy this application to Railway with these steps:

### Prerequisites
- [Railway account](https://railway.app/)
- [Railway CLI](https://docs.railway.app/deployment/cli) (optional)

### Deployment Steps

1. **Via Railway Dashboard**:
   ```bash
   # Push your code to GitHub
   git add .
   git commit -m "Ready for Railway deployment"
   git push origin main
   ```
   
   - Go to [Railway Dashboard](https://railway.app/dashboard)
   - Click "New Project" 
   - Select "Deploy from GitHub repo"
   - Choose this repository
   - Railway will auto-detect and deploy!

2. **Via Railway CLI**:
   ```bash
   # Install Railway CLI
   npm install -g @railway/cli
   
   # Login and deploy
   railway login
   railway init
   railway up
   ```

### Configuration

Railway will automatically:
- ✅ Detect Node.js and Rust
- ✅ Install wasm-pack and build tools
- ✅ Build the Rust WASM module
- ✅ Start the Express server
- ✅ Assign a public URL

The deployed app includes:
- 🎮 32x32 Grid Game (fully functional)
- 🦀 Rust WASM calculations
- 🔄 HTMX dynamic content
- 📱 Responsive design

**Note**: WebSocket features are disabled in production for simplicity. The core WASM grid functionality works perfectly!

### Environment Variables

Railway automatically sets:
- `PORT` - Server port (assigned by Railway)
- `NODE_ENV=production` - Production mode

## 🤝 Contributing

Feel free to open issues or submit pull requests for improvements!

## 📄 License

MIT License - feel free to use this code for learning and projects. 