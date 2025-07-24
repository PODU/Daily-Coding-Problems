# Day 26: Remove kth-from-last node in one pass with two pointers. Time O(n), Space O(1).
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def remove_kth_from_last(head, k):
    dummy = Node(0)
    dummy.next = head
    lead = trail = dummy
    for _ in range(k):
        lead = lead.next
    while lead.next:
        lead = lead.next
        trail = trail.next
    trail.next = trail.next.next
    return dummy.next


if __name__ == "__main__":
    head = Node(1)
    head.next = Node(2)
    head.next.next = Node(3)
    head.next.next.next = Node(4)
    head.next.next.next.next = Node(5)
    head = remove_kth_from_last(head, 2)
    parts = []
    c = head
    while c:
        parts.append(str(c.val))
        c = c.next
    print(" -> ".join(parts))
