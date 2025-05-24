# Load previous terms and generate new ones.


# File with sequence terms.
f_name = 'sequence.txt'

# Number of terms to calculate up to.
num_terms = 100

# Progression of length 'n' and spacing 'n' starting at 'x'.
# Presented as a 'range' object for "C optimization".
def progression(x, n):
    return range(x, x + n**2, n)

# To store progressions of previous terms.
used_values = set()

# Load previously calculated terms and their progressions.
with open(f_name) as seq_file_read:
    for line in seq_file_read:
        # File format is "<index> <term>".
        n, x = line.strip().split()
        n, x = int(n), int(x)
        used_values.update(progression(x,n))


# Re-open file for appending.
with open(f_name, 'a') as seq_file_append:
    # Set 'n' to the last term and increment each loop.
    for n in range(n + 1, num_terms + 1):
        # Always start at x=1 to search for the next term.
        x = 1
        while True:
            # Check for collisions.
            if any(y in used_values for y in progression(x, n)):
                # Try incrementing 'x' (the greedy choice).
                x +=1
                continue
            
            # Add this progression to the used values.
            used_values.update(progression(x,n))

            # Add the first term to the sequence text file.
            seq_file_append.write(f'{n} {x}\n')

            # Flush to see updates in real time.
            seq_file_append.flush()

            # Move to the next term.
            n += 1
            break

