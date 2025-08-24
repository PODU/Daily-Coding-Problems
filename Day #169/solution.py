# Day 169: Bottom-up (iterative) merge sort on a singly linked list. O(n log n) time, O(1) space.

class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt

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
    while tail.next:
        tail = tail.next
    return dummy.next, tail

def length(h):
    n = 0
    while h:
        n += 1; h = h.next
    return n

def split(head, n):
    for _ in range(n - 1):
        if not head:
            break
        head = head.next
    if not head:
        return None
    rest = head.next
    head.next = None
    return rest

def sort_list(head):
    n = length(head)
    dummy = Node(0); dummy.next = head
    size = 1
    while size < n:
        cur = dummy.next
        tail = dummy
        while cur:
            left = cur
            right = split(left, size)
            cur = split(right, size)
            merged_head, merged_tail = merge(left, right)
            tail.next = merged_head
            tail = merged_tail
        size <<= 1
    return dummy.next

def main():
    vals = [4, 1, -3, 99]
    dummy = Node(0); t = dummy
    for v in vals:
        t.next = Node(v); t = t.next
    head = sort_list(dummy.next)
    parts = []
    p = head
    while p:
        parts.append(str(p.val)); p = p.next
    print(" -> ".join(parts))

if __name__ == "__main__":
    main()
