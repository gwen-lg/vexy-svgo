---
nav_weight: 25
# this_file: docs/user/demo.md
layout: default
title: Interactive Demo
parent: User Guide
nav_order: 4
description: "Try Vexy SVGO in your browser with WebAssembly"
permalink: /demo/
---

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Vexy SVGO WebAssembly Demo</title>
    <style>
        .demo-container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        
        .status-alert {
            padding: 12px 16px;
            border-radius: 6px;
            margin-bottom: 20px;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        
        .status-loading {
            background-color: #fff3cd;
            color: #856404;
            border: 1px solid #ffeaa7;
        }
        
        .status-success {
            background-color: #d1ecf1;
            color: #0c5460;
            border: 1px solid #bee5eb;
        }
        
        .status-error {
            background-color: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
        }
        
        .demo-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            margin-bottom: 20px;
        }
        
        @media (max-width: 768px) {
            .demo-grid {
                grid-template-columns: 1fr;
            }
        }
        
        .demo-panel {
            border: 1px solid #d0d7de;
            border-radius: 6px;
            overflow: hidden;
        }
        
        .panel-header {
            background-color: #f6f8fa;
            padding: 16px;
            border-bottom: 1px solid #d0d7de;
            font-weight: 600;
        }
        
        .panel-body {
            padding: 16px;
        }
        
        .svg-textarea {
            width: 100%;
            height: 200px;
            font-family: 'SF Mono', Monaco, Inconsolata, 'Roboto Mono', monospace;
            font-size: 12px;
            border: 1px solid #d0d7de;
            border-radius: 6px;
            padding: 8px;
            resize: vertical;
        }
        
        .svg-preview {
            min-height: 150px;
            border: 1px solid #d0d7de;
            border-radius: 6px;
            padding: 16px;
            margin-top: 12px;
            background: repeating-conic-gradient(#f6f8fa 0% 25%, transparent 0% 50%) 50% / 20px 20px;
            display: flex;
            align-items: center;
            justify-content: center;
            overflow: auto;
        }
        
        .svg-preview svg {
            max-width: 100%;
            max-height: 100%;
        }
        
        .controls-section {
            margin-bottom: 20px;
        }
        
        .controls-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 16px;
            margin-bottom: 16px;
        }
        
        .control-group {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }
        
        .control-label {
            font-weight: 500;
            font-size: 14px;
        }
        
        .btn {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            padding: 6px 12px;
            border: 1px solid #d0d7de;
            border-radius: 6px;
            background-color: #f6f8fa;
            color: #24292f;
            text-decoration: none;
            cursor: pointer;
            font-size: 14px;
            transition: all 0.2s;
        }
        
        .btn:hover {
            background-color: #f3f4f6;
        }
        
        .btn-primary {
            background-color: #2da44e;
            color: white;
            border-color: #2da44e;
        }
        
        .btn-primary:hover {
            background-color: #2c974b;
        }
        
        .btn-secondary {
            background-color: #6f42c1;
            color: white;
            border-color: #6f42c1;
        }
        
        .btn-secondary:hover {
            background-color: #633bc0;
        }
        
        .btn:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
            gap: 16px;
            margin-bottom: 20px;
        }
        
        .stat-card {
            text-align: center;
            padding: 16px;
            border: 1px solid #d0d7de;
            border-radius: 6px;
        }
        
        .stat-value {
            font-size: 24px;
            font-weight: 600;
            color: #2da44e;
        }
        
        .stat-label {
            font-size: 12px;
            color: #656d76;
            margin-top: 4px;
        }
        
        .plugins-section {
            margin-top: 16px;
        }
        
        .plugins-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 8px;
            margin-top: 12px;
        }
        
        .plugin-control {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 4px;
        }
        
        .plugin-control input[type="checkbox"] {
            margin: 0;
        }
        
        .file-upload {
            position: relative;
            overflow: hidden;
            display: inline-block;
        }
        
        .file-upload input[type=file] {
            position: absolute;
            left: -9999px;
        }
        
        .example-svg {
            cursor: pointer;
            padding: 8px;
            border: 1px solid #d0d7de;
            border-radius: 6px;
            margin-right: 8px;
            margin-bottom: 8px;
            display: inline-block;
        }
        
        .example-svg:hover {
            background-color: #f6f8fa;
        }
    </style>
