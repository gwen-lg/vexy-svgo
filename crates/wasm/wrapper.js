// High-level JavaScript wrapper for VEXYSVGO WebAssembly
// this_file: crates/wasm/wrapper.js

/**
 * VexySVGO WebAssembly Wrapper
 * 
 * Provides a user-friendly JavaScript API for the VexySVGO SVG optimizer.
 * This wrapper handles common use cases and provides convenient methods
 * for working with SVG optimization in web applications.
 */

let wasmModule = null;

/**
 * Initialize the VexySVGO WebAssembly module
 * @param {string|ArrayBuffer} wasmSource - Path to WASM file or ArrayBuffer
 * @returns {Promise<object>} The initialized VexySVGO module
 */
export async function initVEXYSVGO(wasmSource) {
    if (wasmModule) {
        return wasmModule;
    }

    try {
        if (typeof wasmSource === 'string') {
            // Load from URL
            const module = await import(wasmSource);
            await module.default();
            wasmModule = module;
        } else if (wasmSource instanceof ArrayBuffer) {
            // Load from ArrayBuffer
            const module = await WebAssembly.instantiate(wasmSource);
            wasmModule = module.instance.exports;
        } else {
            throw new Error('Invalid WASM source. Expected string (URL) or ArrayBuffer.');
        }

        return wasmModule;
    } catch (error) {
        throw new Error(`Failed to initialize VexySVGO WASM: ${error.message}`);
    }
}

/**
 * Check if VexySVGO is initialized
 * @returns {boolean}
 */
export function isInitialized() {
    return wasmModule !== null;
}

/**
 * Main VexySVGO class providing high-level optimization methods
 */
export class VexySVGO {
    constructor() {
        if (!isInitialized()) {
            throw new Error('VEXYSVGO must be initialized before use. Call initVEXYSVGO() first.');
        }
        this.wasmModule = wasmModule;
    }

    /**
     * Optimize an SVG string with simple options
     * @param {string} svg - SVG content to optimize
     * @param {object} options - Optimization options
     * @returns {Promise<object>} Optimization result
     */
    async optimize(svg, options = {}) {
        try {
            const config = this.createConfig(options);
            const result = this.wasmModule.optimizeEnhanced(svg, config);

            return {
                data: result.data,
                originalSize: result.originalSize,
                optimizedSize: result.optimizedSize,
                compressionRatio: result.compressionRatio,
                sizeReduction: result.sizeReduction,
                success: result.isSuccess,
                errors: JSON.parse(result.getErrors()),
                warnings: JSON.parse(result.getWarnings()),
                metrics: result.getMetrics(),
            };
        } catch (error) {
            return {
                data: svg,
                originalSize: svg.length,
                optimizedSize: svg.length,
                compressionRatio: 1.0,
                sizeReduction: 0.0,
                success: false,
                errors: [error.message],
                warnings: [],
                metrics: null,
            };
        }
    }

    /**
     * Optimize an SVG file
     * @param {File} file - SVG file to optimize
     * @param {object} options - Optimization options
     * @returns {Promise<object>} Optimization result
     */
    async optimizeFile(file, options = {}) {
        if (!file || file.type !== 'image/svg+xml') {
            throw new Error('Invalid file. Expected SVG file.');
        }

        const svg = await this.readFileAsText(file);
        return this.optimize(svg, options);
    }

    /**
     * Optimize multiple SVG files
     * @param {FileList|File[]} files - SVG files to optimize
     * @param {object} options - Optimization options
     * @returns {Promise<object[]>} Array of optimization results
     */
    async optimizeFiles(files, options = {}) {
        const results = [];
        const fileArray = Array.from(files);

        for (let i = 0; i < fileArray.length; i++) {
            const file = fileArray[i];
            try {
                const result = await this.optimizeFile(file, options);
                results.push({
                    filename: file.name,
                    ...result,
                });
            } catch (error) {
                results.push({
                    filename: file.name,
                    success: false,
                    errors: [error.message],
                    warnings: [],
                });
            }
        }

        return results;
    }

