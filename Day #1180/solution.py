# Day 1180: Swap every two adjacent nodes in a singly linked list.
# Iterative pointer rewiring with a dummy head. Time O(N), Space O(1).


class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def swap_pairs(head):
    dummy = ListNode(0)
    dummy.next = head
    prev = dummy
    while prev.next and prev.next.next:
        a, b = prev.next, prev.next.next
        a.next = b.next
        b.next = a
        prev.next = b
        prev = a
    return dummy.next


def build(vals):
    dummy = ListNode(0)
    t = dummy
    for x in vals:
        t.next = ListNode(x)
        t = t.next
    return dummy.next


def to_str(h):
    parts = []
    while h:
        parts.append(str(h.val))
        h = h.next
    return " -> ".join(parts)


if __name__ == "__main__":
    print(to_str(swap_pairs(build([1, 2, 3, 4]))))  # 2 -> 1 -> 4 -> 3
