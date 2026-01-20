# Day 930: Sort a linked list in O(n log n) time, O(1) extra space.
# Bottom-up (iterative) merge sort on the list; no recursion stack -> constant space.


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def split(head, size):
    """Advance `size` nodes, cut, return the start of the remainder."""
    for _ in range(size - 1):
        if head is None:
            break
        head = head.next
    if head is None:
        return None
    rest = head.next
    head.next = None
    return rest


def merge(a, b, tail):
    """Merge sorted lists a, b; append to tail; return new tail."""
    while a and b:
        if a.val <= b.val:
            tail.next = a
            a = a.next
        else:
            tail.next = b
            b = b.next
        tail = tail.next
    tail.next = a if a else b
    while tail.next:
        tail = tail.next
    return tail


def sort_list(head):
    if head is None or head.next is None:
        return head
    n = 0
    node = head
    while node:
        n += 1
        node = node.next
    dummy = Node(0, head)
    size = 1
    while size < n:
        cur = dummy.next
        tail = dummy
        while cur:
            left = cur
            right = split(left, size)
            cur = split(right, size)
            tail = merge(left, right, tail)
        size *= 2
    return dummy.next


def build(vals):
    dummy = Node(0)
    t = dummy
    for v in vals:
        t.next = Node(v)
        t = t.next
    return dummy.next


def to_str(head):
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    return " -> ".join(parts)


if __name__ == "__main__":
    head = build([4, 1, -3, 99])
    print(to_str(sort_list(head)))