    /**
     * Optimize SVG with streaming for large files
     * @param {string} svg - SVG content
     * @param {object} options - Optimization options
     * @returns {Promise<object>} Optimization result
     */
    async optimizeStreaming(svg, options = {}) {
        try {
            const config = this.createConfig(options);
            const optimizer = new this.wasmModule.StreamingOptimizer(config);

            // Process in chunks for very large SVGs
            const chunkSize = options.chunkSize || 64 * 1024; // 64KB chunks
            for (let i = 0; i < svg.length; i += chunkSize) {
                const chunk = svg.slice(i, i + chunkSize);
                optimizer.addChunk(chunk);
            }

            const result = optimizer.finalize();

            return {
                data: result.data,
                originalSize: result.originalSize,
                optimizedSize: result.optimizedSize,
                compressionRatio: result.compressionRatio,
                sizeReduction: result.sizeReduction,
                success: result.isSuccess,
                errors: JSON.parse(result.getErrors()),
                warnings: JSON.parse(result.getWarnings()),
                metrics: result.getMetrics(),
            };
        } catch (error) {
            throw new Error(`Streaming optimization failed: ${error.message}`);
        }
    }

    /**
     * Validate SVG without optimization
     * @param {string} svg - SVG content to validate
     * @returns {object} Validation result
     */
    validate(svg) {
        try {
            const result = this.wasmModule.validateSvg(svg);
            return {
                valid: result.valid,
                elementCount: result.elementCount,
                hasViewBox: result.hasViewBox,
                hasNamespace: result.hasNamespace,
                issues: JSON.parse(result.getIssues()),
            };
        } catch (error) {
            return {
                valid: false,
                elementCount: 0,
                hasViewBox: false,
                hasNamespace: false,
                issues: [error.message],
            };
        }
    }

    /**
     * Get information about available plugins
     * @returns {object[]} Array of plugin information
     */
    getPlugins() {
        try {
            return this.wasmModule.getAllPlugins();
        } catch (error) {
            console.error('Failed to get plugins:', error);
            return [];
        }
    }

    /**
     * Create configuration object from options
     * @param {object} options - Configuration options
     * @returns {object} Enhanced configuration object
     */
    createConfig(options = {}) {
        const config = new this.wasmModule.EnhancedConfig();

        // Basic settings
        if (options.multipass !== undefined) {
            config.multipass = options.multipass;
        }
        if (options.pretty !== undefined) {
            config.pretty = options.pretty;
        }
        if (options.precision !== undefined) {
            config.precision = options.precision;
        }

        // Performance mode
        if (options.performanceMode) {
            config.setPerformanceMode(options.performanceMode);
        }

        // Error handling
        if (options.errorHandling) {
            config.setErrorHandling(options.errorHandling);
        }

        // Configure plugins
        if (options.plugins) {
            for (const [pluginName, pluginConfig] of Object.entries(options.plugins)) {
                if (typeof pluginConfig === 'boolean') {
                    config.setPluginEnabled(pluginName, pluginConfig);
                } else if (typeof pluginConfig === 'object') {
                    config.configurePlugin(pluginName, JSON.stringify(pluginConfig));
                }
            }
        }

        return config;
    }

    /**
     * Read file as text
     * @param {File} file - File to read
     * @returns {Promise<string>} File content as text
     */
    async readFileAsText(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = (event) => resolve(event.target.result);
            reader.onerror = (error) => reject(error);
            reader.readAsText(file);
        });
    }

    /**
     * Get memory usage information
     * @returns {object} Memory usage statistics
     */
    getMemoryUsage() {
        try {
            return this.wasmModule.getMemoryUsage();
        } catch (error) {
            console.error('Failed to get memory usage:', error);
            return { usedKb: 0, totalKb: 0, peakKb: 0 };
        }
    }

    /**
     * Get VexySVGO version
     * @returns {string} Version string
     */
    getVersion() {
        return this.wasmModule.getVersion();
    }

    /**
     * Enable a feature
     * @param {string} featureName - Feature to enable
     */
    enableFeature(featureName) {
        try {
            this.wasmModule.enableFeature(featureName);
        } catch (error) {
            console.error(`Failed to enable feature ${featureName}:`, error);
        }
    }

    /**
     * Check if a feature is enabled
     * @param {string} featureName - Feature to check
     * @returns {boolean} Whether the feature is enabled
     */
    isFeatureEnabled(featureName) {
        try {
            return this.wasmModule.isFeatureEnabled(featureName);
        } catch (error) {
            console.error(`Failed to check feature ${featureName}:`, error);
            return false;
        }
    }

    /**
     * Get list of available features
     * @returns {string[]} Array of available feature names
     */
    getAvailableFeatures() {
        try {
            return this.wasmModule.getAvailableFeatures();
        } catch (error) {
            console.error('Failed to get available features:', error);
            return [];
        }
    }
}

/**
 * Utility functions
 */
