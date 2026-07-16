# Benchmarks

The benchmark programs generate terms from scratch. They do not read from or
write to `results/sequence.txt`.

All benchmarked versions use the same greedy search. The difference is only how
they store and check previously used branch heights.

## Timing Results

Single-run benchmark timings on the author's desktop PC:

| Terms | Python set | Rust chunked bitmap | Rust dense bitmap | Rust dense modular buckets |
|---:|---:|---:|---:|---:|
| 32 | 0.0056s | 0.0009s | 0.0002s | 0.0001s |
| 64 | 0.0876s | 0.0146s | 0.0031s | 0.0003s |
| 128 | 1.8557s | 0.2315s | 0.0458s | 0.0020s |
| 256 | 40.6220s | 5.8836s | 0.7996s | 0.0174s |
| 512 | 1087.6581s | 173.7632s | 17.2664s | 0.2395s |

The scaling table shows how much slower each method became when the requested
term count doubled.

| Transition | Python set | Rust chunked bitmap | Rust dense bitmap | Rust dense modular buckets |
|---:|---:|---:|---:|---:|
| 32 -> 64 | 15.6x | 16.2x | 15.5x | 3.0x |
| 64 -> 128 | 21.2x | 15.9x | 14.8x | 6.7x |
| 128 -> 256 | 21.9x | 25.4x | 17.5x | 8.7x |
| 256 -> 512 | 26.8x | 29.5x | 21.6x | 13.8x |

## Implementations

The Python benchmark uses a plain `set` for branch-height membership checks.
This is simple and useful as a correctness-oriented baseline, but it becomes
slow quickly as the number of generated terms grows.

The Rust benchmark compares two bitmap representations:

- `dense`: one growing `Vec<u64>`
- `chunked`: a `HashMap` whose values are `u64` bitmap words

The current generator uses the dense Rust bitmap approach because it is the
fastest of these measured implementations for the tested term counts.

## Running Benchmarks

Run the Python set benchmark with:

```bash
python3 tests/python_set_bench.py 100
```

Compile the Rust bitmap benchmark once:

```bash
rustc -O tests/rust/rust_bitmap_bench.rs -o tests/rust/rust_bitmap_bench
```

Then run both Rust bitmap methods:

```bash
tests/rust/rust_bitmap_bench 100
```

To run just one Rust method:

```bash
tests/rust/rust_bitmap_bench 100 dense
tests/rust/rust_bitmap_bench 100 chunked
```

Compile the modular bucket benchmark with:

```bash
rustc -O tests/rust/rust_modular_bucket_bench.rs -o tests/rust/rust_modular_bucket_bench
```

Then compare the original dense brute-force search with the modular bucket
search:

```bash
tests/rust/rust_modular_bucket_bench 100
```

To verify that both algorithms produce the same terms:

```bash
tests/rust/rust_modular_bucket_bench 100 check
```
