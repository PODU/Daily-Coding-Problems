# Day 1537: Simulate a Markov chain for num_steps and count visits per state.
# Time O(num_steps * outdegree), Space O(states + transitions).
import random
from collections import defaultdict


def run_chain(start, num_steps, transitions, seed=42):
    trans = defaultdict(list)
    for src, dst, p in transitions:
        trans[src].append((dst, p))
    rng = random.Random(seed)
    counts = defaultdict(int)
    cur = start
    for _ in range(num_steps):
        counts[cur] += 1
        r, acc = rng.random(), 0.0
        for dst, p in trans[cur]:
            acc += p
            if r <= acc:
                cur = dst
                break
    return dict(counts)


if __name__ == "__main__":
    transitions = [
        ('a', 'a', 0.9), ('a', 'b', 0.075), ('a', 'c', 0.025),
        ('b', 'a', 0.15), ('b', 'b', 0.8), ('b', 'c', 0.05),
        ('c', 'a', 0.25), ('c', 'b', 0.25), ('c', 'c', 0.5),
    ]
    counts = run_chain('a', 5000, transitions)
    print({k: counts[k] for k in sorted(counts)})
