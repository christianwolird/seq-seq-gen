# Benchmarks

The benchmark programs generate terms from scratch. They do not read from or
write to `results/sequence.txt`.

All benchmarked versions generate the same sequence. The framework comparison
uses the same naive incremental search and varies only how previously used
branch heights are stored and checked.

The Python benchmark uses a plain `set` for branch-height membership checks.
The Rust framework benchmarks cover two bitmap representations: `dense`, a
single growing `Vec<u64>`, and `chunked`, a `HashMap` whose values are `u64`
bitmap words. Each benchmark script runs one framework or algorithm only.

The current generator uses the dense Rust modular bucket approach because it is
the fastest of these measured implementations for the tested term counts.

## Framework Comparison

Single-run benchmark timings on the author's desktop PC:

| Terms | Python set | Rust bitmap (chunked) | Rust bitmap (dense) |
|---:|---:|---:|---:|
| 32 | 0.0056s | 0.0009s | 0.0002s |
| 64 | 0.0876s | 0.0146s | 0.0031s |
| 128 | 1.8557s | 0.2315s | 0.0458s |
| 256 | 40.6220s | 5.8836s | 0.7996s |
| 512 | 1087.6581s | 173.7632s | 17.2664s |

The scaling table shows how much slower each method became when the requested
term count doubled.

| Transition | Python set | Rust bitmap (chunked) | Rust bitmap (dense) |
|---:|---:|---:|---:|
| 32 -> 64 | 15.6x | 16.2x | 15.5x |
| 64 -> 128 | 21.2x | 15.9x | 14.8x |
| 128 -> 256 | 21.9x | 25.4x | 17.5x |
| 256 -> 512 | 26.8x | 29.5x | 21.6x |

### Framework Benchmark Instructions

Run the Python set benchmark with:

```bash
python3 tests/framework_comparison/python_set_bench.py 100
```

Compile the Rust bitmap benchmarks:

```bash
rustc -O tests/framework_comparison/rust_dense_bitmap_bench.rs -o tests/framework_comparison/rust_dense_bitmap_bench
rustc -O tests/framework_comparison/rust_chunked_bitmap_bench.rs -o tests/framework_comparison/rust_chunked_bitmap_bench
```

Run each Rust bitmap benchmark:

```bash
tests/framework_comparison/rust_dense_bitmap_bench 100
tests/framework_comparison/rust_chunked_bitmap_bench 100
```

## Algorithm Comparison

All algorithm comparison timings use dense Rust bitmaps.

| Terms | Naive incremental | Modular jumping | Modular + x8 Multithreading |
|---:|---:|---:|---:|
| 128 | 0.0458s | 0.0008s | 0.0393s |
| 256 | 0.7996s | 0.0065s | 0.1018s |
| 512 | 17.2664s | 0.0669s | 0.3072s |
| 1024 | 367.9468s | 0.9685s | 1.2507s |
| 2048 | N/A | 14.6916s | 7.1334s |
| 4096 | N/A | 467.9203s | 126.9103s |

| Transition | Naive incremental | Modular jumping | Modular + x8 Multithreading |
|---:|---:|---:|---:|
| 128 -> 256 | 17.5x | 8.1x | 2.6x |
| 256 -> 512 | 21.6x | 10.3x | 3.0x |
| 512 -> 1024 | 21.3x | 14.5x | 4.1x |
| 1024 -> 2048 | N/A | 15.2x | 5.7x |
| 2048 -> 4096 | N/A | 31.8x | 17.8x |

### Algorithm Benchmark Instructions

Compile the modular jumping benchmark with:

```bash
rustc -O tests/algorithm_comparison/modular_jumping_bench.rs -o tests/algorithm_comparison/modular_jumping_bench
```

Compile the multithreaded modular jumping benchmark with:

```bash
rustc -O tests/algorithm_comparison/multithreaded_modular_jumping_bench.rs -o tests/algorithm_comparison/multithreaded_modular_jumping_bench
```

Compile the clumped multithreaded benchmark, which assigns 100 modular buckets
per worker job, with:

```bash
rustc -O tests/algorithm_comparison/clumped_multithreaded_modular_jumping_bench.rs -o tests/algorithm_comparison/clumped_multithreaded_modular_jumping_bench
```

Run the modular jumping benchmark:

```bash
tests/algorithm_comparison/modular_jumping_bench 100
```

Run the multithreaded modular jumping benchmark with a fixed worker count:

```bash
tests/algorithm_comparison/multithreaded_modular_jumping_bench 100 4
```

Run the clumped multithreaded benchmark with a fixed worker count:

```bash
tests/algorithm_comparison/clumped_multithreaded_modular_jumping_bench 100 4
```
