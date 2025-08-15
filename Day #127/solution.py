# Day 127: Add two numbers stored as reversed-digit linked lists.
# Single pass with carry. O(max(m,n)) time, O(1) extra space.


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def add_lists(a, b):
    dummy = Node(0)
    tail = dummy
    carry = 0
    while a or b or carry:
        s = carry
        if a:
            s += a.val
            a = a.next
        if b:
            s += b.val
            b = b.next
        carry = s // 10
        tail.next = Node(s % 10)
        tail = tail.next
    return dummy.next


def build(d):
    dummy = Node(0)
    t = dummy
    for v in d:
        t.next = Node(v)
        t = t.next
    return dummy.next


def to_str(n):
    parts = []
    while n:
        parts.append(str(n.val))
        n = n.next
    return " -> ".join(parts)


if __name__ == "__main__":
    a = build([9, 9])  # 99
    b = build([5, 2])  # 25
    print(to_str(add_lists(a, b)))  # 4 -> 2 -> 1
