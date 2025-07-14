// this_file: examples/nodejs-integration.js

/**
 * Node.js Integration Examples for Vexy SVGO
 * 
 * This file demonstrates various ways to integrate Vexy SVGO into Node.js applications,
 * including Express middleware, build tools, and streaming processing.
 */

const fs = require('fs').promises;
const path = require('path');
const { pipeline } = require('stream/promises');
const { Transform } = require('stream');

// Using WASM module (if available)
let vexyOptimize;
try {
    const vexyWasm = require('@vexy/svgo-wasm');
    vexyOptimize = vexyWasm.optimize;
} catch (e) {
    console.warn('WASM module not available, using native binary fallback');
}

// Fallback to native binary
const { spawn } = require('child_process');

/**
 * Basic SVG optimization using WASM module
 */
async function optimizeSvg(svgContent, options = {}) {
    if (vexyOptimize) {
        // Use WASM module (fastest)
        try {
            return vexyOptimize(svgContent, JSON.stringify(options));
        } catch (error) {
            console.error('WASM optimization failed:', error);
            // Fall back to binary
        }
    }
    
    // Fallback to native binary
    return optimizeWithBinary(svgContent, options);
}

/**
 * Optimize using native binary
 */
async function optimizeWithBinary(svgContent, options = {}) {
    return new Promise((resolve, reject) => {
        const args = ['-s', svgContent];
        
        // Add options as CLI arguments
        if (options.multipass) args.push('--multipass');
        if (options.pretty) args.push('--pretty');
        if (options.precision) args.push('--precision', options.precision.toString());
        
        const proc = spawn('vexy_svgo', args);
        
        let output = '';
        let error = '';
        
        proc.stdout.on('data', data => output += data);
        proc.stderr.on('data', data => error += data);
        
        proc.on('close', code => {
            if (code === 0) {
                resolve(output);
            } else {
                reject(new Error(`vexy_svgo failed: ${error}`));
            }
        });
    });
}

/**
 * Express.js middleware for SVG optimization
 */
function createSvgMiddleware(options = {}) {
    return async (req, res, next) => {
        // Only process SVG requests
        if (!req.path.endsWith('.svg')) {
            return next();
        }
        
        try {
            // Get original response
            const originalSend = res.send;
            
            res.send = async function(data) {
                if (typeof data === 'string' && data.includes('<svg')) {
                    // Optimize SVG content
                    const optimized = await optimizeSvg(data, options);
                    
                    // Set appropriate headers
                    res.set({
                        'Content-Type': 'image/svg+xml',
                        'Content-Length': Buffer.byteLength(optimized),
                        'X-Optimized': 'vexy-svgo'
                    });
                    
                    return originalSend.call(this, optimized);
                }
                
                return originalSend.call(this, data);
            };
            
            next();
        } catch (error) {
            console.error('SVG optimization middleware error:', error);
            next();
        }
    };
}

/**
 * Batch processing with progress tracking
 */
async function optimizeBatch(inputDir, outputDir, options = {}) {
    console.log(`Processing SVGs from ${inputDir} to ${outputDir}`);
    
    // Ensure output directory exists
    await fs.mkdir(outputDir, { recursive: true });
    
    // Find all SVG files
    const files = await findSvgFiles(inputDir);
    console.log(`Found ${files.length} SVG files`);
    
    const results = [];
    const concurrency = options.concurrency || 4;
    
    // Process files in batches
    for (let i = 0; i < files.length; i += concurrency) {
        const batch = files.slice(i, i + concurrency);
        
        const batchResults = await Promise.all(
            batch.map(async (file) => {
                try {
                    const inputPath = path.join(inputDir, file);
                    const outputPath = path.join(outputDir, file);
                    
                    const content = await fs.readFile(inputPath, 'utf8');
                    const optimized = await optimizeSvg(content, options);
                    
                    await fs.mkdir(path.dirname(outputPath), { recursive: true });
                    await fs.writeFile(outputPath, optimized);
                    
                    return {
                        file,
                        originalSize: content.length,
                        optimizedSize: optimized.length,
                        reduction: ((content.length - optimized.length) / content.length * 100).toFixed(1)
                    };
                } catch (error) {
                    return { file, error: error.message };
                }
            })
        );
        
        results.push(...batchResults);
        
        // Progress update
        const completed = Math.min(i + concurrency, files.length);
        console.log(`Processed ${completed}/${files.length} files`);
    }
    
    // Summary
    const successful = results.filter(r => !r.error);
    const failed = results.filter(r => r.error);
    
    console.log('\nOptimization Summary:');
    console.log(`✅ Successful: ${successful.length}`);
    console.log(`❌ Failed: ${failed.length}`);
    
    if (successful.length > 0) {
        const totalOriginal = successful.reduce((sum, r) => sum + r.originalSize, 0);
        const totalOptimized = successful.reduce((sum, r) => sum + r.optimizedSize, 0);
        const overallReduction = ((totalOriginal - totalOptimized) / totalOriginal * 100).toFixed(1);
        
        console.log(`📊 Overall reduction: ${overallReduction}%`);
        console.log(`💾 Space saved: ${(totalOriginal - totalOptimized / 1024).toFixed(1)}KB`);
    }
    
    return results;
}

/**
 * Streaming SVG processor for large files or real-time processing
 */
class SvgOptimizeStream extends Transform {
    constructor(options = {}) {
        super(options);
        this.chunks = [];
        this.optimizeOptions = options.optimizeOptions || {};
    }
    
    _transform(chunk, encoding, callback) {
        this.chunks.push(chunk);
        callback();
    }
    
