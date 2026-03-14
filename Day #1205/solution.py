# Day 1205: Add two numbers stored as reversed-digit linked lists.
# Traverse both lists with a running carry. Time O(max(m,n)), Space O(max(m,n)).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def build(ds):
    dummy = Node(0)
    t = dummy
    for d in ds:
        t.next = Node(d)
        t = t.next
    return dummy.next


def add_lists(a, b):
    dummy = Node(0)
    t = dummy
    carry = 0
    while a or b or carry:
        s = carry + (a.val if a else 0) + (b.val if b else 0)
        carry, digit = divmod(s, 10)
        t.next = Node(digit)
        t = t.next
        a = a.next if a else None
        b = b.next if b else None
    return dummy.next


if __name__ == "__main__":
    s = add_lists(build([9, 9]), build([5, 2]))
    out = []
    while s:
        out.append(str(s.val))
        s = s.next
    print(" -> ".join(out))  # 4 -> 2 -> 1