export const utils = {
    /**
     * Create a download link for optimized SVG
     * @param {string} svg - SVG content
     * @param {string} filename - Download filename
     * @returns {HTMLAnchorElement} Download link element
     */
    createDownloadLink(svg, filename = 'optimized.svg') {
        const blob = new Blob([svg], { type: 'image/svg+xml' });
        const url = URL.createObjectURL(blob);

        const link = document.createElement('a');
        link.href = url;
        link.download = filename;

        return link;
    },

    /**
     * Format bytes to human-readable string
     * @param {number} bytes - Number of bytes
     * @returns {string} Formatted string
     */
    formatBytes(bytes) {
        if (bytes < 1024) return `${bytes} B`;
        if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
        return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    },

    /**
     * Format optimization statistics
     * @param {object} result - Optimization result
     * @returns {string} Formatted statistics
     */
    formatStats(result) {
        return [
            `Original: ${this.formatBytes(result.originalSize)}`,
            `Optimized: ${this.formatBytes(result.optimizedSize)}`,
            `Reduction: ${result.sizeReduction.toFixed(1)}%`,
            result.metrics ? `Time: ${result.metrics.totalTimeMs.toFixed(1)}ms` : '',
        ].filter(Boolean).join(' | ');
    },

    /**
     * Validate SVG file before processing
     * @param {File} file - File to validate
     * @returns {boolean} Whether file is valid SVG
     */
    isValidSVGFile(file) {
        return file && (
            file.type === 'image/svg+xml' ||
            file.name.toLowerCase().endsWith('.svg')
        );
    },

    /**
     * Extract SVG viewBox dimensions
     * @param {string} svg - SVG content
     * @returns {object|null} ViewBox dimensions or null
     */
    extractViewBox(svg) {
        const viewBoxMatch = svg.match(/viewBox="([^"]+)"/);
        if (!viewBoxMatch) return null;

        const values = viewBoxMatch[1].split(/\s+/).map(Number);
        if (values.length !== 4) return null;

        return {
            x: values[0],
            y: values[1],
            width: values[2],
            height: values[3],
        };
    },
};

/**
 * Presets for common optimization scenarios
 */
export const presets = {
    /**
     * Web production preset - aggressive optimization
     */
    webProduction: {
        multipass: true,
        precision: 2,
        performanceMode: 'compression',
        plugins: {
            removeComments: true,
            removeEmptyAttrs: true,
            removeEmptyText: true,
            removeUnusedNS: true,
            removeEditorsNSData: true,
            removeMetadata: true,
            removeTitle: false, // Keep for accessibility
            removeDesc: false,   // Keep for accessibility
            removeUselessDefs: true,
            removeEmptyContainers: true,
            removeUnknownElem: true,
            removeUnknownAttrs: true,
            collapseGroups: true,
            convertPathData: true,
            convertTransform: true,
            convertColors: true,
            mergePaths: true,
        },
    },

    /**
     * Icon optimization preset
     */
    icons: {
        multipass: true,
        precision: 1,
        performanceMode: 'compression',
        plugins: {
            removeComments: true,
            removeEmptyAttrs: true,
            removeMetadata: true,
            removeTitle: true,
            removeDesc: true,
            removeUselessDefs: true,
            removeEmptyContainers: true,
            collapseGroups: true,
            convertPathData: true,
            convertTransform: true,
            convertColors: true,
            mergePaths: true,
            cleanupIds: { minify: true },
        },
    },

    /**
     * Preserve editability preset
     */
    editable: {
        multipass: false,
        pretty: true,
        precision: 3,
        performanceMode: 'balanced',
        plugins: {
            removeComments: true,
            removeEmptyAttrs: true,
            removeUnusedNS: true,
            removeEditorsNSData: false, // Keep for editors
            removeMetadata: false,      // Keep metadata
            convertPathData: false,     // Don't simplify paths
            mergePaths: false,          // Don't merge paths
            convertTransform: false,    // Keep original transforms
        },
    },

    /**
     * Fast processing preset
     */
    fast: {
        multipass: false,
        precision: 3,
        performanceMode: 'speed',
        plugins: {
            removeComments: true,
            removeEmptyAttrs: true,
            removeEmptyText: true,
            removeUnusedNS: true,
            removeEditorsNSData: true,
            removeMetadata: true,
        },
    },
};

// Default export
export default {
    initVEXYSVGO,
    isInitialized,
    VexySVGO,
    utils,
    presets,
};