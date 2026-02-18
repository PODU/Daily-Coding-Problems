# Day 1094: Uniformly shuffle a linked list via Fisher-Yates on node values. Time O(n), Space O(n).
# Space-over-time alternative: walk to a random remaining node and swap in place -> O(1) extra, O(n^2) time.
import random


class Node:
    def __init__(self, v):
        self.val = v
        self.next = None


def build(arr):
    head = tail = None
    for x in arr:
        n = Node(x)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    return head


def to_list(h):
    a = []
    while h:
        a.append(h.val)
        h = h.next
    return a


def shuffle_list(head, rng):
    nodes = []
    c = head
    while c:
        nodes.append(c)
        c = c.next
    for i in range(len(nodes) - 1, 0, -1):
        j = rng.randint(0, i)
        nodes[i].val, nodes[j].val = nodes[j].val, nodes[i].val


if __name__ == "__main__":
    head = build([1, 2, 3, 4, 5])
    print("Original:", to_list(head))
    rng = random.Random(42)
    shuffle_list(head, rng)
    print("Shuffled:", to_list(head))
