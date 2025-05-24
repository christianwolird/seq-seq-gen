import sys



# ----------------------------
#      Argument Parsing
# ----------------------------

if len(sys.argv) < 2:
    print('Usage: python generate.py <target amount of terms>')
    sys.exit(1)

num_terms = int(sys.argv[1])



# ----------------------------
#     Reload Progressions
# ----------------------------

print('Loading previous progressions...')

seq_file_name = 'sequence.txt'
used_values = set()
n = 0

# Load previously calculated terms and their progressions.
with open(seq_file_name) as seq_file_read:
    for line in seq_file_read:
        # File format is "<index> <term>".
        n, x = line.strip().split()
        n, x = int(n), int(x)
        used_values.update(range(x, x + n**2, n))

print('Loading complete.')



# ----------------------------
#     Generate New Terms
# ----------------------------

print(f'Generating terms {n+1} through {num_terms}...')

# Re-open file for appending.
with open(seq_file_name, 'a') as seq_file_append:
    # Set 'n' to the last known term and increment each loop.
    for n in range(n + 1, num_terms + 1):
        x = 1
        while True:
            prog = list(range(x, x + n**2, n))

            if any(y in used_values for y in prog):
                x += 1
                continue
            
            used_values.update(prog)
            seq_file_append.write(f'{n} {x}\n')
            seq_file_append.flush()

            break

print('Done.')
