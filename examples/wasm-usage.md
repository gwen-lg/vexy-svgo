# WebAssembly Usage Examples

This guide provides comprehensive examples for using Vexy SVGO's WebAssembly module in various environments including browsers, Node.js, Deno, and bundlers.

## Table of Contents

1. [Installation](#installation)
2. [Browser Usage](#browser-usage)
3. [Node.js Usage](#nodejs-usage)
4. [Bundler Integration](#bundler-integration)
5. [Advanced Usage](#advanced-usage)
6. [Performance Tips](#performance-tips)

## Installation

### NPM Package

```bash
npm install @vexy/svgo-wasm

# or with yarn
yarn add @vexy/svgo-wasm

# or with pnpm
pnpm add @vexy/svgo-wasm
```

### CDN Usage

```html
<script type="module">
  import init, { optimize } from 'https://unpkg.com/@vexy/svgo-wasm/web/vexy_svgo_wasm.js';
  
  await init();
  // Ready to use
</script>
```

## Browser Usage

### Basic Example

```html
<!DOCTYPE html>
<html>
<head>
  <title>Vexy SVGO WASM Example</title>
</head>
<body>
  <textarea id="input" placeholder="Paste SVG here..."></textarea>
  <button id="optimize">Optimize</button>
  <pre id="output"></pre>

  <script type="module">
    import init, { optimize } from './vexy_svgo_wasm.js';

    async function setupOptimizer() {
      // Initialize WASM module
      await init();

      document.getElementById('optimize').addEventListener('click', () => {
        const input = document.getElementById('input').value;
        
        try {
          const result = optimize(input);
          document.getElementById('output').textContent = result;
        } catch (error) {
          console.error('Optimization failed:', error);
        }
      });
    }

    setupOptimizer();
  </script>
</body>
</html>
```

### With Custom Configuration

```javascript
import init, { optimizeWithConfig } from '@vexy/svgo-wasm';

await init();

const config = {
  multipass: true,
  plugins: [
    { name: 'removeDoctype', active: true },
    { name: 'removeComments', active: true },
    { 
      name: 'cleanupIds', 
      params: { 
        minify: true,
        preserve: ['icon-'] 
      } 
    }
  ],
  js2svg: {
    pretty: true,
    indent: 2
  }
};

const svg = '<svg><!-- comment --><rect id="icon-rect"/></svg>';
const result = optimizeWithConfig(svg, JSON.stringify(config));
console.log(result);
```

### File Upload Example

```javascript
// Handle file upload and optimization
function setupFileUpload() {
  const fileInput = document.getElementById('fileInput');
  const output = document.getElementById('output');

  fileInput.addEventListener('change', async (event) => {
    const file = event.target.files[0];
    if (!file || !file.name.endsWith('.svg')) {
      alert('Please select an SVG file');
      return;
    }

    const text = await file.text();
    
    try {
      const optimized = optimize(text);
      
      // Display results
      output.innerHTML = `
        <h3>Optimization Results</h3>
        <p>Original size: ${text.length} bytes</p>
        <p>Optimized size: ${optimized.length} bytes</p>
        <p>Reduction: ${((1 - optimized.length / text.length) * 100).toFixed(1)}%</p>
        <pre>${escapeHtml(optimized)}</pre>
      `;
      
      // Offer download
      const blob = new Blob([optimized], { type: 'image/svg+xml' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = file.name.replace('.svg', '.min.svg');
      a.textContent = 'Download optimized SVG';
      output.appendChild(a);
    } catch (error) {
      output.innerHTML = `<p class="error">Error: ${error.message}</p>`;
    }
  });
}
```

## Node.js Usage

### Basic Example

```javascript
const { readFileSync, writeFileSync } = require('fs');
const { optimize, optimizeWithConfig } = require('@vexy/svgo-wasm');

// Read SVG file
const svg = readFileSync('input.svg', 'utf8');

// Optimize with default settings
const optimized = optimize(svg);
writeFileSync('output.svg', optimized);

// Optimize with custom config
const config = {
  multipass: true,
  plugins: ['removeDoctype', 'removeComments', 'cleanupIds']
};

const customOptimized = optimizeWithConfig(svg, JSON.stringify(config));
writeFileSync('output-custom.svg', customOptimized);
```

### Batch Processing

```javascript
const fs = require('fs').promises;
const path = require('path');
const { optimize } = require('@vexy/svgo-wasm');

async function optimizeDirectory(inputDir, outputDir) {
  // Ensure output directory exists
  await fs.mkdir(outputDir, { recursive: true });
  
  // Get all SVG files
  const files = await fs.readdir(inputDir);
  const svgFiles = files.filter(f => f.endsWith('.svg'));
  
  console.log(`Found ${svgFiles.length} SVG files`);
  
  // Process each file
  const results = await Promise.all(
    svgFiles.map(async (file) => {
      const inputPath = path.join(inputDir, file);
      const outputPath = path.join(outputDir, file);
      
      try {
        const input = await fs.readFile(inputPath, 'utf8');
        const output = optimize(input);
        await fs.writeFile(outputPath, output);
        
        return {
          file,
          originalSize: input.length,
          optimizedSize: output.length,
          reduction: ((1 - output.length / input.length) * 100).toFixed(1)
        };
      } catch (error) {
        return { file, error: error.message };
      }
    })
  );
  
  // Print summary
  console.log('\nOptimization Summary:');
  results.forEach(r => {
    if (r.error) {
      console.log(`❌ ${r.file}: ${r.error}`);
    } else {
      console.log(`✅ ${r.file}: ${r.originalSize}B → ${r.optimizedSize}B (-${r.reduction}%)`);
    }
  });
}

// Usage
optimizeDirectory('./svg-input', './svg-output');
```

### Stream Processing

```javascript
const { Transform } = require('stream');
const { optimize } = require('@vexy/svgo-wasm');

// Create a transform stream for SVG optimization
class SvgOptimizeStream extends Transform {
  constructor(options = {}) {
    super(options);
    this.chunks = [];
  }

  _transform(chunk, encoding, callback) {
    this.chunks.push(chunk);
    callback();
  }

  _flush(callback) {
    const svg = Buffer.concat(this.chunks).toString();
    
    try {
      const optimized = optimize(svg);
      this.push(optimized);
      callback();
    } catch (error) {
      callback(error);
    }
  }
}

// Usage with streams
const fs = require('fs');

fs.createReadStream('input.svg')
  .pipe(new SvgOptimizeStream())
  .pipe(fs.createWriteStream('output.svg'))
  .on('finish', () => console.log('Optimization complete'));
```

## Bundler Integration

### Webpack Plugin

```javascript
// webpack-svgo-plugin.js
const { optimize } = require('@vexy/svgo-wasm');

class VexySvgoWebpackPlugin {
  constructor(options = {}) {
    this.options = options;
  }

  apply(compiler) {
    compiler.hooks.emit.tapAsync('VexySvgoWebpackPlugin', (compilation, callback) => {
      const svgAssets = Object.keys(compilation.assets)
        .filter(name => name.endsWith('.svg'));

      svgAssets.forEach(assetName => {
        const asset = compilation.assets[assetName];
        const source = asset.source();

        try {
          const optimized = optimize(source, this.options);
          
          compilation.assets[assetName] = {
            source: () => optimized,
            size: () => optimized.length
          };
        } catch (error) {
          console.error(`Failed to optimize ${assetName}:`, error);
        }
      });

      callback();
    });
  }
}

// webpack.config.js
const VexySvgoWebpackPlugin = require('./webpack-svgo-plugin');

module.exports = {
  // ... other config
  plugins: [
    new VexySvgoWebpackPlugin({
      multipass: true,
      plugins: ['removeComments', 'cleanupIds']
    })
  ]
};
```

### Vite Plugin

```javascript
// vite-plugin-vexy-svgo.js
import { optimize } from '@vexy/svgo-wasm';

export default function vexySvgo(options = {}) {
  return {
    name: 'vite-plugin-vexy-svgo',
    
    transform(code, id) {
      if (!id.endsWith('.svg')) return null;
      
      try {
        const optimized = optimize(code, options);
        
        return {
          code: `export default ${JSON.stringify(optimized)}`,
          map: null
        };
      } catch (error) {
        this.error(`SVG optimization failed: ${error.message}`);
      }
    }
  };
}

// vite.config.js
import vexySvgo from './vite-plugin-vexy-svgo';

export default {
  plugins: [
    vexySvgo({
      multipass: true
    })
  ]
};
```

### Rollup Plugin

```javascript
// rollup-plugin-vexy-svgo.js
import { optimize } from '@vexy/svgo-wasm';
import { createFilter } from '@rollup/pluginutils';

export default function vexySvgo(options = {}) {
  const filter = createFilter(options.include || '**/*.svg', options.exclude);
  
  return {
    name: 'vexy-svgo',
    
    transform(code, id) {
      if (!filter(id)) return null;
      
      try {
        const optimized = optimize(code, options.svgoConfig || {});
        
        return {
          code: `export default ${JSON.stringify(optimized)};`,
          map: { mappings: '' }
        };
      } catch (error) {
        this.error(error);
      }
    }
  };
}
```

## Advanced Usage

### Web Worker Implementation

```javascript
// svgo.worker.js
import init, { optimize, optimizeWithConfig } from '@vexy/svgo-wasm';

let initialized = false;

self.addEventListener('message', async (event) => {
  const { type, data } = event.data;
  
  // Initialize WASM on first use
  if (!initialized) {
    await init();
    initialized = true;
  }
  
  try {
    let result;
    
    switch (type) {
      case 'optimize':
        result = optimize(data.svg);
        break;
        
      case 'optimizeWithConfig':
        result = optimizeWithConfig(data.svg, JSON.stringify(data.config));
        break;
        
      default:
        throw new Error(`Unknown message type: ${type}`);
    }
    
    self.postMessage({ 
      type: 'success', 
      result,
      stats: {
        originalSize: data.svg.length,
        optimizedSize: result.length,
        reduction: ((1 - result.length / data.svg.length) * 100).toFixed(1)
      }
    });
  } catch (error) {
    self.postMessage({ 
      type: 'error', 
      error: error.message 
    });
  }
});

// main.js - Using the worker
const worker = new Worker('./svgo.worker.js', { type: 'module' });

function optimizeInWorker(svg, config = null) {
  return new Promise((resolve, reject) => {
    const handler = (event) => {
      worker.removeEventListener('message', handler);
      
      if (event.data.type === 'success') {
        resolve(event.data);
      } else {
        reject(new Error(event.data.error));
      }
    };
    
    worker.addEventListener('message', handler);
    
    if (config) {
      worker.postMessage({ type: 'optimizeWithConfig', data: { svg, config } });
    } else {
      worker.postMessage({ type: 'optimize', data: { svg } });
    }
  });
}
```

### React Hook

```jsx
// useSvgOptimizer.js
import { useState, useCallback, useEffect } from 'react';
import init, { optimize, optimizeWithConfig } from '@vexy/svgo-wasm';

export function useSvgOptimizer() {
  const [isReady, setIsReady] = useState(false);
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [error, setError] = useState(null);

  // Initialize WASM module
  useEffect(() => {
    init().then(() => setIsReady(true));
  }, []);

  const optimizeSvg = useCallback(async (svg, config = null) => {
    if (!isReady) {
      throw new Error('WASM module not initialized');
    }

    setIsOptimizing(true);
    setError(null);

    try {
      const result = config 
        ? optimizeWithConfig(svg, JSON.stringify(config))
        : optimize(svg);

      const stats = {
        originalSize: svg.length,
        optimizedSize: result.length,
        reduction: ((1 - result.length / svg.length) * 100).toFixed(1)
      };

      return { svg: result, stats };
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setIsOptimizing(false);
    }
  }, [isReady]);

  return {
    optimizeSvg,
    isReady,
    isOptimizing,
    error
  };
}

// Usage in component
function SvgOptimizer() {
  const { optimizeSvg, isReady, isOptimizing, error } = useSvgOptimizer();
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');
  const [stats, setStats] = useState(null);

  const handleOptimize = async () => {
    try {
      const result = await optimizeSvg(input, {
        multipass: true,
        plugins: ['removeComments', 'cleanupIds']
      });
      
      setOutput(result.svg);
      setStats(result.stats);
    } catch (err) {
      console.error('Optimization failed:', err);
    }
  };

  return (
    <div>
      <textarea 
        value={input} 
        onChange={e => setInput(e.target.value)}
        placeholder="Paste SVG here..."
      />
      
      <button 
        onClick={handleOptimize} 
        disabled={!isReady || isOptimizing}
      >
        {isOptimizing ? 'Optimizing...' : 'Optimize'}
      </button>
      
      {error && <div className="error">{error}</div>}
      
      {stats && (
        <div className="stats">
          <p>Original: {stats.originalSize} bytes</p>
          <p>Optimized: {stats.optimizedSize} bytes</p>
          <p>Reduction: {stats.reduction}%</p>
        </div>
      )}
      
      {output && <pre>{output}</pre>}
    </div>
  );
}
```

### Deno Example

```typescript
// optimize.ts
import init, { optimize } from "https://deno.land/x/vexy_svgo_wasm/mod.ts";

await init();

// Read file
const svg = await Deno.readTextFile("input.svg");

// Optimize
const optimized = optimize(svg);

// Write result
await Deno.writeTextFile("output.svg", optimized);

console.log(`Optimized ${svg.length} bytes to ${optimized.length} bytes`);
```

## Performance Tips

### 1. Initialization

Initialize the WASM module once and reuse it:

```javascript
// ❌ Bad - initializes on every call
async function optimizeSvg(svg) {
  await init();
  return optimize(svg);
}

// ✅ Good - initialize once
let initialized = false;

async function optimizeSvg(svg) {
  if (!initialized) {
    await init();
    initialized = true;
  }
  return optimize(svg);
}
```

### 2. Batch Processing

Process multiple SVGs efficiently:

```javascript
// ❌ Bad - sequential processing
for (const svg of svgFiles) {
  const result = optimize(svg);
  results.push(result);
}

// ✅ Good - parallel processing
const results = await Promise.all(
  svgFiles.map(svg => optimize(svg))
);
```

### 3. Memory Management

For large files, consider chunking:

```javascript
async function optimizeLargeFile(filePath) {
  const stats = await fs.stat(filePath);
  
  // For files over 10MB, use streaming
  if (stats.size > 10 * 1024 * 1024) {
    // Implement streaming optimization
    return optimizeWithStreaming(filePath);
  }
  
  // Otherwise, read entire file
  const svg = await fs.readFile(filePath, 'utf8');
  return optimize(svg);
}
```

### 4. Error Handling

Always wrap optimization in try-catch:

```javascript
function safeOptimize(svg, fallbackToOriginal = true) {
  try {
    return optimize(svg);
  } catch (error) {
    console.error('Optimization failed:', error);
    
    if (fallbackToOriginal) {
      return svg; // Return original if optimization fails
    }
    
    throw error;
  }
}
```

### 5. Configuration Caching

Cache parsed configurations:

```javascript
const configCache = new Map();

function getCachedConfig(config) {
  const key = JSON.stringify(config);
  
  if (!configCache.has(key)) {
    configCache.set(key, JSON.stringify(config));
  }
  
  return configCache.get(key);
}

function optimizeWithCachedConfig(svg, config) {
  const configString = getCachedConfig(config);
  return optimizeWithConfig(svg, configString);
}
```

## Troubleshooting

### Common Issues

1. **Module not initialized**: Always await `init()` before using optimization functions
2. **Invalid SVG**: Ensure input is valid SVG XML
3. **Memory limits**: For very large files, consider using Node.js instead of browser
4. **CORS issues**: When loading WASM from CDN, ensure proper CORS headers

### Debug Mode

Enable debug logging:

```javascript
// Set debug flag before init
window.VEXY_SVGO_DEBUG = true;

await init();
// Now optimization will log debug information
```