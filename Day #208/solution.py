# Day 208: Partition a linked list around pivot k (stable).
# Build two lists (< k and >= k) in original order, then splice. Time: O(n), Space: O(1).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def partition(head, k):
    less_dummy = Node(0)
    ge_dummy = Node(0)
    less, ge = less_dummy, ge_dummy
    cur = head
    while cur:
        if cur.val < k:
            less.next = cur
            less = cur
        else:
            ge.next = cur
            ge = cur
        cur = cur.next
    ge.next = None
    less.next = ge_dummy.next
    return less_dummy.next


def build(vals):
    dummy = Node(0)
    t = dummy
    for x in vals:
        t.next = Node(x)
        t = t.next
    return dummy.next


def to_str(h):
    parts = []
    while h:
        parts.append(str(h.val))
        h = h.next
    return " -> ".join(parts)


if __name__ == "__main__":
    print(to_str(partition(build([5, 1, 8, 0, 3]), 3)))  # 1 -> 0 -> 5 -> 8 -> 3
