# Day 927: Rotate list right by k: find length L, make a ring, break at L-(k%L).
# Time O(n), Space O(1).
class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def build(vals):
    head = None
    for v in reversed(vals):
        head = Node(v, head)
    return head


def to_str(head):
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    return " -> ".join(parts)


def rotate_right(head, k):
    if not head or not head.next:
        return head
    L = 1
    tail = head
    while tail.next:
        tail = tail.next
        L += 1
    k %= L
    if k == 0:
        return head
    tail.next = head  # ring
    steps = L - k
    new_tail = head
    for _ in range(steps - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head


if __name__ == "__main__":
    head = build([1, 2, 3, 4, 5])
    print(to_str(rotate_right(head, 3)))
