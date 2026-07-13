import argparse
from pathlib import Path

import matplotlib.pyplot as plt


repo_root = Path(__file__).resolve().parents[2]
seq_file_name = repo_root / 'results' / 'sequence.txt'

parser = argparse.ArgumentParser(description='Plot the sequoia sequence.')
parser.add_argument(
    '--log',
    action='store_true',
    help='plot both axes on a logarithmic scale',
)
args = parser.parse_args()

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

print('Plotting...')

plt.scatter(indices, seq, s=3, c='blue')
plt.title('Sequoia Sequence')

if args.log:
    plt.xscale('log')
    plt.yscale('log')

plt.show()
