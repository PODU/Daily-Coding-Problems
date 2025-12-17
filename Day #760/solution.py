# Day 760: Uniformly shuffle a linked list. Space-prioritized variant:
# forward Fisher-Yates that swaps node values in place using O(1) extra space
# at the cost of O(n^2) time (re-walks to pick a uniform remaining node).
# A deterministic LCG is used so the demo output is reproducible.


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


class LCG:
    def __init__(self, seed):
        self.s = seed

    def next(self):
        self.s = (self.s * 1103515245 + 12345) & 0x7fffffff
        return self.s


def shuffle(head, rng):
    p = head
    while p:
        m = 0
        t = p
        while t:
            m += 1
            t = t.next
        r = rng.next() % m
        q = p
        while r > 0:
            q = q.next
            r -= 1
        p.val, q.val = q.val, p.val
        p = p.next


def print_list(head):
    parts = []
    p = head
    while p:
        parts.append(str(p.val))
        p = p.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    head = Node(1)
    cur = head
    for v in range(2, 6):
        cur.next = Node(v)
        cur = cur.next

    print("original: ", end="")
    print_list(head)
    shuffle(head, LCG(42))
    print("shuffled: ", end="")
    print_list(head)
