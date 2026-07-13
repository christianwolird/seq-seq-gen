import argparse
import math
from pathlib import Path

import matplotlib.pyplot as plt


repo_root = Path(__file__).resolve().parents[2]
seq_file_name = repo_root / 'results' / 'sequence.txt'
figure_size = (12.8, 7.2)
output_dpi = 150

parser = argparse.ArgumentParser(description='Plot normalized sequoia sequence residuals.')
parser.add_argument(
    'constant',
    nargs='?',
    type=float,
    default=0.228,
    help='constant c in c * n^3 / log(n); defaults to 0.228',
)
parser.add_argument(
    '--output',
    type=Path,
    help='save the plot to this file instead of opening a window',
)
args = parser.parse_args()
c = args.constant

seq = []
indices = []

print('Loading previous progressions...')

# Load previously calculated terms and their progressions.
with open(seq_file_name) as seq_file_read:
    for line in seq_file_read:
        # File format is "<index> <term>".
        n, x = line.strip().split()
        n, x = int(n), int(x)

        indices.append(n)
        seq.append(x)

plot_indices = []
plot_terms = []
plot_model_values = []
y_axis_terms = []
y_axis_model_values = []
running_max = None
min_y_axis_percentage = 99
max_y_axis_percentage = 101

for n, x in zip(indices, seq):
    previous_running_max = running_max
    running_max = x if running_max is None else max(running_max, x)

    if n <= 1:
        continue

    model_value = n**3 / math.log(n)
    plot_indices.append(n)
    plot_terms.append(x)
    plot_model_values.append(model_value)

    percent_of_previous_max = 100.0 * x / previous_running_max
    if min_y_axis_percentage <= percent_of_previous_max <= max_y_axis_percentage:
        y_axis_terms.append(x)
        y_axis_model_values.append(model_value)

if not y_axis_terms:
    raise ValueError('No terms were within 1% of the previous running maximum.')

residuals = [x - c * m for x, m in zip(plot_terms, plot_model_values)]
y_axis_residuals = [x - c * m for x, m in zip(y_axis_terms, y_axis_model_values)]

print(f'Using c: {c:.12g}')
print(f'Y-axis points: {len(y_axis_terms)} of {len(plot_terms)}')
print('Plotting...')

plt.figure(figsize=figure_size, constrained_layout=True)
plt.scatter(plot_indices, residuals, s=3, c='blue')
plt.axhline(0, color='black', linewidth=0.8)
plt.ylim(min(y_axis_residuals), max(y_axis_residuals))
plt.title(f'Sequoia Sequence Residuals: a(n) - {c:.6g} n^3 / log(n)')
plt.xlabel('n')
plt.ylabel('Residual')
if args.output:
    args.output.parent.mkdir(parents=True, exist_ok=True)
    plt.savefig(args.output, dpi=output_dpi)
else:
    plt.show()
