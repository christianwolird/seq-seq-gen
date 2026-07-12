# Benchmarks

These benchmarks generate terms from scratch. They do not read or write
`results/sequence.txt`.

## Python Set

```bash
python tests/python_set_bench.py 100
```

This measures the speed of using Python set membership to check for tangles 
(branches at the same height).

## Rust Bitmaps

Alternatively, we can measure the speed of Rust bitmaps.

Compile once:

```bash
rustc -O tests/rust/rust_bitmap_bench.rs -o tests/rust/rust_bitmap_bench
```

Then run:

```bash
tests/rust/rust_bitmap_bench 100
```

That runs both Rust methods. To run just one:

```bash
tests/rust/rust_bitmap_bench 100 dense
tests/rust/rust_bitmap_bench 100 chunked
```

The Rust methods are:

- `rust dense bitmap`: one growing `Vec<u64>`;
- `rust chunked bitmap`: a `HashMap` whose values are `u64` bitmap words.
