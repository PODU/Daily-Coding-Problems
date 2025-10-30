# Day 515: Partition list: build "less than k" and ">= k" sublists, then join. O(n) time, O(1) extra.
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
    head = None
    for v in reversed(vals):
        head = Node(v, head)
    return head


def show(head):
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    return " -> ".join(parts)


if __name__ == "__main__":
    print(show(partition(build([5, 1, 8, 0, 3]), 3)))
