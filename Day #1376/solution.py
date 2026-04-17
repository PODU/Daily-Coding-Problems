# Day 1376: Uniform linked-list shuffle via Fisher-Yates. Time O(n), Space O(n) for the
# index pass (space-over-time variant: O(1) space, O(n^2) by random node selection).
# A deterministic LCG is used so output is reproducible.


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


_seed = 42


def next_rand():
    global _seed
    _seed = (_seed * 1103515245 + 12345) % 2147483648
    return _seed


def shuffle(head):
    nodes = []
    p = head
    while p:
        nodes.append(p)
        p = p.next
    for i in range(len(nodes) - 1, 0, -1):
        j = next_rand() % (i + 1)
        nodes[i].val, nodes[j].val = nodes[j].val, nodes[i].val
    return head


if __name__ == "__main__":
    head = tail = None
    for v in range(1, 6):
        node = Node(v)
        if head is None:
            head = tail = node
        else:
            tail.next = node
            tail = node
    head = shuffle(head)
    out = []
    p = head
    while p:
        out.append(str(p.val))
        p = p.next
    print(" -> ".join(out))
