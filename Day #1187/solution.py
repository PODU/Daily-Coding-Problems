# Day 1187: Markov chain simulation: cumulative transition table, draw uniform RNG per step.
# Result is stochastic/approximate (fixed seed for reproducibility). Time O(steps), Space O(states^2).
import random
from collections import defaultdict


def simulate(start, num_steps, transitions, seed=12345):
    raw = defaultdict(list)
    for frm, to, prob in transitions:
        raw[frm].append((prob, to))
    table = {}
    for st, lst in raw.items():
        cum = 0.0
        table[st] = []
        for prob, to in lst:
            cum += prob
            table[st].append((cum, to))

    rng = random.Random(seed)
    counts = defaultdict(int)
    state = start
    for _ in range(num_steps):
        r = rng.random()
        for cum, to in table[state]:
            if r < cum:
                state = to
                break
        counts[state] += 1
    return counts


def main():
    transitions = [
        ('a', 'a', 0.9), ('a', 'b', 0.075), ('a', 'c', 0.025),
        ('b', 'a', 0.15), ('b', 'b', 0.8), ('b', 'c', 0.05),
        ('c', 'a', 0.25), ('c', 'b', 0.25), ('c', 'c', 0.5),
    ]
    counts = simulate('a', 5000, transitions)
    ordered = {k: counts.get(k, 0) for k in sorted(counts)}
    print(ordered)


if __name__ == '__main__':
    main()
