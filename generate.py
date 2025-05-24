# Load previous terms and generate new ones.


# Number of terms to calculate up to.
num_terms = 100

# Progression of length 'n' and spacing 'n' starting at 'x'.
# Presented as a 'range' object for "C optimization".
def progression(x, n):
    return range(x, x + n**2, n)

# Load previously calculated terms.
# Argument 'a+' specified append+(read) permission.
with open("sequence.txt", 'a+') as seq_file:
    # Appending usually moves a pointer to the end of the file.
    # Move it back to the start for reading.
    seq_file.seek(0)
    seq = [int(line.strip()) for line in seq_file]

    # To store progression values.
    used_values = set()

    # Load all progressions.
    for n, x in enumerate(seq, start=1):
        used_values.update(progression(x,n))

    # Set 'n' to the last term and increment each loop.
    for n in range(len(seq), num_terms):
        # Move to the next term.
        n += 1

        # Start search at x=0.
        x = 0
        while True:
            # But immediately increment to x=1.
            x +=1

            # Check for collisions.
            if any(y in used_values for y in progression(x, n)):
                continue
            
            # Add this progression to the used values.
            used_values.update(progression(x,n))

            # Add the first term to the sequence text file.
            seq_file.write(f'{x}\n')
            break

