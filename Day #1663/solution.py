# Day 1663: Fisher-Yates shuffle of linked-list nodes via array: O(n) time, O(n) space.
# Space-over-time tradeoff: O(1)-extra approach repeatedly picks a random node by traversal in O(n^2) time.
import random

class Node:
    def __init__(self, val): self.val = val; self.next = None

def main():
    head = tail = None
    for v in range(1, 6):
        n = Node(v)
        if head is None: head = tail = n
        else: tail.next = n; tail = n
    a = []
    p = head
    while p: a.append(p); p = p.next
    rng = random.Random(12345)
    n = len(a)
    for i in range(n - 1, 0, -1):
        j = rng.randint(0, i)
        a[i], a[j] = a[j], a[i]
    for i in range(n):
        a[i].next = a[i + 1] if i + 1 < n else None
    head = a[0]
    orig = [1, 2, 3, 4, 5]
    shuf = []
    p = head
    while p: shuf.append(p.val); p = p.next
    valid = sorted(shuf) == orig
    print(" ".join(map(str, orig)) + " -> " + ("valid shuffle (same elements)" if valid else "INVALID"))

if __name__ == "__main__":
    main()