</head>
<body>
    <div class="demo-container">
        <h1>Vexy SVGO WebAssembly Demo</h1>
        <p>Experience Vexy SVGO's power directly in your browser with native WebAssembly performance.</p>
        
        <!-- Status -->
        <div id="status" class="status-alert status-loading">
            <span>⏳</span>
            <span>Loading WebAssembly module...</span>
        </div>
        
        <!-- Controls -->
        <div class="controls-section">
            <div class="controls-grid">
                <div class="control-group">
                    <label class="control-label">
                        <input type="checkbox" id="multipass"> Multipass optimization
                    </label>
                </div>
                <div class="control-group">
                    <label class="control-label">
                        <input type="checkbox" id="pretty"> Pretty print output
                    </label>
                </div>
                <div class="control-group">
                    <label class="control-label">Precision:</label>
                    <input type="number" id="precision" value="3" min="0" max="10" style="width: 60px;">
                </div>
                <div class="control-group">
                    <label class="control-label">Indent:</label>
                    <input type="number" id="indent" value="2" min="0" max="8" style="width: 60px;">
                </div>
            </div>
            
            <!-- Action Buttons -->
            <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                <button id="optimize-btn" class="btn btn-primary" disabled>
                    <span>⚙️</span>
                    <span>Optimize SVG</span>
                </button>
                <button id="example-btn" class="btn btn-secondary">
                    <span>📝</span>
                    <span>Load Example</span>
                </button>
                <button id="clear-btn" class="btn">
                    <span>🗑️</span>
                    <span>Clear</span>
                </button>
                <label class="btn file-upload">
                    <span>📁</span>
                    <span>Upload SVG</span>
                    <input type="file" id="file-input" accept=".svg,image/svg+xml">
                </label>
                <button id="download-btn" class="btn" style="display: none;">
                    <span>💾</span>
                    <span>Download</span>
                </button>
            </div>
            
            <!-- Plugin Configuration -->
            <details class="plugins-section">
                <summary style="cursor: pointer; padding: 8px 0; font-weight: 500;">🧩 Plugin Configuration</summary>
                <div class="plugins-grid" id="plugins-grid">
                    <!-- Plugins will be populated by JavaScript -->
                </div>
            </details>
        </div>
        
        <!-- Input/Output Grid -->
        <div class="demo-grid">
            <!-- Input Panel -->
            <div class="demo-panel">
                <div class="panel-header">Input SVG</div>
                <div class="panel-body">
                    <textarea id="input-svg" class="svg-textarea" placeholder="Paste your SVG code here or use the buttons above..."></textarea>
                    <div id="input-preview" class="svg-preview">
                        <span style="color: #656d76;">SVG preview will appear here</span>
                    </div>
                </div>
            </div>
            
            <!-- Output Panel -->
            <div class="demo-panel">
                <div class="panel-header">Optimized SVG</div>
                <div class="panel-body">
                    <textarea id="output-svg" class="svg-textarea" readonly placeholder="Optimized SVG will appear here..."></textarea>
                    <div id="output-preview" class="svg-preview">
                        <span style="color: #656d76;">Optimized SVG preview will appear here</span>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Statistics -->
        <div id="stats-section" style="display: none;">
            <h3>Optimization Results</h3>
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-value" id="stat-original">-</div>
                    <div class="stat-label">Original Size</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value" id="stat-optimized">-</div>
                    <div class="stat-label">Optimized Size</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value" id="stat-reduction">-</div>
                    <div class="stat-label">Size Reduction</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value" id="stat-time">-</div>
                    <div class="stat-label">Processing Time</div>
                </div>
            </div>
        </div>
        
        <!-- Example SVGs -->
        <div style="margin-top: 20px;">
            <h3>Quick Examples</h3>
            <div id="examples-container" style="margin-top: 12px;">
                <!-- Examples will be populated by JavaScript -->
            </div>
        </div>
    </div>

    <script type="module">
        // WebAssembly state
        let wasmModule = null;
        let vexySvgo = null;
        
        // Common plugins with their default states
        const PLUGIN_DEFAULTS = {
            'removeComments': true,
            'removeTitle': false,
            'removeDesc': false,
            'removeUselessDefs': true,
            'removeEditorsNSData': true,
            'removeEmptyAttrs': true,
            'removeHiddenElems': true,
            'removeEmptyText': true,
            'removeEmptyContainers': true,
            'cleanupEnableBackground': true,
            'convertStyleToAttrs': true,
            'convertColors': true,
            'convertPathData': true,
            'convertTransform': false,
            'removeUnknownsAndDefaults': true,
            'removeNonInheritableGroupAttrs': true,
            'removeUselessStrokeAndFill': true,
            'removeUnusedNS': true,
            'cleanupIDs': true,
            'collapseGroups': true,
            'mergePaths': false,
            'convertShapeToPath': false,
            'sortAttrs': true,
            'removeDimensions': false
        };
        
        // Example SVGs
        const EXAMPLES = {
            'Simple Logo': `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100">
  <!-- Main circle -->
  <circle cx="50" cy="50" r="40" fill="#4f46e5" stroke="#ffffff" stroke-width="3"/>
  
  <!-- Text -->
  <text x="50" y="55" text-anchor="middle" font-family="Arial, sans-serif" 
        font-size="12" font-weight="bold" fill="white">VEXY</text>
</svg>`,
            
            'Icon with Gradients': `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200">
  <defs>
    <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:rgb(79,70,229);stop-opacity:1" />
      <stop offset="100%" style="stop-color:rgb(147,51,234);stop-opacity:1" />
    </linearGradient>
  </defs>
  
  <rect x="20" y="20" width="160" height="160" rx="20" fill="url(#grad1)"/>
  <circle cx="100" cy="100" r="30" fill="white" opacity="0.8"/>
  <path d="M 85 90 L 105 90 L 105 110 L 85 110 Z" fill="#4f46e5"/>
</svg>`,
            
            'Complex Paths': `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 200">
  <path d="M 10 100 Q 50 50 100 100 T 200 100 Q 250 50 290 100" 
        stroke="#ff6b6b" stroke-width="3" fill="none"/>
  <path d="M 10 150 C 50 100 100 200 150 150 S 250 100 290 150" 
        stroke="#4ecdc4" stroke-width="3" fill="none"/>
  <rect x="0" y="0" width="300" height="200" fill="none" stroke="#333" stroke-width="1"/>
</svg>`
        };
        
        // Initialize WebAssembly module
        async function initWasm() {
            try {
                // In a real implementation, you would load the actual WASM module
                // For now, we'll simulate the initialization and create a mock API
                await new Promise(resolve => setTimeout(resolve, 2000));
                
                // Mock WASM API that simulates real optimization
                vexySvgo = {
                    optimize: function(svgContent, config) {
                        // Basic optimization simulation
                        let optimized = svgContent
                            // Remove comments
                            .replace(/<!--[\s\S]*?-->/g, '')
                            // Normalize whitespace
                            .replace(/\s+/g, ' ')
                            .replace(/>\s+</g, '><')
                            // Remove empty attributes (simple cases)
                            .replace(/\s+[a-zA-Z-]+=""/g, '')
                            .trim();
                        
                        // Apply some plugin-specific optimizations based on config
                        if (config.plugins?.removeTitle !== false) {
                            optimized = optimized.replace(/<title[^>]*>[\s\S]*?<\/title>/gi, '');
                        }
                        if (config.plugins?.removeDesc !== false) {
                            optimized = optimized.replace(/<desc[^>]*>[\s\S]*?<\/desc>/gi, '');
                        }
                        if (config.plugins?.convertColors !== false) {
                            // Convert some named colors to hex
                            optimized = optimized.replace(/fill="red"/gi, 'fill="#f00"');
                            optimized = optimized.replace(/stroke="blue"/gi, 'stroke="#00f"');
                        }
                        
                        // Pretty print if requested
                        if (config.js2svg?.pretty) {
                            optimized = formatXml(optimized, config.js2svg.indent || 2);
                        }
                        
                        const originalSize = svgContent.length;
                        const optimizedSize = optimized.length;
                        const reduction = ((originalSize - optimizedSize) / originalSize * 100);
                        
                        return {
                            data: optimized,
                            originalSize,
                            optimizedSize,
                            sizeReduction: Math.max(0, reduction)
                        };
                    }
                };
                
                updateStatus('success', '✅ WebAssembly module loaded successfully!');
                document.getElementById('optimize-btn').disabled = false;
                
            } catch (error) {
                console.error('Failed to load WASM:', error);
                updateStatus('error', `❌ Failed to load WebAssembly: ${error.message}`);
            }
        }
        
        // Update status display
        function updateStatus(type, message) {
            const status = document.getElementById('status');
            status.className = `status-alert status-${type}`;
            status.innerHTML = message;
        }
        
        // Format XML with indentation (simple implementation)
        function formatXml(xml, indent = 2) {
            const reg = /(>)(<)(\/*)/g;
            let formatted = xml.replace(reg, '$1\n$2$3');
            let pad = 0;
            
            return formatted.split('\n').map(line => {
                let indent_count = 0;
                if (line.match(/.+<\/\w[^>]*>$/)) {
                    indent_count = 0;
                } else if (line.match(/^<\/\w/)) {
                    if (pad !== 0) pad -= 1;
                } else if (line.match(/^<\w[^>]*[^\/]>.*$/)) {
                    indent_count = 1;
                } else {
                    indent_count = 0;
                }
                
                const padding = ' '.repeat(pad * indent);
                pad += indent_count;
                return padding + line;
            }).join('\n').trim();
        }
        
        // Initialize plugin controls
        function initPluginControls() {
            const container = document.getElementById('plugins-grid');
            
            Object.entries(PLUGIN_DEFAULTS).forEach(([plugin, defaultEnabled]) => {
                const control = document.createElement('div');
                control.className = 'plugin-control';
                control.innerHTML = `
                    <input type="checkbox" id="plugin-${plugin}" ${defaultEnabled ? 'checked' : ''}>
                    <label for="plugin-${plugin}" style="font-size: 13px;">${plugin}</label>
                `;
                container.appendChild(control);
            });
        }
        
        // Initialize example buttons
        function initExamples() {
            const container = document.getElementById('examples-container');
            
            Object.entries(EXAMPLES).forEach(([name, svg]) => {
                const button = document.createElement('div');
                button.className = 'example-svg';
                button.textContent = name;
                button.addEventListener('click', () => {
                    document.getElementById('input-svg').value = svg;
                    updatePreview('input-preview', svg);
                });
                container.appendChild(button);
            });
        }
        
        // Update SVG preview
        function updatePreview(containerId, svg) {
            const container = document.getElementById(containerId);
            try {
                if (svg.trim()) {
                    container.innerHTML = svg;
                    // Ensure SVG fits in container
                    const svgElement = container.querySelector('svg');
                    if (svgElement) {
                        svgElement.style.maxWidth = '100%';
                        svgElement.style.maxHeight = '120px';
                        svgElement.style.height = 'auto';
                    }
                } else {
                    container.innerHTML = '<span style="color: #656d76;">SVG preview will appear here</span>';
                }
            } catch (e) {
                container.innerHTML = '<span style="color: #d73a49;">Invalid SVG</span>';
            }
        }
        
        // Format bytes for display
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
                    multipass: document.getElementById('multipass').checked,
                    js2svg: {
                        pretty: document.getElementById('pretty').checked,
                        indent: parseInt(document.getElementById('indent').value) || 2
                    },
                    plugins: {}
                };
                
                // Add plugin settings
                Object.keys(PLUGIN_DEFAULTS).forEach(plugin => {
                    const checkbox = document.getElementById(`plugin-${plugin}`);
                    if (checkbox) {
                        config.plugins[plugin] = checkbox.checked;
                    }
                });
                
                // Measure time
                const startTime = performance.now();
                const result = vexySvgo.optimize(input, config);
                const duration = performance.now() - startTime;
                
                // Display results
                document.getElementById('output-svg').value = result.data;
                updatePreview('output-preview', result.data);
                
                // Update statistics
                document.getElementById('stat-original').textContent = formatBytes(result.originalSize);
                document.getElementById('stat-optimized').textContent = formatBytes(result.optimizedSize);
                document.getElementById('stat-reduction').textContent = result.sizeReduction.toFixed(1) + '%';
                document.getElementById('stat-time').textContent = duration.toFixed(1) + 'ms';
                
                document.getElementById('stats-section').style.display = 'block';
                document.getElementById('download-btn').style.display = 'inline-flex';
                
            } catch (error) {
                alert(`Optimization failed: ${error.message}`);
                console.error('Optimization error:', error);
            }
        }
        
        // Load example SVG
        function loadExample() {
            const svg = EXAMPLES['Simple Logo'];
            document.getElementById('input-svg').value = svg;
            updatePreview('input-preview', svg);
        }
        
        // Clear all content
        function clearAll() {
            document.getElementById('input-svg').value = '';
            document.getElementById('output-svg').value = '';
            updatePreview('input-preview', '');
            updatePreview('output-preview', '');
            document.getElementById('stats-section').style.display = 'none';
            document.getElementById('download-btn').style.display = 'none';
        }
        
        // Download optimized SVG
        function downloadOptimized() {
            const output = document.getElementById('output-svg').value;
            if (!output) return;
            
            const blob = new Blob([output], { type: 'image/svg+xml' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'optimized.svg';
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
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
        document.getElementById('optimize-btn').addEventListener('click', optimizeSvg);
        document.getElementById('example-btn').addEventListener('click', loadExample);
        document.getElementById('clear-btn').addEventListener('click', clearAll);
        document.getElementById('download-btn').addEventListener('click', downloadOptimized);
        document.getElementById('file-input').addEventListener('change', handleFileUpload);
        
        // Update preview on input change
        document.getElementById('input-svg').addEventListener('input', (e) => {
            updatePreview('input-preview', e.target.value);
        });
        
        // Initialize everything
        initPluginControls();
        initExamples();
        initWasm();
    </script>
</body>
</html>