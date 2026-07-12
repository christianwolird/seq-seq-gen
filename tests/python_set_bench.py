#!/usr/bin/env python3
"""Time the plain Python set generator."""

import sys
import time


def generate(num_terms):
    used_values = set()
    terms = []

    for n in range(1, num_terms + 1):
        x = 1
        while True:
            progression = range(x, x + n * n, n)

            if any(y in used_values for y in progression):
                x += 1
                continue

            used_values.update(progression)
            terms.append(x)
            break

    return terms


if len(sys.argv) != 2:
    print("Usage: python tests/python_set_bench.py <term count>")
    sys.exit(1)

num_terms = int(sys.argv[1])
if num_terms < 1:
    print("term count must be positive")
    sys.exit(1)

start = time.perf_counter()
terms = generate(num_terms)
seconds = time.perf_counter() - start

print(f"python set: {seconds:.4f}s")
print(f"last term:  {terms[-1]}")
