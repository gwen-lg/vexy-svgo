// this_file: docs/assets/js/svgo-optimizer.js

/**
 * SVGO Server-Side Optimizer JavaScript Handler
 *
 * This class demonstrates server-side SVG optimization using SVGO.
 * In a real deployment, this would send SVG files to a server endpoint
 * for processing using the original Node.js SVGO library.
 *
 * For demonstration purposes in this static site, it shows the UI
 * and simulates server-side processing behavior.
 */
class SVGOOptimizer {
    constructor() {
        this.init();
        this.apiEndpoint = '/api/optimize'; // Server endpoint for SVGO processing (demo)
        this.maxFileSize = 10 * 1024 * 1024; // 10MB limit
        this.supportedTypes = ['image/svg+xml', 'text/xml'];
        this.isDemoMode = true; // Static site demo mode
    }

    /**
     * Initialize the optimizer with event listeners and UI setup
     */
    init() {
        this.setupEventListeners();
        this.setupDragAndDrop();
        this.hideAllSections();
    }

    /**
     * Set up event listeners for user interactions
     */
    setupEventListeners() {
        // File selection
        const fileInput = document.getElementById('file-input');
        const fileSelectBtn = document.getElementById('file-select-btn');

        if (fileSelectBtn) {
            fileSelectBtn.addEventListener('click', () => fileInput?.click());
        }

        if (fileInput) {
            fileInput.addEventListener('change', (e) => this.handleFiles(e.target.files));
        }

        // Copy buttons
        document.getElementById('copy-original')?.addEventListener('click', () =>
            this.copyToClipboard('original-code'));
        document.getElementById('copy-optimized')?.addEventListener('click', () =>
            this.copyToClipboard('optimized-code'));

        // Download buttons
        document.getElementById('download-optimized')?.addEventListener('click', () =>
            this.downloadOptimized());
        document.getElementById('download-all')?.addEventListener('click', () =>
            this.downloadAll());
    }

