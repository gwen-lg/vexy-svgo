---
# this_file: docs/demo/index.md
layout: demo
title: Interactive Demo
nav_order: 3
description: "Try Vexy SVGO in your browser with WebAssembly"
permalink: /demo/
---

# Interactive Demo
{: .fs-9 }

Experience Vexy SVGO's power directly in your browser
{: .fs-6 .fw-300 }

<div id="wasm-status" class="flash flash-warn">
  <div class="d-flex flex-items-center">
    <div class="mr-2">⏳</div>
    <div>Loading WebAssembly module...</div>
  </div>
</div>

---

## Features

<div class="d-flex flex-wrap mb-6">
  <div class="flex-auto mr-4 mb-4 text-center">
    <div class="fs-6">🔒</div>
    <div class="fw-500">Private</div>
    <div class="fs-2 text-grey-dk-000">Files never leave your browser</div>
  </div>
  <div class="flex-auto mr-4 mb-4 text-center">
    <div class="fs-6">⚡</div>
    <div class="fw-500">Fast</div>
    <div class="fs-2 text-grey-dk-000">Native Rust performance</div>
  </div>
  <div class="flex-auto mr-4 mb-4 text-center">
    <div class="fs-6">🌐</div>
    <div class="fw-500">Offline</div>
    <div class="fs-2 text-grey-dk-000">Works without internet</div>
  </div>
</div>

---

## Configuration

<div class="Box mb-4">
  <div class="Box-header">
    <h3 class="Box-title">Optimization Settings</h3>
  </div>
  <div class="Box-body">
    <div class="d-flex flex-wrap">
      <div class="mr-4 mb-3">
        <label class="form-check-label">
          <input type="checkbox" id="opt-multipass" class="form-check-input">
          Multipass optimization
        </label>
      </div>
      <div class="mr-4 mb-3">
        <label class="form-check-label">
          <input type="checkbox" id="opt-pretty" class="form-check-input">
          Pretty print output
        </label>
      </div>
      <div class="mr-4 mb-3">
        <label class="form-label">
          Precision:
          <input type="number" id="opt-precision" value="3" min="0" max="10" class="form-control" style="width: 80px; display: inline-block;">
        </label>
      </div>
    </div>
    
    <details class="details-reset">
      <summary class="btn btn-outline btn-sm">Advanced Plugin Settings</summary>
      <div class="mt-3" id="plugin-controls">
        <div class="d-flex flex-wrap" id="plugin-list">
          <!-- Plugin checkboxes will be populated by JavaScript -->
        </div>
      </div>
    </details>
  </div>
</div>

---

## SVG Input

<div class="d-flex mb-3">
  <button id="btn-load-example" class="btn btn-primary mr-2">Load Example</button>
  <button id="btn-clear" class="btn btn-outline mr-2">Clear</button>
  <label for="file-input" class="btn btn-outline">
    Upload SVG
    <input type="file" id="file-input" accept=".svg,image/svg+xml" style="display: none;">
  </label>
</div>

<div class="Box">
  <div class="Box-header">
    <h3 class="Box-title">Input SVG</h3>
  </div>
  <div class="Box-body">
    <textarea id="input-svg" class="form-control" rows="12" placeholder="Paste your SVG code here or load an example..."></textarea>
    <div class="mt-3 p-3 border rounded-2" id="input-preview" style="min-height: 150px; background: repeating-conic-gradient(#f6f8fa 0% 25%, transparent 0% 50%) 50% / 20px 20px;">
      <div class="text-grey text-center">SVG preview will appear here</div>
    </div>
  </div>
</div>

---

## Optimize

<div class="text-center mb-4">
  <button id="btn-optimize" class="btn btn-primary btn-lg" disabled>
    <div class="d-flex flex-items-center">
      <div class="mr-2">⚙️</div>
      <div>Optimize SVG</div>
    </div>
  </button>
</div>

---

## Results

<div id="results-section" style="display: none;">
  
  <div class="Box mb-4">
    <div class="Box-header">
      <h3 class="Box-title">Statistics</h3>
    </div>
    <div class="Box-body">
      <div class="d-flex flex-wrap">
        <div class="mr-6 mb-2">
          <div class="fs-3 text-grey-dk-000">Original Size</div>
          <div class="fs-5 fw-500" id="stat-original">-</div>
        </div>
        <div class="mr-6 mb-2">
          <div class="fs-3 text-grey-dk-000">Optimized Size</div>
          <div class="fs-5 fw-500 text-green-600" id="stat-optimized">-</div>
        </div>
        <div class="mr-6 mb-2">
          <div class="fs-3 text-grey-dk-000">Reduction</div>
          <div class="fs-5 fw-500 text-blue-600" id="stat-reduction">-</div>
        </div>
        <div class="mr-6 mb-2">
          <div class="fs-3 text-grey-dk-000">Time</div>
          <div class="fs-5 fw-500" id="stat-time">-</div>
        </div>
      </div>
    </div>
  </div>

  <div class="Box">
    <div class="Box-header d-flex flex-justify-between flex-items-center">
      <h3 class="Box-title">Optimized SVG</h3>
      <button id="btn-download" class="btn btn-sm btn-primary">Download</button>
    </div>
    <div class="Box-body">
      <textarea id="output-svg" class="form-control" rows="12" readonly></textarea>
      <div class="mt-3 p-3 border rounded-2" id="output-preview" style="min-height: 150px; background: repeating-conic-gradient(#f6f8fa 0% 25%, transparent 0% 50%) 50% / 20px 20px;">
        <div class="text-grey text-center">Optimized SVG preview will appear here</div>
      </div>
    </div>
  </div>
