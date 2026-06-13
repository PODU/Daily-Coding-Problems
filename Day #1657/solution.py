# Day 1657: Rotate singly linked list right by k. Make ring, break at n-(k%n). O(n) time, O(1) space.
class Node:
    def __init__(self, v):
        self.v = v
        self.next = None

def rotate_right(head, k):
    if not head or not head.next:
        return head
    n = 1; tail = head
    while tail.next:
        tail = tail.next; n += 1
    tail.next = head                  # ring
    steps = n - (k % n)
    new_tail = head
    for _ in range(steps - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head

def build(xs):
    h = t = None
    for x in xs:
        nd = Node(x)
        if h is None: h = t = nd
        else: t.next = nd; t = nd
    return h

def show(h):
    parts = []
    while h:
        parts.append(str(h.v)); h = h.next
    print(" -> ".join(parts))

if __name__ == "__main__":
    show(rotate_right(build([7, 7, 3, 5]), 2))
    show(rotate_right(build([1, 2, 3, 4, 5]), 3))
