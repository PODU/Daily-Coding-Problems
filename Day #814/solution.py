# Day 814: Add two numbers stored as reversed-digit linked lists via elementary addition
# with carry, walking both lists. Time O(max(m,n)), Space O(max(m,n)).


class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def build(digits):
    dummy = ListNode(0)
    cur = dummy
    for x in digits:
        cur.next = ListNode(x)
        cur = cur.next
    return dummy.next


def add_lists(a, b):
    dummy = ListNode(0)
    cur = dummy
    carry = 0
    while a or b or carry:
        s = carry + (a.val if a else 0) + (b.val if b else 0)
        carry, digit = divmod(s, 10)
        cur.next = ListNode(digit)
        cur = cur.next
        a = a.next if a else None
        b = b.next if b else None
    return dummy.next


def to_str(n):
    parts = []
    while n:
        parts.append(str(n.val))
        n = n.next
    return " -> ".join(parts)


def main():
    a = build([9, 9])
    b = build([5, 2])
    print(to_str(add_lists(a, b)))


if __name__ == "__main__":
    main()