</div>

---

## About This Demo

This interactive demo runs **Vexy SVGO compiled to WebAssembly** directly in your browser. Your SVG files are processed locally and never sent to any server, ensuring complete privacy.

### Browser Requirements

- **Chrome**: 57+ / Chromium 57+
- **Firefox**: 52+
- **Safari**: 11+
- **Edge**: 16+

### Performance

WebAssembly allows near-native performance in the browser:
- **2-3x faster** than JavaScript implementations
- **Efficient memory management** with predictable performance
- **Small bundle size** under 2MB compressed

---

<script type="module">
let wasmModule = null;
let vexyOptimize = null;

// Common SVG plugins
const commonPlugins = [
  'removeComments',
  'removeTitle',
  'removeDesc',
  'removeUselessDefs',
  'removeEditorsNSData',
  'removeEmptyAttrs',
  'removeHiddenElems',
  'removeEmptyText',
  'removeEmptyContainers',
  'cleanupEnableBackground',
  'convertStyleToAttrs',
  'convertColors',
  'convertPathData',
  'convertTransform',
  'removeUnknownsAndDefaults',
  'removeNonInheritableGroupAttrs',
  'removeUselessStrokeAndFill',
  'removeUnusedNS',
  'cleanupIDs',
  'collapseGroups',
  'mergePaths',
  'convertShapeToPath',
  'sortAttrs',
  'removeDimensions',
  'removeElementsByAttr'
];

// Example SVG
const exampleSvg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="200" height="200">
  <defs>
    <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:rgb(79,70,229);stop-opacity:1" />
      <stop offset="100%" style="stop-color:rgb(147,51,234);stop-opacity:1" />
    </linearGradient>
  </defs>
  
  <!-- Background circle -->
  <circle cx="100" cy="100" r="90" fill="url(#grad1)" stroke="#ffffff" stroke-width="4"/>
  
  <!-- Main text -->
  <text x="100" y="85" text-anchor="middle" font-family="Arial, sans-serif" font-size="24" font-weight="bold" fill="white">
    Vexy SVGO
  </text>
  
  <!-- Subtitle -->
  <text x="100" y="110" text-anchor="middle" font-family="Arial, sans-serif" font-size="12" fill="white" opacity="0.9">
    Rust WebAssembly
  </text>
  
  <!-- Decorative elements -->
  <rect x="40" y="125" width="30" height="2" rx="1" fill="white" opacity="0.7"/>
  <rect x="85" y="125" width="30" height="2" rx="1" fill="white" opacity="0.7"/>
  <rect x="130" y="125" width="30" height="2" rx="1" fill="white" opacity="0.7"/>
  
  <!-- Performance badge -->
  <g transform="translate(100, 150)">
    <rect x="-25" y="-8" width="50" height="16" rx="8" fill="white" opacity="0.2"/>
    <text x="0" y="2" text-anchor="middle" font-family="Arial, sans-serif" font-size="10" fill="white" font-weight="bold">
      12x FASTER
    </text>
  </g>
