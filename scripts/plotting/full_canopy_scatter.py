import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.transforms import IdentityTransform


repo_root = Path(__file__).resolve().parents[2]
seq_file_name = repo_root / 'results' / 'sequence.txt'
figure_size = (12.8, 7.2)
output_dpi = 150


def scatter_canopy(ax, indices, terms, log_scale):
    """Rasterize every additional branch point at the output resolution."""
    # Drawing once establishes the axes' final pixel bounds after constrained layout.
    ax.figure.canvas.draw()
    bounds = ax.get_window_extent()
    width = max(1, round(bounds.width))
    height = max(1, round(bounds.height))
    canopy = np.zeros((height, width, 4), dtype=np.uint8)

    x_min, x_max = ax.get_xlim()
    y_min, y_max = ax.get_ylim()
    if log_scale:
        x_min, x_max = np.log(x_min), np.log(x_max)
        y_min, y_max = np.log(y_min), np.log(y_max)

    for n, term in zip(indices, terms):
        branch_heights = term + n * np.arange(1, n, dtype=np.float64)
        plot_x = np.log(n) if log_scale else n
        plot_y = np.log(branch_heights) if log_scale else branch_heights
        pixel_x = round((plot_x - x_min) * (width - 1) / (x_max - x_min))
        pixel_y = np.rint(
            (plot_y - y_min) * (height - 1) / (y_max - y_min)
        ).astype(np.intp)
        visible = (pixel_y >= 0) & (pixel_y < height)
        if 0 <= pixel_x < width:
            canopy[pixel_y[visible], pixel_x] = (255, 0, 0, 255)

    ax.imshow(
        canopy,
        origin='lower',
        extent=(bounds.x0, bounds.x1, bounds.y0, bounds.y1),
        transform=IdentityTransform(),
        interpolation='nearest',
        aspect='auto',
        zorder=1,
    )


def main():
    parser = argparse.ArgumentParser(
        description='Plot the sequoia sequence and every branch in its full canopy.'
    )
    parser.add_argument(
        '--log',
        action='store_true',
        help='plot both axes on a logarithmic scale',
    )
    parser.add_argument(
        '--output',
        type=Path,
        help='save the plot to this file instead of opening a window',
    )
    args = parser.parse_args()

    seq = []
    indices = []

    print('Loading previous progressions...')

    with open(seq_file_name) as seq_file_read:
        for line in seq_file_read:
            # File format is "<index> <term>".
            n, x = line.strip().split()
            n, x = int(n), int(x)

            indices.append(n)
            seq.append(x)

    print('Plotting full canopy...')

    figure, ax = plt.subplots(
        figsize=figure_size,
        dpi=output_dpi,
        constrained_layout=True,
    )
    max_canopy_height = max(x + (n - 1) * n for n, x in zip(indices, seq))
    ax.set_xlim(min(indices), max(indices))
    ax.set_ylim(min(seq), max_canopy_height)

    if args.log:
        ax.set_xscale('log')
        ax.set_yscale('log')

    scatter_canopy(ax, indices, seq, args.log)
    # Plot sequence terms last so the blue points remain visible.
    ax.scatter(indices, seq, s=3, c='blue', zorder=2)
    ax.set_title('Sequoia Sequence Full Canopy')

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        figure.savefig(args.output, dpi=output_dpi)
    else:
        plt.show()


if __name__ == '__main__':
    main()