    async _flush(callback) {
        try {
            const content = Buffer.concat(this.chunks).toString();
            
            // Only process if it looks like SVG
            if (content.trim().startsWith('<svg') || content.includes('<svg')) {
                const optimized = await optimizeSvg(content, this.optimizeOptions);
                this.push(optimized);
            } else {
                this.push(content);
            }
            
            callback();
        } catch (error) {
            callback(error);
        }
    }
}

/**
 * Watch directory for changes and auto-optimize
 */
async function watchAndOptimize(inputDir, outputDir, options = {}) {
    const chokidar = require('chokidar');
    
    console.log(`Watching ${inputDir} for SVG changes...`);
    
    const watcher = chokidar.watch(path.join(inputDir, '**/*.svg'), {
        persistent: true,
        ignoreInitial: options.ignoreInitial !== false
    });
    
    watcher.on('add', async (filePath) => {
        await processSingleFile(filePath, inputDir, outputDir, options);
    });
    
    watcher.on('change', async (filePath) => {
        await processSingleFile(filePath, inputDir, outputDir, options);
    });
    
    watcher.on('error', error => {
        console.error('Watcher error:', error);
    });
    
    return watcher;
}

/**
 * Process a single file
 */
async function processSingleFile(filePath, inputDir, outputDir, options) {
    try {
        const relativePath = path.relative(inputDir, filePath);
        const outputPath = path.join(outputDir, relativePath);
        
        console.log(`Processing: ${relativePath}`);
        
        const content = await fs.readFile(filePath, 'utf8');
        const optimized = await optimizeSvg(content, options);
        
        await fs.mkdir(path.dirname(outputPath), { recursive: true });
        await fs.writeFile(outputPath, optimized);
        
        const reduction = ((content.length - optimized.length) / content.length * 100).toFixed(1);
        console.log(`✅ ${relativePath}: ${content.length}B → ${optimized.length}B (-${reduction}%)`);
    } catch (error) {
        console.error(`❌ Error processing ${filePath}:`, error.message);
    }
}

/**
 * Find all SVG files in a directory recursively
 */
async function findSvgFiles(dir) {
    const files = [];
    
    async function scan(currentDir) {
        const entries = await fs.readdir(currentDir, { withFileTypes: true });
        
        for (const entry of entries) {
            const fullPath = path.join(currentDir, entry.name);
            
            if (entry.isDirectory()) {
                await scan(fullPath);
            } else if (entry.isFile() && entry.name.toLowerCase().endsWith('.svg')) {
                files.push(path.relative(dir, fullPath));
            }
        }
    }
    
    await scan(dir);
    return files;
}

/**
 * Build tool integration (Webpack-style)
 */
class VexySvgoWebpackPlugin {
    constructor(options = {}) {
        this.options = options;
    }
    
    apply(compiler) {
        compiler.hooks.emit.tapAsync('VexySvgoPlugin', async (compilation, callback) => {
            const svgAssets = Object.keys(compilation.assets)
                .filter(name => name.endsWith('.svg'));
            
            for (const assetName of svgAssets) {
                try {
                    const asset = compilation.assets[assetName];
                    const source = asset.source();
                    
                    const optimized = await optimizeSvg(source, this.options);
                    
                    compilation.assets[assetName] = {
                        source: () => optimized,
                        size: () => optimized.length
                    };
                } catch (error) {
                    console.error(`Failed to optimize ${assetName}:`, error);
                }
            }
            
            callback();
        });
    }
}

/**
 * CLI-style usage example
 */
async function main() {
    const args = process.argv.slice(2);
    const command = args[0];
    
    switch (command) {
        case 'optimize':
            if (args[1]) {
                const content = await fs.readFile(args[1], 'utf8');
                const result = await optimizeSvg(content);
                console.log(result);
            } else {
                console.error('Usage: node integration.js optimize <file.svg>');
            }
            break;
            
        case 'batch':
            if (args[1] && args[2]) {
                await optimizeBatch(args[1], args[2], {
                    multipass: true,
                    concurrency: 4
                });
            } else {
                console.error('Usage: node integration.js batch <input-dir> <output-dir>');
            }
            break;
            
        case 'watch':
            if (args[1] && args[2]) {
                const watcher = await watchAndOptimize(args[1], args[2]);
                console.log('Press Ctrl+C to stop watching...');
                
                process.on('SIGINT', () => {
                    console.log('\nStopping watcher...');
                    watcher.close();
                    process.exit(0);
                });
            } else {
                console.error('Usage: node integration.js watch <input-dir> <output-dir>');
            }
            break;
            
        default:
            console.log('Available commands:');
            console.log('  optimize <file.svg>        - Optimize a single file');
            console.log('  batch <input-dir> <output-dir> - Batch process directory');
            console.log('  watch <input-dir> <output-dir> - Watch and auto-optimize');
    }
}

// Export for use as module
module.exports = {
    optimizeSvg,
    createSvgMiddleware,
    optimizeBatch,
    SvgOptimizeStream,
    watchAndOptimize,
    VexySvgoWebpackPlugin
};

// Run as CLI if called directly
if (require.main === module) {
    main().catch(console.error);
}

// Example Express.js setup
/*
const express = require('express');
const app = express();

// Add SVG optimization middleware
app.use(createSvgMiddleware({
    multipass: true,
    pretty: false
}));

// Serve static files
app.use(express.static('public'));

app.listen(3000, () => {
    console.log('Server running on http://localhost:3000');
    console.log('SVG files will be automatically optimized');
});
*/