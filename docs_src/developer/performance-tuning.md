---
nav_weight: 35
---

# Performance Tuning Guide

This guide covers advanced performance optimization techniques for Vexy SVGO, helping you achieve maximum throughput for your SVG optimization workflows.

## Table of Contents

1. [Overview](#overview)
2. [Performance Analysis](#performance-analysis)
3. [Configuration Optimization](#configuration-optimization)
4. [Parallel Processing](#parallel-processing)
5. [Memory Optimization](#memory-optimization)
6. [Batch Processing](#batch-processing)
7. [Plugin Selection](#plugin-selection)
8. [Platform Optimization](#platform-optimization)
9. [Monitoring and Profiling](#monitoring-and-profiling)

## Overview

Vexy SVGO is designed for high performance out of the box, but various configuration options and usage patterns can significantly impact performance. This guide helps you optimize for different scenarios.

### Performance Targets

| Scenario | Target Performance | Typical Hardware |
|----------|-------------------|-----------------|
| Single small SVG (<1KB) | <1ms | Modern CPU |
| Single large SVG (100KB) | <10ms | Modern CPU |
| Batch processing (1000 files) | <500ms | Modern CPU, 8 cores |
| Large SVG (>1MB) | <100ms | Modern CPU, sufficient RAM |
| Streaming (very large files) | <memory limit | Any hardware |

## Performance Analysis

### Built-in Benchmarking

```bash
# Basic performance measurement
vexy-svgo --benchmark input.svg

# Detailed timing breakdown
vexy-svgo --verbose --timing input.svg

# Memory usage tracking
vexy-svgo --memory-profile input.svg

# Compare plugin performance
vexy-svgo --plugin-timing input.svg
```

### Performance Metrics

Vexy SVGO provides detailed performance metrics:

```bash
# Sample output
Optimization completed in 12.3ms
├─ Parsing: 2.1ms (17%)
├─ Plugin execution: 8.4ms (68%)
│  ├─ removeComments: 0.8ms
│  ├─ convertPathData: 4.2ms
│  └─ cleanupIds: 3.4ms
├─ Stringification: 1.5ms (12%)
└─ Memory usage: 4.2MB peak

Files processed: 1
Total size: 45KB → 32KB (28% reduction)
Throughput: 3.65 MB/s
```

### Profiling Tools Integration

```bash
# CPU profiling (Linux/macOS)
perf record vexy-svgo large-file.svg
perf report

# Memory profiling with Valgrind
valgrind --tool=massif vexy-svgo large-file.svg

# Custom profiling (if compiled with profiling support)
vexy-svgo --profile=cpu input.svg
```

## Configuration Optimization

### Plugin Selection Strategy

**For Maximum Speed (minimal processing):**
```javascript
module.exports = {
  plugins: [
    'removeDoctype',
    'removeComments',
    'removeMetadata',
    'removeTitle',
    'removeDesc',
    'removeEmptyAttrs'
  ]
};
```

**For Balanced Performance (recommended):**
```javascript
module.exports = {
  plugins: [
    'removeDoctype',
    'removeComments', 
    'removeMetadata',
    'removeTitle',
    'removeDesc',
    'removeEmptyAttrs',
    'removeHiddenElems',
    'removeEmptyText',
    'removeEmptyContainers',
    'convertColors',
    'cleanupIds',
    'removeUselessStrokeAndFill'
  ]
};
```

**For Maximum Optimization (slower but best results):**
```javascript
module.exports = {
  multipass: true,
  plugins: [
    // All available plugins with optimized parameters
    {
      name: 'convertPathData',
      params: {
        floatPrecision: 2,
        transformPrecision: 5,
        removeUseless: true,
        collapseRepeated: true,
        utilizeAbsolute: true
      }
    },
    // ... all other plugins
  ]
};
```

### Multipass Optimization

```javascript
// Single pass (faster)
module.exports = {
  multipass: false,
  plugins: ['removeComments', 'convertColors']
};

// Multiple passes (better optimization)
module.exports = {
  multipass: true,  // Usually 2-3 passes sufficient
  plugins: ['removeComments', 'convertColors', 'cleanupIds']
};

// Limited passes (balanced)
module.exports = {
  multipass: 2,  // Explicit pass limit
  plugins: ['removeComments', 'convertColors', 'cleanupIds']
};
```

### Precision Settings

```javascript
// High precision (slower, larger files)
module.exports = {
  plugins: [
    {
      name: 'convertPathData',
      params: {
        floatPrecision: 6,
        transformPrecision: 6
      }
    },
    {
      name: 'cleanupNumericValues',
      params: {
        floatPrecision: 6
      }
    }
  ]
};

// Optimized precision (faster, smaller files)
module.exports = {
  plugins: [
    {
      name: 'convertPathData',
      params: {
        floatPrecision: 2,
        transformPrecision: 3
      }
    },
    {
      name: 'cleanupNumericValues',
      params: {
        floatPrecision: 2
      }
    }
  ]
};
```

## Parallel Processing

### Automatic Parallel Configuration

```bash
# Auto-detect optimal thread count
vexy-svgo --parallel=auto *.svg

# Use specific thread count
vexy-svgo --parallel=8 *.svg

# Disable parallel processing
vexy-svgo --parallel=1 *.svg
```

### Parallel Processing Thresholds

```javascript
// Configuration for parallel processing
module.exports = {
  parallel: {
    // Enable parallel processing for files larger than 10KB
    sizeThreshold: 10240,
    
    // Enable parallel processing when processing more than 5 elements
    elementThreshold: 5,
    
    // Use specific thread count (0 = auto-detect)
    numThreads: 0
  },
  plugins: [
    // ... your plugins
  ]
};
```

### Parallel Processing Guidelines

| File Count | File Size | Recommended Threads | Expected Speedup |
|------------|-----------|-------------------|------------------|
| 1-10 | <1KB | 1 | 1x (no benefit) |
| 1-10 | 1-100KB | 2-4 | 1.5-2x |
| 10-100 | <10KB | 4-8 | 3-6x |
| 100+ | Any | 8-16 | 6-12x |
| 1000+ | Any | 16+ | 10-20x |

```bash
# Optimal for different scenarios
vexy-svgo --parallel=1 single-large-file.svg         # Single large file
vexy-svgo --parallel=4 small-batch/*.svg             # Small batch
vexy-svgo --parallel=8 medium-batch/*.svg            # Medium batch  
vexy-svgo --parallel=16 large-batch/**/*.svg         # Large batch
```

## Memory Optimization

### Memory Limits

```bash
# Set memory limit (prevents OOM on large files)
vexy-svgo --memory-limit=1GB large-file.svg

# Enable streaming for very large files
vexy-svgo --streaming --memory-limit=512MB huge-file.svg

# Memory-efficient batch processing
vexy-svgo --memory-limit=2GB --parallel=4 --chunked=100 *.svg
```

### Memory Usage Patterns

| Operation | Memory Usage | Optimization |
|-----------|--------------|--------------|
| Parsing | 2-5x file size | Use streaming parser |
| Plugin processing | 1-3x file size | Process in chunks |
| Stringification | 1-2x file size | Stream output |
| Parallel processing | N × single thread | Limit thread count |

### Memory Configuration

```javascript
module.exports = {
  memory: {
    // Maximum memory per file (bytes)
    maxFileMemory: 100 * 1024 * 1024, // 100MB
    
    // Enable streaming for files larger than this
    streamingThreshold: 10 * 1024 * 1024, // 10MB
    
    // Chunk size for large file processing
    chunkSize: 1 * 1024 * 1024, // 1MB
    
    // Enable memory-mapped files for very large inputs
    useMemoryMapped: true
  },
  plugins: [
    // ... plugins
  ]
};
```

### Memory-Efficient Patterns

```bash
# Process large directories in chunks
find icons/ -name "*.svg" | xargs -n 50 vexy-svgo --parallel=4

# Use streaming for very large files
vexy-svgo --streaming --output-stream large-file.svg > optimized.svg

# Limit memory usage in CI/CD
vexy-svgo --memory-limit=512MB --parallel=2 *.svg
```

## Batch Processing

### Efficient Batch Operations

```bash
# Small batches (fastest startup)
vexy-svgo *.svg

# Medium batches (balanced)
vexy-svgo --parallel=4 icons/*.svg

# Large batches (maximum throughput)
vexy-svgo --parallel=8 --chunked=200 **/*.svg

# Huge batches (memory-conscious)
find . -name "*.svg" | xargs -n 100 -P 4 vexy-svgo
```

### Batch Size Optimization

| Batch Size | Processing Mode | Memory Usage | Startup Cost | Recommended For |
|------------|-----------------|--------------|--------------|-----------------|
| 1-10 files | Single-threaded | Low | High per-file | Quick tests |
| 10-100 files | Multi-threaded | Medium | Medium per-file | Development |
| 100-1000 files | Parallel batches | High | Low per-file | Production |
| 1000+ files | Chunked processing | Controlled | Very low per-file | Large deployments |

### Advanced Batch Configuration

```bash
# Progressive batch processing
vexy-svgo --batch-size=auto --parallel=auto icons/

# Memory-aware batch processing  
vexy-svgo --memory-limit=1GB --adaptive-batch icons/

# Priority-based processing (large files first)
vexy-svgo --sort-by-size --parallel=8 icons/
```

## Plugin Selection

### Performance-Oriented Plugin Sets

**Ultra-Fast (minimal optimization):**
```javascript
module.exports = {
  plugins: [
    'removeDoctype',      // 0.1ms
    'removeComments',     // 0.2ms  
    'removeMetadata',     // 0.1ms
    'removeTitle',        // 0.1ms
    'removeDesc'          // 0.1ms
  ]
  // Total: ~0.6ms per file
};
```

**Fast (good optimization/speed balance):**
```javascript
module.exports = {
  plugins: [
    'removeDoctype',
    'removeComments',
    'removeMetadata', 
    'removeTitle',
    'removeDesc',
    'removeEmptyAttrs',   // 0.3ms
    'removeHiddenElems',  // 0.4ms
    'removeEmptyText',    // 0.2ms
    'convertColors',      // 0.8ms
    'cleanupIds'          // 1.2ms
  ]
  // Total: ~3.5ms per file
};
```

**Comprehensive (best optimization):**
```javascript
module.exports = {
  multipass: true,
  plugins: [
    // All plugins enabled
    // Total: ~15-25ms per file
  ]
};
```

### Plugin Performance Characteristics

| Plugin | Speed | Size Reduction | Use Case |
|--------|-------|----------------|----------|
| `removeDoctype` | Very Fast | Small | Always enable |
| `removeComments` | Very Fast | Small-Medium | Always enable |
| `removeMetadata` | Very Fast | Small-Medium | Always enable |
| `convertColors` | Fast | Medium | Usually enable |
| `convertPathData` | Medium | Large | Enable for paths |
| `cleanupIds` | Medium | Medium | Enable if IDs present |
| `mergePaths` | Slow | Large | Enable for complex graphics |
| `convertShapeToPath` | Slow | Large | Enable for shapes |

### Smart Plugin Selection

```bash
# Analyze file to recommend plugins
vexy-svgo --analyze --recommend-plugins input.svg

# Auto-select plugins based on file content
vexy-svgo --smart-plugins input.svg

# Profile plugin impact
vexy-svgo --plugin-profile input.svg
```

## Platform Optimization

### CPU-Specific Optimizations

```bash
# Enable CPU-specific optimizations (compile-time)
cargo install vexy-svgo --features="native-cpu"

# Runtime CPU detection
vexy-svgo --cpu-optimizations=auto input.svg

# Disable CPU optimizations (for compatibility)
vexy-svgo --cpu-optimizations=none input.svg
```

### Architecture-Specific Settings

**Apple Silicon (M1/M2):**
```bash
# Optimized thread count for efficiency cores
vexy-svgo --parallel=6 --efficiency-cores=2 *.svg
```

**Intel (x86_64):**
```bash
# Utilize hyperthreading
vexy-svgo --parallel=16 --hyperthreading=true *.svg
```

**ARM64 (Server):**
```bash
# Conservative threading for thermal management
vexy-svgo --parallel=8 --thermal-throttle *.svg
```

### Memory Architecture Optimization

```bash
# NUMA-aware processing (Linux)
numactl --interleave=all vexy-svgo --parallel=16 *.svg

# Large page support (Linux)
echo madvise > /sys/kernel/mm/transparent_hugepage/enabled
vexy-svgo --large-pages *.svg
```

## Monitoring and Profiling

### Built-in Monitoring

```bash
# Real-time performance monitoring
vexy-svgo --monitor --parallel=8 *.svg

# Export performance metrics
vexy-svgo --metrics-export=json *.svg > metrics.json

# Continuous monitoring
vexy-svgo --watch --metrics icons/
```

### Performance Metrics Collection

```javascript
// metrics.json example output
{
  "totalTime": "2.345s",
  "throughput": "15.6 MB/s", 
  "filesProcessed": 1543,
  "memoryPeak": "124MB",
  "cpuUsage": "76%",
  "pluginTiming": {
    "removeComments": "234ms",
    "convertPathData": "1.2s",
    "cleanupIds": "456ms"
  },
  "parallelEfficiency": "87%"
}
```

### Integration with Monitoring Systems

**Prometheus Metrics:**
```bash
# Export Prometheus metrics
vexy-svgo --prometheus-metrics=:9090 *.svg
```

**StatsD Integration:**
```bash
# Send metrics to StatsD
vexy-svgo --statsd=localhost:8125 *.svg
```

**Custom Metrics:**
```bash
# JSON output for custom monitoring
vexy-svgo --json-metrics *.svg | jq '.throughput'
```

## Optimization Recipes

### Development Workflow

```bash
# Fast development feedback
vexy-svgo --fast --parallel=2 src/icons/

# Development with file watching
vexy-svgo --watch --fast src/icons/
```

### Production Build

```bash
# Maximum optimization for production
vexy-svgo --production --parallel=auto --multipass *.svg

# Production with size budget
vexy-svgo --production --size-budget=50% *.svg
```

### CI/CD Pipeline

```bash
# Balanced CI/CD performance
vexy-svgo --ci --parallel=4 --memory-limit=1GB *.svg

# Fast CI/CD (minimal optimization)
vexy-svgo --ci-fast --parallel=2 *.svg
```

### Large-Scale Processing

```bash
# Enterprise-scale batch processing
find assets/ -name "*.svg" | \
  xargs -n 200 -P 8 vexy-svgo \
    --parallel=4 \
    --memory-limit=512MB \
    --production
```

## Performance Troubleshooting

### Common Performance Issues

**Issue: Slow single-file processing**
```bash
# Diagnosis
vexy-svgo --verbose --timing slow-file.svg

# Solution: Check for expensive plugins
vexy-svgo --plugin-timing slow-file.svg
```

**Issue: High memory usage**
```bash
# Diagnosis  
vexy-svgo --memory-profile large-file.svg

# Solution: Enable streaming
vexy-svgo --streaming --memory-limit=256MB large-file.svg
```

**Issue: Poor parallel scaling**
```bash
# Diagnosis
vexy-svgo --parallel-analysis *.svg

# Solution: Adjust thread count and batch size
vexy-svgo --parallel=4 --batch-size=50 *.svg
```

### Performance Regression Detection

```bash
# Benchmark against reference
vexy-svgo --benchmark --reference=baseline.json *.svg

# Continuous performance testing
vexy-svgo --perf-test --ci baseline/ current/
```

## Best Practices Summary

1. **Start with defaults**: Vexy SVGO is optimized out of the box
2. **Profile before optimizing**: Use `--benchmark` and `--timing`
3. **Match optimization to use case**: Development vs. production
4. **Use parallel processing**: For batch operations (>10 files)
5. **Set memory limits**: Prevent OOM on large files
6. **Monitor performance**: Track metrics over time
7. **Test configurations**: Benchmark different plugin sets
8. **Consider file characteristics**: Size, complexity, batch size

The key to optimal performance is understanding your specific use case and choosing the right configuration parameters for your workload.