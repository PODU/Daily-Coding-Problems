# Day 1626: Add two numbers stored as reversed-digit linked lists.
# Single pass with carry. Time O(max(m,n)), Space O(max(m,n)).
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
            s += a.val; a = a.next
        if b:
            s += b.val; b = b.next
        carry, digit = divmod(s, 10)
        tail.next = Node(digit)
        tail = tail.next
    return dummy.next


def build(vals):
    dummy = t = Node(0)
    for x in vals:
        t.next = Node(x); t = t.next
    return dummy.next


if __name__ == "__main__":
    r = add_lists(build([9, 9]), build([5, 2]))
    parts = []
    while r:
        parts.append(str(r.val)); r = r.next
    print(" -> ".join(parts))
