# Day 1025: Remove all consecutive linked-list nodes that sum to zero.
# Approach: prefix-sum + dict of last node per prefix sum (dummy head). O(N) time, O(N) space.


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def remove_zero_sum(head):
    dummy = Node(0, head)
    last = {}
    s = 0
    cur = dummy
    while cur:
        s += cur.val
        last[s] = cur  # keep last occurrence
        cur = cur.next
    s = 0
    cur = dummy
    while cur:
        s += cur.val
        cur.next = last[s].next
        cur = cur.next
    return dummy.next


def build(vals):
    head = None
    for v in reversed(vals):
        head = Node(v, head)
    return head


if __name__ == "__main__":
    head = remove_zero_sum(build([3, 4, -7, 5, -6, 6]))
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    print(" -> ".join(parts))
