import os
import time
import tempfile
import random
import string
from pathlib import Path
from typing import Callable, Any
import statistics

import o3rg  # The Rust implementation
from pure_python import search_file as py_search_file, search_dir as py_search_dir

def generate_test_file(path: Path, size_mb: int, pattern_frequency: float = 0.1):
    """Generate a test file of given size with some searchable patterns."""
    words = ['test', 'example', 'benchmark', 'python', 'rust', 'performance']
    
    with open(path, 'w') as f:
        written_mb = 0
        while written_mb < size_mb:
            # Generate a line of random text
            line = ''.join(random.choices(string.ascii_letters + ' ', k=100))
            
            # Maybe insert a searchable pattern
            if random.random() < pattern_frequency:
                pos = random.randint(0, len(line) - 10)
                word = random.choice(words)
                line = line[:pos] + word + line[pos + len(word):]
            
            f.write(line + '\n')
            written_mb = os.path.getsize(path) / (1024 * 1024)

def generate_test_directory(root_dir: Path, num_files: int, size_mb_per_file: int):
    """Generate a test directory with multiple files."""
    for i in range(num_files):
        file_path = root_dir / f"test_file_{i}.txt"
        generate_test_file(file_path, size_mb_per_file)
        
        # Create some hidden files too
        if i % 5 == 0:  # 20% of files are hidden
            hidden_path = root_dir / f".hidden_file_{i}.txt"
            generate_test_file(hidden_path, size_mb_per_file)

def benchmark_function(func: Callable, *args: Any, num_runs: int = 5) -> dict:
    """Run a function multiple times and measure its performance."""
    times = []
    
    for _ in range(num_runs):
        start = time.perf_counter()
        result = func(*args)
        end = time.perf_counter()
        times.append(end - start)
    
    return {
        'mean': statistics.mean(times),
        'median': statistics.median(times),
        'std_dev': statistics.stdev(times) if len(times) > 1 else 0,
        'min': min(times),
        'max': max(times),
        'num_matches': len(result)
    }

def format_results(name: str, results: dict) -> str:
    """Format benchmark results as a string."""
    return (f"{name}:\n"
            f"  Mean: {results['mean']:.4f}s\n"
            f"  Median: {results['median']:.4f}s\n"
            f"  Std Dev: {results['std_dev']:.4f}s\n"
            f"  Min: {results['min']:.4f}s\n"
            f"  Max: {results['max']:.4f}s\n"
            f"  Matches found: {results['num_matches']}\n")

def main():
    # Test parameters
    SINGLE_FILE_SIZE_MB = 100
    DIR_NUM_FILES = 50
    DIR_SIZE_PER_FILE_MB = 10
    PATTERN = r"test|example|benchmark"
    
    print("Preparing benchmark data...")
    
    # Create a temporary directory for our tests
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        
        # Single file benchmark
        print("\nSingle file benchmark:")
        single_file = temp_path / "large_file.txt"
        print(f"Generating {SINGLE_FILE_SIZE_MB}MB test file...")
        generate_test_file(single_file, SINGLE_FILE_SIZE_MB)
        
        print("Running single file benchmarks...")
        py_results = benchmark_function(py_search_file, str(single_file), PATTERN)
        rust_results = benchmark_function(o3rg.search, str(single_file), PATTERN)
        
        print("\nSingle file search results:")
        print(format_results("Python", py_results))
        print(format_results("Rust", rust_results))
        print(f"Speedup: {py_results['mean'] / rust_results['mean']:.2f}x")
        
        # Directory benchmark
        print("\nDirectory benchmark:")
        test_dir = temp_path / "test_dir"
        test_dir.mkdir()
        print(f"Generating {DIR_NUM_FILES} files of {DIR_SIZE_PER_FILE_MB}MB each...")
        generate_test_directory(test_dir, DIR_NUM_FILES, DIR_SIZE_PER_FILE_MB)
        
        print("Running directory benchmarks...")
        py_results = benchmark_function(py_search_dir, str(test_dir), PATTERN, False)
        rust_results = benchmark_function(o3rg.search_dir, str(test_dir), PATTERN, False)
        
        print("\nDirectory search results:")
        print(format_results("Python", py_results))
        print(format_results("Rust", rust_results))
        print(f"Speedup: {py_results['mean'] / rust_results['mean']:.2f}x")

if __name__ == "__main__":
    main() 