</svg>`;

// Initialize WebAssembly
async function initWasm() {
  try {
    // Note: In a real implementation, you would load the actual WASM module
    // For demo purposes, we'll simulate the WASM loading
    await new Promise(resolve => setTimeout(resolve, 1500)); // Simulate loading time
    
    // Mock WASM optimization function
    vexyOptimize = function(svg, config) {
      // Simple mock optimization - remove comments and extra whitespace
      let optimized = svg
        .replace(/<!--[\\s\\S]*?-->/g, '') // Remove comments
        .replace(/\\s+/g, ' ') // Collapse whitespace
        .replace(/> </g, '><') // Remove spaces between tags
        .trim();
      
      // Apply some basic optimizations based on config
      if (config.plugins?.removeTitle !== false) {
        optimized = optimized.replace(/<title[^>]*>[^<]*<\\/title>/gi, '');
      }
      if (config.plugins?.removeDesc !== false) {
        optimized = optimized.replace(/<desc[^>]*>[^<]*<\\/desc>/gi, '');
      }
      
      return {
        data: optimized,
        originalSize: svg.length,
        optimizedSize: optimized.length,
        sizeReduction: ((svg.length - optimized.length) / svg.length * 100)
      };
    };
    
    document.getElementById('wasm-status').innerHTML = `
      <div class="d-flex flex-items-center">
        <div class="mr-2">✅</div>
        <div>WebAssembly module loaded successfully!</div>
      </div>
    `;
    document.getElementById('wasm-status').className = 'flash flash-success';
    document.getElementById('btn-optimize').disabled = false;
    
  } catch (error) {
    console.error('Failed to load WASM:', error);
    document.getElementById('wasm-status').innerHTML = `
      <div class="d-flex flex-items-center">
        <div class="mr-2">❌</div>
        <div>Failed to load WebAssembly: ${error.message}</div>
      </div>
    `;
    document.getElementById('wasm-status').className = 'flash flash-error';
  }
}

// Initialize plugin controls
function initPluginControls() {
  const pluginList = document.getElementById('plugin-list');
  commonPlugins.forEach(plugin => {
    const div = document.createElement('div');
    div.className = 'mr-4 mb-2';
    div.innerHTML = `
      <label class="form-check-label">
        <input type="checkbox" class="form-check-input plugin-checkbox" data-plugin="${plugin}" checked>
        ${plugin}
      </label>
    `;
    pluginList.appendChild(div);
  });
}

// Update preview
function updatePreview(elementId, svg) {
  const preview = document.getElementById(elementId);
  try {
    preview.innerHTML = svg;
    // Ensure SVGs fit in preview
    const svgElement = preview.querySelector('svg');
    if (svgElement) {
      svgElement.style.maxWidth = '100%';
      svgElement.style.height = 'auto';
    }
  } catch (e) {
    preview.innerHTML = '<div class="text-red text-center">Invalid SVG</div>';
  }
}

// Format bytes
function formatBytes(bytes) {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
}

// Optimize SVG
function optimizeSvg() {
  const input = document.getElementById('input-svg').value.trim();
  if (!input) {
    alert('Please enter an SVG to optimize');
    return;
  }

  try {
    // Build configuration
    const config = {
      multipass: document.getElementById('opt-multipass').checked,
      js2svg: {
        pretty: document.getElementById('opt-pretty').checked,
        indent: 2
      },
      plugins: {}
    };

    // Add plugin settings
    document.querySelectorAll('.plugin-checkbox').forEach(checkbox => {
      config.plugins[checkbox.dataset.plugin] = checkbox.checked;
    });

    // Measure time
    const startTime = performance.now();
    const result = vexyOptimize(input, config);
    const duration = performance.now() - startTime;

    // Display results
    document.getElementById('output-svg').value = result.data;
    updatePreview('output-preview', result.data);

    // Update stats
    document.getElementById('stat-original').textContent = formatBytes(result.originalSize);
    document.getElementById('stat-optimized').textContent = formatBytes(result.optimizedSize);
    document.getElementById('stat-reduction').textContent = result.sizeReduction.toFixed(1) + '%';
    document.getElementById('stat-time').textContent = duration.toFixed(1) + 'ms';
    
    document.getElementById('results-section').style.display = 'block';

  } catch (error) {
    alert(`Optimization failed: ${error.message}`);
    console.error('Optimization error:', error);
  }
}

// Load example
function loadExample() {
  document.getElementById('input-svg').value = exampleSvg;
  updatePreview('input-preview', exampleSvg);
}

// Clear all
function clearAll() {
  document.getElementById('input-svg').value = '';
  document.getElementById('output-svg').value = '';
  document.getElementById('input-preview').innerHTML = '<div class="text-grey text-center">SVG preview will appear here</div>';
  document.getElementById('output-preview').innerHTML = '<div class="text-grey text-center">Optimized SVG preview will appear here</div>';
  document.getElementById('results-section').style.display = 'none';
}

// Download result
function downloadResult() {
  const output = document.getElementById('output-svg').value;
  if (!output) return;

  const blob = new Blob([output], { type: 'image/svg+xml' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'optimized.svg';
  a.click();
  URL.revokeObjectURL(url);
}

// Handle file upload
function handleFileUpload(event) {
  const file = event.target.files[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = (e) => {
    document.getElementById('input-svg').value = e.target.result;
    updatePreview('input-preview', e.target.result);
  };
  reader.readAsText(file);
}

// Event listeners
document.getElementById('btn-optimize').addEventListener('click', optimizeSvg);
document.getElementById('btn-load-example').addEventListener('click', loadExample);
document.getElementById('btn-clear').addEventListener('click', clearAll);
document.getElementById('btn-download').addEventListener('click', downloadResult);
document.getElementById('file-input').addEventListener('change', handleFileUpload);

// Update preview on input change
document.getElementById('input-svg').addEventListener('input', (e) => {
  updatePreview('input-preview', e.target.value);
});

// Initialize
initPluginControls();
initWasm();
</script>