    /**
     * Set up drag and drop functionality for the file drop zone
     */
    setupDragAndDrop() {
        const dropZone = document.getElementById('file-drop-zone');
        if (!dropZone) return;

        ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
            dropZone.addEventListener(eventName, this.preventDefaults, false);
        });

        ['dragenter', 'dragover'].forEach(eventName => {
            dropZone.addEventListener(eventName, () => this.highlight(dropZone), false);
        });

        ['dragleave', 'drop'].forEach(eventName => {
            dropZone.addEventListener(eventName, () => this.unhighlight(dropZone), false);
        });

        dropZone.addEventListener('drop', (e) => this.handleDrop(e), false);
    }

    /**
     * Prevent default drag behaviors
     */
    preventDefaults(e) {
        e.preventDefault();
        e.stopPropagation();
    }

    /**
     * Highlight drop zone during drag
     */
    highlight(element) {
        element.classList.add('drag-over');
    }

    /**
     * Remove highlight from drop zone
     */
    unhighlight(element) {
        element.classList.remove('drag-over');
    }

    /**
     * Handle file drop events
     */
    handleDrop(e) {
        const dt = e.dataTransfer;
        const files = dt.files;
        this.handleFiles(files);
    }

    /**
     * Process uploaded files
     */
    async handleFiles(files) {
        if (!files || files.length === 0) return;

        const validFiles = this.validateFiles(files);
        if (validFiles.length === 0) {
            this.showError('No valid SVG files selected. Please upload SVG files only.');
            return;
        }

        this.hideError();
        this.showProcessing();

        try {
            await this.processFiles(validFiles);
        } catch (error) {
            console.error('Error processing files:', error);
            this.showError('Failed to optimize SVG files. Please try again.');
        } finally {
            this.hideProcessing();
        }
    }

    /**
     * Validate uploaded files
     */
    validateFiles(files) {
        return Array.from(files).filter(file => {
            // Check file size
            if (file.size > this.maxFileSize) {
                console.warn(`File ${file.name} is too large (${file.size} bytes)`);
                return false;
            }

            // Check file type
            if (!this.supportedTypes.includes(file.type) && !file.name.toLowerCase().endsWith('.svg')) {
                console.warn(`File ${file.name} is not an SVG file`);
                return false;
            }

            return true;
        });
    }

    /**
     * Process files by sending them to the server for optimization
     */
    async processFiles(files) {
        const results = [];

        for (const file of files) {
            try {
                const originalContent = await this.readFileAsText(file);
                const optimizedResult = await this.optimizeViaSVGO(originalContent, file.name);

                results.push({
                    name: file.name,
                    original: originalContent,
                    optimized: optimizedResult.data,
                    originalSize: file.size,
                    optimizedSize: new Blob([optimizedResult.data]).size,
                    savings: optimizedResult.info
                });
            } catch (error) {
                console.error(`Error processing file ${file.name}:`, error);
                this.showError(`Failed to optimize ${file.name}: ${error.message}`);
                return;
            }
        }

        this.displayResults(results);
    }

    /**
     * Read file content as text
     */
    readFileAsText(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = e => resolve(e.target.result);
            reader.onerror = e => reject(new Error('Failed to read file'));
            reader.readAsText(file);
        });
    }

    /**
     * Send SVG content to server for SVGO optimization
     * In demo mode, simulates server-side processing
     */
    async optimizeViaSVGO(svgContent, fileName) {
        if (this.isDemoMode) {
            // Simulate server processing with basic optimizations for demo
            return this.simulateServerOptimization(svgContent, fileName);
        }

        // Real server-side implementation would look like this:
        const response = await fetch(this.apiEndpoint, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                input: svgContent,
                path: fileName,
                config: {
                    // Default SVGO configuration
                    plugins: [
                        'preset-default'
                    ]
                }
            })
        });

        if (!response.ok) {
            throw new Error(`Server error: ${response.status} ${response.statusText}`);
        }

        return await response.json();
    }

    /**
     * Simulate server-side SVGO optimization for demo purposes
     */
    async simulateServerOptimization(svgContent, fileName) {
        // Add artificial delay to simulate server processing
        await new Promise(resolve => setTimeout(resolve, 1500));

        // Basic optimization simulation
        let optimized = svgContent
            // Remove XML declaration if present
            .replace(/<\?xml[^>]*\?>\s*/g, '')
            // Remove comments
            .replace(/<!--[\s\S]*?-->/g, '')
            // Remove unnecessary whitespace
            .replace(/>\s+</g, '><')
            .trim();

        // Calculate approximate savings
        const originalSize = new TextEncoder().encode(svgContent).length;
        const optimizedSize = new TextEncoder().encode(optimized).length;
        const savings = originalSize - optimizedSize;
        const percentage = Math.round((savings / originalSize) * 100);

        return {
            data: optimized,
            info: {
                originalSize,
                optimizedSize,
                savings,
                percentage
            }
        };
    }

    /**
     * Display optimization results
     */
    displayResults(results) {
        if (results.length === 0) return;

        // For single file, show comparison view
        if (results.length === 1) {
            const result = results[0];
            this.displaySingleResult(result);
        } else {
            // For multiple files, show batch results
            this.displayBatchResults(results);
        }

        this.showResults();
        this.updateBatchDownload(results);
    }

    /**
     * Display single file optimization result
     */
    displaySingleResult(result) {
        // Update original preview and code
        this.updatePreview('original-preview', result.original);
        this.updateCode('original-code', result.original);
        this.updateSizeDisplay('original-size-display', result.originalSize);

        // Update optimized preview and code
        this.updatePreview('optimized-preview', result.optimized);
        this.updateCode('optimized-code', result.optimized);
        this.updateSizeDisplay('optimized-size-display', result.optimizedSize);

        // Store current result for downloads
        this.currentResult = result;
    }

    /**
     * Display batch optimization results
     */
    displayBatchResults(results) {
        // For now, just show the first result in the comparison view
        // TODO: Implement proper batch results UI
        this.displaySingleResult(results[0]);
        this.currentResults = results;
    }

    /**
     * Update SVG preview
     */
    updatePreview(containerId, svgContent) {
        const container = document.getElementById(containerId);
        if (!container) return;

        try {
            // Create a safe preview by parsing and re-serializing the SVG
            const parser = new DOMParser();
            const doc = parser.parseFromString(svgContent, 'image/svg+xml');
            const svgElement = doc.documentElement;

            if (svgElement.tagName === 'svg') {
                container.innerHTML = svgElement.outerHTML;
            } else {
                container.innerHTML = '<div class= PROTECTED_0_ >Invalid SVG content</div>';
            }
        } catch (error) {
            console.error('Error updating preview:', error);
            container.innerHTML = '<div class= PROTECTED_0_ >Preview unavailable</div>';
        }
    }

    /**
     * Update code display
     */
    updateCode(codeId, content) {
        const codeElement = document.getElementById(codeId);
        if (!codeElement) return;

        // Format the SVG content for display
        const formattedContent = this.formatSVG(content);
        codeElement.textContent = formattedContent;
    }

    /**
     * Format SVG content for display
     */
    formatSVG(content) {
        try {
            // Basic formatting - add line breaks after tags
            return content
                .replace(/></g, '>\n<')
                .replace(/^\s+|\s+$/g, '');
        } catch (error) {
            return content;
        }
    }

    /**
     * Update file size display
     */
    updateSizeDisplay(sizeId, bytes) {
        const sizeElement = document.getElementById(sizeId);
        if (!sizeElement) return;

        const kb = (bytes / 1024).toFixed(1);
        sizeElement.textContent = `${kb} KB`;
    }

    /**
     * Copy content to clipboard
     */
    async copyToClipboard(codeId) {
        const codeElement = document.getElementById(codeId);
        if (!codeElement) return;

        try {
            await navigator.clipboard.writeText(codeElement.textContent);
            this.showTempMessage('Copied to clipboard!');
        } catch (error) {
            console.error('Failed to copy to clipboard:', error);
        }
    }

    /**
     * Download optimized SVG
     */
    downloadOptimized() {
        if (!this.currentResult) return;

        const blob = new Blob([this.currentResult.optimized], { type: 'image/svg+xml' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `optimized_${this.currentResult.name}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }

    /**
     * Download all optimized files as ZIP
     */
    async downloadAll() {
        if (!this.currentResults || this.currentResults.length === 0) return;

        // Note: This requires a ZIP library like JSZip
        // For now, download files individually
        for (const result of this.currentResults) {
            const blob = new Blob([result.optimized], { type: 'image/svg+xml' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `optimized_${result.name}`;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);

            // Small delay between downloads
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    }

    /**
     * Show processing status
     */
    showProcessing() {
        document.getElementById('processing-status')?.style.setProperty('display', 'block');
    }

    /**
     * Hide processing status
     */
    hideProcessing() {
        document.getElementById('processing-status')?.style.setProperty('display', 'none');
    }

    /**
     * Show results area
     */
    showResults() {
        document.getElementById('results-area')?.style.setProperty('display', 'block');
    }

    /**
     * Update batch download button
     */
    updateBatchDownload(results) {
        const batchDownload = document.getElementById('batch-download');
        if (!batchDownload) return;

        if (results.length > 1) {
            batchDownload.style.display = 'block';
        } else {
            batchDownload.style.display = 'none';
        }
    }

    /**
     * Show error message
     */
    showError(message) {
        const errorDisplay = document.getElementById('error-display');
        const errorMessage = document.getElementById('error-message');

        if (errorDisplay && errorMessage) {
            errorMessage.textContent = message;
            errorDisplay.style.display = 'block';
        }
    }

    /**
     * Hide error message
     */
    hideError() {
        document.getElementById('error-display')?.style.setProperty('display', 'none');
    }

    /**
     * Hide all sections initially
     */
    hideAllSections() {
        this.hideProcessing();
        this.hideError();
        document.getElementById('results-area')?.style.setProperty('display', 'none');
        document.getElementById('batch-download')?.style.setProperty('display', 'none');
    }

    /**
     * Show temporary message
     */
    showTempMessage(message) {
        // Create a temporary toast message
        const toast = document.createElement('div');
        toast.className = 'alert alert-success fixed top-4 right-4 z-50 max-w-sm';
        toast.innerHTML = `
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
            </svg>
            <span>${message}</span>
        `;

        document.body.appendChild(toast);

        // Remove after 3 seconds
        setTimeout(() => {
            if (toast.parentNode) {
                toast.parentNode.removeChild(toast);
            }
        }, 3000);
    }
}

// Initialize the optimizer when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new SVGOOptimizer();
});