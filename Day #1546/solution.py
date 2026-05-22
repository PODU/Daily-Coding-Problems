# Day 1546: Stable partition of a linked list around pivot k.
# Build two sublists (< k) and (>= k) preserving order, then splice. Time O(n), Space O(1).
class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def partition(head, k):
    less_d = Node(0)
    ge_d = Node(0)
    l, g = less_d, ge_d
    c = head
    while c:
        if c.val < k:
            l.next = c
            l = c
        else:
            g.next = c
            g = c
        c = c.next
    g.next = None
    l.next = ge_d.next
    return less_d.next


def build(vals):
    head = tail = None
    for v in vals:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    return head


def to_str(head):
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    return " -> ".join(parts)


if __name__ == "__main__":
    head = build([5, 1, 8, 0, 3])
    head = partition(head, 3)
    print(to_str(head))
