# Day 1080: Markov chain simulation: seeded random.Random; O(steps*states) time O(states^2) space
# Counts state arrived at after each step (not initial state); total counts = num_steps = 5000
# Exact counts vary by seed; approx distribution: ~60% a, ~33% b, ~7% c
import random

def simulate_markov(start, transitions, num_steps, seed=42):
    trans = {}
    for (frm, to, prob) in transitions:
        trans.setdefault(frm, []).append((to, prob))

    rng = random.Random(seed)
    state = start
    counts = {s: 0 for s in trans}

    for _ in range(num_steps):
        r = rng.random()
        cumulative = 0.0
        for (to, prob) in trans[state]:
            cumulative += prob
            if r < cumulative:
                state = to
                break
        counts[state] += 1

    return counts

transitions = [
    ('a', 'a', 0.9),  ('a', 'b', 0.075), ('a', 'c', 0.025),
    ('b', 'a', 0.15), ('b', 'b', 0.8),   ('b', 'c', 0.05),
    ('c', 'a', 0.25), ('c', 'b', 0.25),  ('c', 'c', 0.5),
]

counts = simulate_markov('a', transitions, 5000)
print(f"{{ 'a': {counts['a']}, 'b': {counts['b']}, 'c': {counts['c']} }}")
