# Day 452: Add two numbers stored as reversed-digit linked lists.
# Single pass with carry. Time O(max(n,m)), Space O(max(n,m)).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def add_lists(a, b):
    dummy = tail = Node(0)
    carry = 0
    while a or b or carry:
        s = carry
        if a:
            s += a.val
            a = a.next
        if b:
            s += b.val
            b = b.next
        carry, digit = divmod(s, 10)
        tail.next = Node(digit)
        tail = tail.next
    return dummy.next


def build(xs):
    dummy = tail = Node(0)
    for x in xs:
        tail.next = Node(x)
        tail = tail.next
    return dummy.next


def show(n):
    parts = []
    while n:
        parts.append(str(n.val))
        n = n.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    show(add_lists(build([9, 9]), build([5, 2])))  # 4 -> 2 -> 1
