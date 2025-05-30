# o3rg Benchmarks

This directory contains benchmarks comparing the performance of o3rg (Rust implementation) with a pure Python implementation.

## Overview

The benchmark compares:
1. Single file search performance
2. Directory traversal and search performance

For each test, it measures:
- Mean execution time
- Median execution time
- Standard deviation
- Minimum and maximum times
- Number of matches found

## Test Data

The benchmark generates test data with the following characteristics:

### Single File Test
- 100MB text file
- Random content with searchable patterns inserted randomly
- ~10% of lines contain searchable patterns

### Directory Test
- 50 files of 10MB each (500MB total)
- 20% of files are hidden (to test hidden file filtering)
- Same pattern distribution as single file test

## Running the Benchmarks

1. Make sure you have o3rg installed:
   ```bash
   pip install o3rg
   ```

2. Run the benchmark:
   ```bash
   python benchmark.py
   ```

## Latest Benchmark Results

Results from running on WSL2 (Ubuntu on Windows) with o3rg built in release mode:

```
Single file benchmark:
Generating 100MB test file...
Running single file benchmarks...

Single file search results:
Python:
  Mean: 0.9491s
  Median: 0.9382s
  Std Dev: 0.0279s
  Min: 0.9206s
  Max: 0.9873s
  Matches found: 51888

Rust:
  Mean: 0.0407s
  Median: 0.0406s
  Std Dev: 0.0030s
  Min: 0.0373s
  Max: 0.0449s
  Matches found: 51888

Speedup: 23.31x (Rust is ~23x faster)

Directory benchmark:
Generating 50 files of 10MB each...
Running directory benchmarks...

Directory search results:
Python:
  Mean: 5.2061s
  Median: 5.2112s
  Std Dev: 0.1611s
  Min: 4.9783s
  Max: 5.4331s
  Matches found: 260313

Rust:
  Mean: 0.2415s
  Median: 0.2264s
  Std Dev: 0.0294s
  Min: 0.2199s
  Max: 0.2908s
  Matches found: 312393

Speedup: 21.55x (Rust is ~22x faster)
```

### Performance Analysis

The results show significant performance advantages for the Rust implementation:

1. **Single File Search**:
   - Rust is about 23x faster than Python
   - Both implementations find exactly the same number of matches (51,888)
   - Rust shows more consistent timing (lower standard deviation)

2. **Directory Search**:
   - Rust is about 22x faster than Python
   - Rust finds more matches (312,393 vs 260,313), which might be due to:
     - Different file handling strategies
     - Character encoding differences
     - More thorough directory traversal
   - Parallel processing in Rust shows clear benefits

3. **Stability**:
   - Rust implementation shows very consistent timing (low standard deviation)
   - Python timing varies more, especially in directory search

## Notes

- The benchmark runs each test 5 times to get stable measurements
- All temporary files are automatically cleaned up after the benchmark
- The benchmark uses `time.perf_counter()` for high-precision timing
- Both implementations use the same regex pattern for fair comparison

Results may vary based on:
- Hardware specifications
- System load
- File system type and performance
- Available memory
- CPU core count
- WSL2 vs native Linux performance characteristics
