# Day 1623: Sentence equivalence via synonym set.
# Build symmetric synonym set; compare word-by-word. Time O(W), Space O(S).
def equivalent(a, b, syns):
    pairs = set()
    for x, y in syns:
        pairs.add((x, y))
        pairs.add((y, x))
    wa, wb = a.split(), b.split()
    if len(wa) != len(wb):
        return False
    return all(x == y or (x, y) in pairs for x, y in zip(wa, wb))


if __name__ == "__main__":
    syns = [("big", "large"), ("eat", "consume")]
    eq = equivalent("He wants to eat food.", "He wants to consume food.", syns)
    print("The two sentences are equivalent." if eq else "The two sentences are not equivalent.")
