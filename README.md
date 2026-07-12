# Sequoia Sequence Generator

This repository generates terms of the **sequoia sequence**.

The name comes from a tree analogy. Each term represents one tree, and that tree puts out branches at evenly spaced heights. The term value is the height of the tree's lowest branch. The arithmetic progression generated from that term represents all branch heights on that tree.

For tree `n`, the branch spacing is `n`, and the tree has `n` branches:

```text
x, x + n, x + 2n, ..., x + (n - 1)n
```

The sequoia sequence chooses the smallest possible `x` for each tree such that none of its branch heights collide with any branch height from an earlier tree.

Metaphorically:

- each term is a tree's lowest branch;
- each arithmetic progression is that tree's full set of branch heights;
- branches from different trees must not tangle;
- each new tree places its branches as low as possible;
- each new tree increases its branch spacing by one.

## Definition

Let `a(n)` be the sequoia sequence.

For each positive integer `n`, `a(n)` is the smallest positive integer `x` such that the set

```text
{x + kn : 0 <= k < n}
```

is disjoint from every previously used branch height.

After choosing `a(n)`, all values

```text
a(n), a(n) + n, a(n) + 2n, ..., a(n) + (n - 1)n
```

are marked as used.

The sequence begins:

```text
1, 2, 3, 7, 8, 14, 22, 31, 40, 60, ...
```

Equivalently, the first few trees are:

```text
n = 1, a(1) = 1   →  1
n = 2, a(2) = 2   →  2, 4
n = 3, a(3) = 3   →  3, 6, 9
n = 4, a(4) = 7   →  7, 11, 15, 19
n = 5, a(5) = 8   →  8, 13, 18, 23, 28
```

## Goal

The goal of this project is simple: generate more terms of the sequoia sequence.

Implementation experiments are only a means to that end. Faster collision checks, lower memory usage, and alternative data structures are useful insofar as they help generate more terms correctly.

The main reason to generate more terms is to look for interesting patterns. One current conjecture is that the sequence grows approximately like

```text
n^3 / log(n)
```

up to a constant factor.

## Current Generator

The current generator is a straightforward greedy implementation.

For each `n`, it tests candidate starting heights `x = 1, 2, 3, ...` until it finds one whose full arithmetic progression does not intersect the set of already used branch heights.

The `scripts/generate.py` entry point compiles and runs a Rust dense-bitmap generator. Used branch heights are stored in a growing `Vec<u64>` bitmap, which keeps collision checks fast while preserving the same greedy search.

Run:

```bash
python3 scripts/generate.py <target number of terms>
```

The generated terms are written to:

```text
results/sequence.txt
```

Each line has the format:

```text
<index> <term>
```

For example:

```text
10 60
```

means `a(10) = 60`.

## Repository Layout

```text
scripts/generate.py                  Main sequence generator
scripts/plots/scatter_plot.py        Scatter plot helper
scripts/plots/log_scatter_plot.py    Log scatter plot helper
tests/python_set_bench.py            Python set benchmark
tests/rust/rust_bitmap_bench.rs      Rust bitmap benchmarks
results/                             Generated sequence data
results/backups/                     Backups of previous generated data
```

## Timing Benchmarks

Single-run benchmark timings on the author's desktop PC:

All three benchmarked versions use the same greedy search; the difference is only how they store and check previously used branch heights.

| Terms | Python set | Rust dense bitmap | Rust chunked bitmap |
|---:|---:|---:|---:|
| 32 | 0.0056s | 0.0002s | 0.0009s |
| 64 | 0.0876s | 0.0031s | 0.0146s |
| 128 | 1.8557s | 0.0458s | 0.2315s |
| 256 | 40.6220s | 0.7996s | 5.8836s |
| 512 | 1087.6581s | 17.2664s | 173.7632s |

Transition scaling factors:

The scaling table shows how much slower each method became when the requested term count doubled.

| Transition | Python set | Rust dense bitmap | Rust chunked bitmap |
|---:|---:|---:|---:|
| 32 → 64 | 15.6x | 15.5x | 16.2x |
| 64 → 128 | 21.2x | 14.8x | 15.9x |
| 128 → 256 | 21.9x | 17.5x | 25.4x |
| 256 → 512 | 26.8x | 21.6x | 29.5x |
