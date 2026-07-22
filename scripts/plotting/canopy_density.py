import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.ticker import FuncFormatter


repo_root = Path(__file__).resolve().parents[2]
seq_file_name = repo_root / 'results' / 'sequence.txt'
figure_size = (12.8, 7.2)
output_dpi = 150
interval_size = 1000
max_display_bars = 1800


def load_terms(limit):
    indices = []
    terms = []

    with open(seq_file_name) as seq_file_read:
        for line in seq_file_read:
            n, term = map(int, line.split())
            if n > limit:
                break
            indices.append(n)
            terms.append(term)

    if len(indices) < limit:
        raise ValueError(
            f'Requested {limit} terms, but {seq_file_name} contains only '
            f'{len(indices)}.'
        )

    return indices, terms


def count_intervals(indices, terms):
    maximum_height = max(
        term + n * (n - 1) for n, term in zip(indices, terms)
    )
    interval_count = (maximum_height + interval_size - 1) // interval_size
    counts = np.zeros(interval_count, dtype=np.uint16)

    for n, term in zip(indices, terms):
        heights = term + n * np.arange(n, dtype=np.int64)
        interval_indices = (heights - 1) // interval_size

        # Spacing of at least 1000 guarantees at most one point from this tree
        # in an interval. Smaller trees can hit an interval more than once.
        if n >= interval_size:
            counts[interval_indices] += 1
        else:
            np.add.at(counts, interval_indices, 1)

    return counts


def compress_for_display(counts):
    """Average adjacent interval bars only when the image cannot distinguish them."""
    intervals_per_bar = max(1, (len(counts) + max_display_bars - 1) // max_display_bars)
    starts = np.arange(0, len(counts), intervals_per_bar)
    totals = np.add.reduceat(counts, starts, dtype=np.uint64)
    lengths = np.minimum(intervals_per_bar, len(counts) - starts)
    percentages = 100.0 * totals / (lengths * interval_size)
    left_edges = starts.astype(np.int64) * interval_size + 1
    widths = lengths.astype(np.int64) * interval_size
    return left_edges, widths, percentages, intervals_per_bar


def main():
    parser = argparse.ArgumentParser(
        description='Plot canopy occupancy in consecutive intervals of 1000 heights.'
    )
    parser.add_argument(
        'terms',
        nargs='?',
        type=int,
        default=16000,
        help='number of sequence terms to include; defaults to 16000',
    )
    parser.add_argument(
        '--output',
        type=Path,
        help='save the plot to this file instead of opening a window',
    )
    args = parser.parse_args()
    if args.terms < 1:
        parser.error('terms must be a positive integer')

    print(f'Loading first {args.terms} terms...')
    indices, terms = load_terms(args.terms)
    print('Counting canopy points in intervals of 1000...')
    counts = count_intervals(indices, terms)
    left_edges, widths, percentages, intervals_per_bar = compress_for_display(counts)

    print(f'Intervals: {len(counts)}')
    if intervals_per_bar > 1:
        print(f'Display aggregation: {intervals_per_bar} intervals per bar')
    print('Plotting...')

    figure, ax = plt.subplots(figsize=figure_size, constrained_layout=True)
    ax.bar(
        left_edges,
        percentages,
        width=widths,
        align='edge',
        color='blue',
        linewidth=0,
    )
    ax.set_xlim(1, len(counts) * interval_size)
    ax.set_ylim(bottom=0)
    ax.yaxis.set_major_formatter(FuncFormatter(lambda y, _position: f'{y:g}%'))
    ax.set_title(f'Sequoia Canopy Density: First {args.terms} Terms')
    ax.set_xlabel('Branch height')
    ax.set_ylabel('Occupied heights per 1000-height interval')

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        figure.savefig(args.output, dpi=output_dpi)
    else:
        plt.show()


if __name__ == '__main__':
    main()
