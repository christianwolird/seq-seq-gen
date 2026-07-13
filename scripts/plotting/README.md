# Plotting Scripts

These scripts read the sequoia sequence from:

```text
results/sequence.txt
```

Each input line is expected to have the format:

```text
<index> <term>
```

Run each script from the repository root.

## Direct Scatter Plot

```bash
python3 scripts/plotting/direct_scatter_plot.py
```

Plots the sequence terms directly as `(n, a(n))`.

By default, both axes are linear. To use a log-log view:

```bash
python3 scripts/plotting/direct_scatter_plot.py --log
```

## Normalized Scatter Plot

```bash
python3 scripts/plotting/normalized_scatter_plot.py
```

Plots residuals after subtracting a conjectural growth term:

```text
a(n) - c * n^3 / log(n)
```

The default constant is:

```text
c = 0.228
```

To provide a different constant:

```bash
python3 scripts/plotting/normalized_scatter_plot.py 0.225
```

The y-axis window is set using only terms within 1% of the previous running
maximum. All residual points are still plotted, but points outside that window
are clipped by the graph bounds.

## Drop Distribution

```bash
python3 scripts/plotting/drop_distribution.py
```

For each term, this tracks the largest previous term and records:

```text
100 * a(n) / previous_running_max
```

Values below 100% are drops from the previous maximum. Values above 100% are new
records.

The plot is a histogram with:

- x-axis range from 0% to 200%
- 1% bucket width
- y-axis measured as percent of sequence terms
- logarithmic y-axis

The dashed vertical line marks 100%.
