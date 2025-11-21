# Day 640: Simulate a Markov chain and tally state visits.
# Approach: build outgoing-edge table, draw next state by cumulative prob.
# Time: O(num_steps * avg_out_degree), Space: O(states + edges).
# Note: output is stochastic; counts approximate the README sample (sum 5000).
import random
from collections import defaultdict


def run_markov(start, transitions, num_steps, seed=42):
    adj = defaultdict(list)
    for src, dst, p in transitions:
        adj[src].append((dst, p))
    rng = random.Random(seed)
    counts = defaultdict(int)
    cur = start
    for _ in range(num_steps):
        counts[cur] += 1
        r, acc = rng.random(), 0.0
        for dst, p in adj[cur]:
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
    counts = run_markov('a', transitions, 5000)
    print({k: counts[k] for k in sorted(counts)})  # e.g. {'a': 3012, 'b': 1656, 'c': 332}
