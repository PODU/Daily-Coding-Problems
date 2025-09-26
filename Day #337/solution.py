# Day 337: Shuffle linked list uniformly via Fisher-Yates on node values.
# O(n) time, O(1) extra (in-place value swaps). Fixed seed -> deterministic.
import random


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def main():
    head = tail = None
    for v in range(1, 6):
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n

    nodes = []
    p = head
    while p:
        nodes.append(p)
        p = p.next

    rng = random.Random(42)
    for i in range(len(nodes) - 1, 0, -1):
        j = rng.randint(0, i)
        nodes[i].val, nodes[j].val = nodes[j].val, nodes[i].val

    out = []
    p = head
    while p:
        out.append(str(p.val))
        p = p.next
    print(" ".join(out))


if __name__ == "__main__":
    main()
