import argparse
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.ticker import FixedLocator, FuncFormatter


repo_root = Path(__file__).resolve().parents[2]
seq_file_name = repo_root / 'results' / 'sequence.txt'
figure_size = (12.8, 7.2)
output_dpi = 150

parser = argparse.ArgumentParser(description='Plot the running-maximum drop distribution.')
parser.add_argument(
    '--output',
    type=Path,
    help='save the plot to this file instead of opening a window',
)
args = parser.parse_args()

percentages = []
running_max = None

print('Loading previous progressions...')

# Load previously calculated terms and compare each term with the maximum
# observed before that term. New records therefore plot above 100%.
with open(seq_file_name) as seq_file_read:
    for line in seq_file_read:
        # File format is "<index> <term>".
        _n, x = line.strip().split()
        x = int(x)

        if running_max is None:
            running_max = x
            percentages.append(100.0)
            continue

        percentages.append(100.0 * x / running_max)
        running_max = max(running_max, x)

print('Plotting...')

fig, ax = plt.subplots(figsize=figure_size, constrained_layout=True)

min_percentage = 0
max_percentage = 200
bin_width = 1
bin_count = int((max_percentage - min_percentage) / bin_width)
bins = [min_percentage + i * bin_width for i in range(bin_count + 1)]

ax.hist(
    percentages,
    bins=bins,
    weights=[100 / len(percentages)] * len(percentages),
    alpha=0.75,
    color='blue',
    edgecolor='black',
    linewidth=0.5,
)

ax.axvline(100, color='black', linewidth=0.8, linestyle='--')
ax.set_xlim(min_percentage, max_percentage)
ax.set_yscale('log')
ax.yaxis.set_major_locator(FixedLocator([0.1, 1, 10]))
ax.yaxis.set_major_formatter(FuncFormatter(lambda y, _pos: f'{y:g}%'))
ax.set_title('Distribution of Terms as Percent of Previous Running Maximum')
ax.set_xlabel('Term as % of previous running maximum')
ax.set_ylabel('% of terms')

if args.output:
    args.output.parent.mkdir(parents=True, exist_ok=True)
    plt.savefig(args.output, dpi=output_dpi)
else:
    plt.show()
