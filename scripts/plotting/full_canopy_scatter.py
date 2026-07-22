import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.transforms import IdentityTransform


repo_root = Path(__file__).resolve().parents[2]
seq_file_name = repo_root / 'results' / 'sequence.txt'
figure_size = (12.8, 7.2)
output_dpi = 150


def scatter_canopy(ax, indices, terms, log_scale, include_terms=True):
    """Rasterize every sequence and additional branch point as one pixel."""
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

    if include_terms:
        # Write sequence terms last so their blue pixels remain visible.
        plot_x = np.log(indices) if log_scale else np.asarray(indices)
        plot_y = np.log(terms) if log_scale else np.asarray(terms)
        pixel_x = np.rint(
            (plot_x - x_min) * (width - 1) / (x_max - x_min)
        ).astype(np.intp)
        pixel_y = np.rint(
            (plot_y - y_min) * (height - 1) / (y_max - y_min)
        ).astype(np.intp)
        visible = (
            (pixel_x >= 0)
            & (pixel_x < width)
            & (pixel_y >= 0)
            & (pixel_y < height)
        )
        canopy[pixel_y[visible], pixel_x[visible]] = (0, 0, 255, 255)

    ax.imshow(
        canopy,
        origin='lower',
        extent=(bounds.x0, bounds.x1, bounds.y0, bounds.y1),
        transform=IdentityTransform(),
        interpolation='nearest',
        aspect='auto',
        zorder=1,
    )


def save_with_blue_pixels(figure, ax, indices, terms, output):
    """Write sequence terms directly into the final PNG pixel buffer."""
    figure.canvas.draw()
    pixels = np.asarray(figure.canvas.buffer_rgba()).copy()
    display_points = ax.transData.transform(np.column_stack((indices, terms)))
    pixel_x = np.rint(display_points[:, 0]).astype(np.intp)
    # Matplotlib display coordinates start at the bottom; the image starts at top.
    pixel_y = pixels.shape[0] - 1 - np.rint(display_points[:, 1]).astype(np.intp)
    visible = (
        (pixel_x >= 0)
        & (pixel_x < pixels.shape[1])
        & (pixel_y >= 0)
        & (pixel_y < pixels.shape[0])
    )
    pixels[pixel_y[visible], pixel_x[visible]] = (0, 0, 255, 255)
    plt.imsave(output, pixels)


def main():
    parser = argparse.ArgumentParser(
        description='Plot the sequoia sequence and every branch in its full canopy.'
    )
    parser.add_argument(
        'terms',
        nargs='?',
        type=int,
        help='number of sequence terms to include; defaults to all available terms',
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
    if args.terms is not None and args.terms < 1:
        parser.error('terms must be a positive integer')

    seq = []
    indices = []

    print('Loading previous progressions...')

    with open(seq_file_name) as seq_file_read:
        for line in seq_file_read:
            # File format is "<index> <term>".
            n, x = line.strip().split()
            n, x = int(n), int(x)

            if args.terms is not None and n > args.terms:
                break

            indices.append(n)
            seq.append(x)

    if args.terms is not None and len(indices) < args.terms:
        raise ValueError(
            f'Requested {args.terms} terms, but {seq_file_name} contains only '
            f'{len(indices)}.'
        )

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

    # For saved plots, blue terms are applied directly to the final PNG below.
    scatter_canopy(ax, indices, seq, args.log, include_terms=args.output is None)
    ax.set_title(f'Sequoia Sequence Full Canopy: First {len(indices):,} Terms')

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        save_with_blue_pixels(figure, ax, indices, seq, args.output)
    else:
        plt.show()


if __name__ == '__main__':
    main()
