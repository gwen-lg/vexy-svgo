// TypeScript definitions for Vexy SVGO WebAssembly module
// this_file: crates/wasm/vexy_svgo.d.ts

/**
 * Basic configuration for SVG optimization
 */
export interface JsConfig {
  new(): JsConfig;
  /** Whether to run multiple optimization passes */
  multipass: boolean;
  /** Whether to prettify the output with indentation */
  pretty: boolean;
  /** Number of spaces for indentation when pretty is true */
  indent: number;
  /**
   * Set the plugins configuration as a JSON string
   * @param plugins - JSON string containing plugin configuration
   */
  setPlugins(plugins: string): void;
  /**
   * Get the current plugins configuration as a JSON string
   * @returns JSON string containing current plugin configuration
   */
  getPlugins(): string;
}

/**
 * Result of an SVG optimization operation
 */
export interface JsOptimizationResult {
  /** The optimized SVG as a string */
  readonly data: string;
  /** Error message if optimization failed */
  readonly error?: string;
  /** Size of the original SVG in bytes */
  readonly originalSize: number;
  /** Size of the optimized SVG in bytes */
  readonly optimizedSize: number;
  /** Compression ratio (optimizedSize / originalSize) */
  readonly compressionRatio: number;
  /** Size reduction percentage */
  readonly sizeReduction: number;
}

export interface EnhancedConfig {
  new(): EnhancedConfig;
  fromJson(json: string): EnhancedConfig;
  toJson(): string;
  multipass: boolean;
  pretty: boolean;
  precision: number;
  setPluginEnabled(name: string, enabled: boolean): void;
  configurePlugin(name: string, params: string): void;
  setPerformanceMode(mode: 'speed' | 'compression' | 'balanced'): void;
  setErrorHandling(mode: 'strict' | 'lenient' | 'autofix'): void;
}

export interface PerformanceMetrics {
  readonly parseTimeMs: number;
  readonly optimizeTimeMs: number;
  readonly stringifyTimeMs: number;
  readonly totalTimeMs: number;
  readonly pluginsApplied: number;
  readonly optimizationPasses: number;
  readonly elementsProcessed: number;
  readonly memoryPeakKb: number;
}

export interface EnhancedResult {
  readonly data: string;
  readonly originalSize: number;
  readonly optimizedSize: number;
  readonly compressionRatio: number;
  readonly sizeReduction: number;
  readonly isSuccess: boolean;
  getErrors(): string;
  getWarnings(): string;
  getMetrics(): PerformanceMetrics;
}

export interface PluginInfo {
  readonly name: string;
  readonly description: string;
  readonly version: string;
  readonly enabled: boolean;
  readonly configurable: boolean;
}

export interface ValidationResult {
  readonly valid: boolean;
  readonly elementCount: number;
  readonly hasViewBox: boolean;
  readonly hasNamespace: boolean;
  getIssues(): string;
}

export interface MemoryInfo {
  readonly usedKb: number;
  readonly totalKb: number;
  readonly peakKb: number;
}

export interface StreamingOptimizer {
  new(config: EnhancedConfig): StreamingOptimizer;
  addChunk(chunk: string): void;
  finalize(): EnhancedResult;
  reset(): void;
  getBufferSize(): number;
}

// Basic API
/**
 * Optimize an SVG string with optional configuration
 * @param svg - The SVG string to optimize
 * @param config - Optional configuration object
 * @returns Optimization result with the optimized SVG and statistics
 */
export function optimize(svg: string, config?: JsConfig): JsOptimizationResult;

/**
 * Optimize an SVG string using default settings
 * @param svg - The SVG string to optimize
 * @returns Optimization result with the optimized SVG and statistics
 */
export function optimizeDefault(svg: string): JsOptimizationResult;

/**
 * Get the current version of VexySVGO
 * @returns Version string (e.g., "2.1.0")
 */
export function getVersion(): string;

/**
 * Get the list of available plugins as a JSON string
 * @returns JSON string containing plugin information
 */
export function getPlugins(): string;

/**
 * Get the default preset configuration
 * @returns JSON string containing default preset configuration
 */
export function getDefaultPreset(): string;

/**
 * Optimize an SVG in chunks for better memory management with large files
 * @param svg - The SVG string to optimize
 * @param config - Optional configuration object
 * @param chunkSize - Size of chunks in bytes (default: 1MB)
 * @returns Optimization result
 */
export function optimizeChunked(svg: string, config?: JsConfig, chunkSize?: number): JsOptimizationResult;

/**
 * Free up memory used by WASM module (useful for long-running applications)
 */
export function freeMemory(): void;

// Enhanced API
export function optimizeEnhanced(svg: string, config: EnhancedConfig): EnhancedResult;
export function getAllPlugins(): PluginInfo[];
export function enableFeature(featureName: string): void;
export function isFeatureEnabled(featureName: string): boolean;
export function getAvailableFeatures(): string[];
export function validateSvg(svg: string): ValidationResult;
export function getMemoryUsage(): MemoryInfo;

// Type definitions for common usage patterns
export interface PluginConfig {
  [pluginName: string]: boolean | {
    [param: string]: any;
  };
}

export interface VexySvgoConfig {
  multipass?: boolean;
  pretty?: boolean;
  precision?: number;
  plugins?: PluginConfig;
  performanceMode?: 'speed' | 'compression' | 'balanced';
  errorHandling?: 'strict' | 'lenient' | 'autofix';
}

// Utility types
export type OptimizationOptions = {
  config?: VexySVGOConfig;
  streaming?: boolean;
  validate?: boolean;
};

export type OptimizationStats = {
  originalSize: number;
  optimizedSize: number;
  compressionRatio: number;
  sizeReduction: number;
  timeMs: number;
  pluginsApplied: number;
};

// Helper functions that can be implemented in TypeScript wrapper
export interface VexySVGOWrapper {
  optimize(svg: string, options?: OptimizationOptions): Promise<{
    data: string;
    stats: OptimizationStats;
    errors?: string[];
    warnings?: string[];
  }>;

  optimizeFile(file: File, options?: OptimizationOptions): Promise<{
    data: string;
    stats: OptimizationStats;
    errors?: string[];
    warnings?: string[];
  }>;

  optimizeStream(stream: ReadableStream<string>, options?: OptimizationOptions): Promise<{
    data: string;
    stats: OptimizationStats;
    errors?: string[];
    warnings?: string[];
  }>;

  validate(svg: string): {
    valid: boolean;
    issues: string[];
    elementCount: number;
    hasViewBox: boolean;
    hasNamespace: boolean;
  };

  getPluginInfo(): PluginInfo[];

  createConfig(options?: VexySVGOConfig): EnhancedConfig;
}

// Default export for easy usage
declare const VexySVGO: {
  // Re-export all functions
  optimize: typeof optimize;
  optimizeDefault: typeof optimizeDefault;
  optimizeEnhanced: typeof optimizeEnhanced;
  getVersion: typeof getVersion;
  getPlugins: typeof getPlugins;
  getAllPlugins: typeof getAllPlugins;
  validateSvg: typeof validateSvg;
  enableFeature: typeof enableFeature;
  isFeatureEnabled: typeof isFeatureEnabled;
  getAvailableFeatures: typeof getAvailableFeatures;
  getMemoryUsage: typeof getMemoryUsage;

  // Constructors
  JsConfig: typeof JsConfig;
  EnhancedConfig: typeof EnhancedConfig;
  StreamingOptimizer: typeof StreamingOptimizer;

  // Utility functions
  createWrapper(): VexySVGOWrapper;
};

export default VexySVGO;
