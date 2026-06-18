# Day 1689: Sort a singly linked list via bottom-up (iterative) merge sort.
# O(n log n) time, O(1) extra space (no recursion).

class Node:
    __slots__ = ("val", "next")
    def __init__(self, val):
        self.val = val
        self.next = None

def length(h):
    n = 0
    while h:
        n += 1
        h = h.next
    return n

def split(head, n):
    # Cut list to first n nodes; return head of the remainder.
    i = 1
    while head and i < n:
        head = head.next
        i += 1
    if not head:
        return None
    second = head.next
    head.next = None
    return second

def merge(a, b):
    dummy = Node(0)
    tail = dummy
    while a and b:
        if a.val <= b.val:
            tail.next = a; a = a.next
        else:
            tail.next = b; b = b.next
        tail = tail.next
    tail.next = a if a else b
    return dummy.next

def sort_list(head):
    if not head:
        return None
    n = length(head)
    dummy = Node(0)
    dummy.next = head
    size = 1
    while size < n:
        prev = dummy
        cur = dummy.next
        while cur:
            left = cur
            right = split(left, size)
            cur = split(right, size)
            prev.next = merge(left, right)
            while prev.next:
                prev = prev.next
        size <<= 1
    return dummy.next

if __name__ == "__main__":
    head = None
    tail = None
    for v in [4, 1, -3, 99]:
        nd = Node(v)
        if head is None:
            head = tail = nd
        else:
            tail.next = nd; tail = nd
    head = sort_list(head)
    parts = []
    p = head
    while p:
        parts.append(str(p.val))
        p = p.next
    print(" -> ".join(parts))
