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

Each plotting script can either open an interactive window or save directly to
an image file. To save a plot, pass `--output <path>`. Parent directories are
created automatically. Saved images use a 16:9 1920x1080 canvas.

## Direct Scatter Plot

![Direct scatter plot](../../docs/images/direct_scatter.png)

```bash
python3 scripts/plotting/direct_scatter_plot.py
```

Plots the sequence terms directly as `(n, a(n))`.

By default, both axes are linear. To use a log-log view:

![Log-log direct scatter plot](../../docs/images/direct_scatter_log.png)

```bash
python3 scripts/plotting/direct_scatter_plot.py --log
```

To save either version:

```bash
python3 scripts/plotting/direct_scatter_plot.py --output docs/images/direct_scatter.png
python3 scripts/plotting/direct_scatter_plot.py --log --output docs/images/direct_scatter_log.png
```

## Normalized Scatter Plot

![Normalized scatter plot](../../docs/images/normalized_scatter.png)

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

To save the plot:

```bash
python3 scripts/plotting/normalized_scatter_plot.py --output docs/images/normalized_scatter.png
```

The y-axis window is set using only terms within 1% of the previous running
maximum. All residual points are still plotted, but points outside that window
are clipped by the graph bounds.

## Drop Distribution

![Drop distribution histogram](../../docs/images/drop_distribution.png)

```bash
python3 scripts/plotting/drop_distribution.py
```

To save the plot:

```bash
python3 scripts/plotting/drop_distribution.py --output docs/images/drop_distribution.png
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
