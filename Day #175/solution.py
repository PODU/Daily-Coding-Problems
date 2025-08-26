# Day 175: Markov chain simulation: sample next state via cumulative probabilities, fixed-seed RNG.
# Time O(num_steps * avg_outdegree), Space O(states). (Exact counts depend on RNG.)
import random


def simulate(start, num_steps, transitions, seed=42):
    trans = {}
    for src, dst, prob in transitions:
        trans.setdefault(src, []).append((dst, prob))
    rng = random.Random(seed)
    counts = {}
    state = start
    for _ in range(num_steps):
        counts[state] = counts.get(state, 0) + 1
        r = rng.random()
        cum = 0.0
        for dst, prob in trans[state]:
            cum += prob
            if r < cum:
                state = dst
                break
    return counts


if __name__ == "__main__":
    transitions = [
        ('a', 'a', 0.9), ('a', 'b', 0.075), ('a', 'c', 0.025),
        ('b', 'a', 0.15), ('b', 'b', 0.8), ('b', 'c', 0.05),
        ('c', 'a', 0.25), ('c', 'b', 0.25), ('c', 'c', 0.5),
    ]
    counts = simulate('a', 5000, transitions)
    print("{'a': %d, 'b': %d, 'c': %d}" % (counts.get('a', 0), counts.get('b', 0), counts.get('c', 0)))
