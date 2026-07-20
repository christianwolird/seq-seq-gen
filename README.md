# Sequoia Sequence Generator

This repository generates terms of the **sequoia sequence**.

![Direct scatter plot of the sequoia sequence](docs/images/direct_scatter.png)

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

The goals of this project are simple:

1. Generate more terms of the sequoia sequence.
2. Find patterns.

One current conjecture is that the sequence grows approximately like

```text
n^3 / log(n)
```

up to a constant factor.

## Current Method

The current approach is still a greedy algorithm, but it searches candidates by
modular bucket.

For each `n`, it first tests candidate starting heights congruent to `0 mod n`,
then `1 mod n`, then `2 mod n`, and so on. Within a bucket, it checks the
candidate's arithmetic progression from largest height to smallest. If
`x + kn` is already used, then `x`, `x + n`, ..., `x + kn` cannot be valid
starting heights in that bucket, so the search jumps directly to
`x + (k + 1)n`.

The `scripts/generate.py` entry point compiles and runs the Rust modular
dense-bitmap generator. Used branch heights are stored in a growing `Vec<u64>`
bitmap.

Run:

```bash
python3 scripts/generate.py <target number of terms>
```

The generator prints progress updates to the terminal as it finds new
terms. For example:

```text
Sequoia(2001) = 231621384  [16.1s]
Sequoia(2002) = 242082158  [19.0s]
Sequoia(2003) = 240326372  [17.0s]
```

The generated terms are also written to:

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
11 82
12 83
```

means `a(10) = 60`.

## Repository Layout

```text
.
├── scripts/
│   ├── generate.py                      Sequence generation entry point
│   │
│   └── plotting/
│       ├── README.md
│       ├── direct_scatter_plot.py       Direct scatter plot of sequence
│       ├── drop_distribution.py         Plot distribution of "dropped" terms
│       └── normalized_scatter_plot.py   Plot the sequence's "residue"
│
├── src/
│   ├── brute_generate.rs                Naive incremental generator
│   └── modular_generate.rs              Modular jumping generator
│
├── tests/
│   ├── README.md
│   │
│   ├── algorithm_comparison/
│   │   ├── modular_jumping_bench.rs     Modular jumping benchmark
│   │   └── multithreaded_modular_jumping_bench.rs Multithreaded modular benchmark
│   │
│   └── framework_comparison/            Framework and bitmap benchmarks
│       ├── python_set_bench.py          Python set benchmark
│       ├── rust_chunked_bitmap_bench.rs Chunked Rust bitmap benchmark
│       └── rust_dense_bitmap_bench.rs   Dense Rust bitmap benchmark
│
└── results/
    └── sequence.txt                     Generated sequence terms
```

## More Documentation

- [Plotting](scripts/plotting/README.md)
- [Benchmarks](tests/README.md)
