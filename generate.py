# Load previously calculated terms.
with open("sequence.txt") as seq_file:
    seq = [int(line.strip()) for line in seq_file]

# Load all arithmetic progressions into a set.
used_values = set()
for n, a in enumerate(seq, start=1):
    used_values.update(range(a, a + n**2, n))

n = len(seq) + 1
print(n, used_values)
