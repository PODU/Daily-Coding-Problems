# Day 1486: Partition a linked list around pivot k (stable).
# Approach: build two sublists (< k and >= k), then concatenate. O(n) time, O(1) extra space.

class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def partition(head, k):
    less_dummy = Node(0)
    ge_dummy = Node(0)
    less = less_dummy
    ge = ge_dummy
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
    head = tail = None
    for v in vals:
        n = Node(v)
        if not head:
            head = tail = n
        else:
            tail.next = n
            tail = n
    return head


if __name__ == "__main__":
    head = partition(build([5, 1, 8, 0, 3]), 3)
    out = []
    while head:
        out.append(str(head.val))
        head = head.next
    print(" -> ".join(out))
