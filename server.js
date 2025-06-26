const express = require('express');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Serve static files from the current directory
app.use(express.static('.'));

// HTMX API Routes
app.get('/api/time', (req, res) => {
  const now = new Date();
  const timeString = now.toLocaleString();
  
  res.send(`
    <div style="color: #4caf50; font-weight: bold;">
      🕒 Server Time: ${timeString}
      <br><small>Loaded via HTMX GET request</small>
    </div>
  `);
});

app.post('/api/echo', (req, res) => {
  const echoInput = req.body['echo-input'];
  
  if (!echoInput) {
    return res.send(`
      <div style="color: #f44336;">
        ❌ No text provided to echo
      </div>
    `);
  }
  
  res.send(`
    <div style="color: #2196f3; font-weight: bold;">
      🔊 Echo: "${echoInput}"
      <br><small>Processed via HTMX POST request</small>
    </div>
  `);
});

app.post('/api/wasm-result', (req, res) => {
  try {
    const { result, num1, num2 } = req.body;
    
    console.log('Received WASM calculation result:', { result, num1, num2 });
    
    res.json({
      success: true,
      message: `Received WASM result: ${result}`,
      calculation: `${num1} + ${num2} = ${result}`,
      timestamp: new Date().toISOString()
    });
  } catch (error) {
    res.status(400).json({
      success: false,
      error: 'Invalid request body'
    });
  }
});

// Serve the main HTML file
app.get('/', (req, res) => {
  res.sendFile(path.join(__dirname, 'index.html'));
});

// Start server
app.listen(PORT, () => {
  console.log(`🚀 Server running at http://localhost:${PORT}`);
  console.log(`📁 Serving files from: ${__dirname}`);
  console.log(`🦀 WASM files should be available at: /pkg/`);
  console.log(`🔄 HTMX endpoints available at: /api/*`);
});

module.exports = app; 