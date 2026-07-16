# Benchmarks

The benchmark programs generate terms from scratch. They do not read from or
write to `results/sequence.txt`.

All benchmarked versions generate the same sequence. The framework comparison
uses the same naive incremental search and varies only how previously used
branch heights are stored and checked.

The Python benchmark uses a plain `set` for branch-height membership checks.
The Rust framework benchmark compares two bitmap representations: `dense`, a
single growing `Vec<u64>`, and `chunked`, a `HashMap` whose values are `u64`
bitmap words. The algorithm comparison uses dense Rust bitmaps for both
algorithms, comparing naive incremental search with modular jumping.

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

Compile the Rust bitmap benchmark once:

```bash
rustc -O tests/framework_comparison/rust_bitmap_bench.rs -o tests/framework_comparison/rust_bitmap_bench
```

Then run both Rust bitmap methods:

```bash
tests/framework_comparison/rust_bitmap_bench 100
```

To run just one Rust method:

```bash
tests/framework_comparison/rust_bitmap_bench 100 dense
tests/framework_comparison/rust_bitmap_bench 100 chunked
```

## Algorithm Comparison

All algorithm comparison timings use dense Rust bitmaps.

| Terms | Naive incremental | Modular jumping |
|---:|---:|---:|
| 32 | 0.0002s | 0.0000s |
| 64 | 0.0031s | 0.0001s |
| 128 | 0.0458s | 0.0008s |
| 256 | 0.7996s | 0.0065s |
| 512 | 17.2664s | 0.0669s |

| Transition | Naive incremental | Modular jumping |
|---:|---:|---:|
| 32 -> 64 | 15.5x | 4.8x |
| 64 -> 128 | 14.8x | 7.5x |
| 128 -> 256 | 17.5x | 8.2x |
| 256 -> 512 | 21.6x | 10.4x |

### Algorithm Benchmark Instructions

Compile the modular jumping benchmark with:

```bash
rustc -O tests/algorithm_comparison/rust_modular_bucket_bench.rs -o tests/algorithm_comparison/rust_modular_bucket_bench
```

Then compare the naive incremental dense bitmap search with modular jumping:

```bash
tests/algorithm_comparison/rust_modular_bucket_bench 100
```

To verify that both algorithms produce the same terms:

```bash
tests/algorithm_comparison/rust_modular_bucket_bench 100 check
```
