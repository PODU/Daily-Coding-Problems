# Day 798: Sentence equivalence via synonym pairs (NOT transitive).
# Store each unordered pair in a set; words match if equal or directly paired.
# Time O(W) per comparison, Space O(P).


class Equiv:
    def __init__(self):
        self.syn = set()

    def add(self, a, b):
        self.syn.add(frozenset((a, b)))

    def same(self, a, b):
        return a == b or frozenset((a, b)) in self.syn

    def equivalent(self, s1, s2):
        if len(s1) != len(s2):
            return False
        return all(self.same(x, y) for x, y in zip(s1, s2))


if __name__ == "__main__":
    e = Equiv()
    e.add("big", "large")
    e.add("eat", "consume")
    a = "He wants to eat food.".split()
    b = "He wants to consume food.".split()
    print("True (equivalent)" if e.equivalent(a, b) else "False")
