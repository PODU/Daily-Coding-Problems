# Day 1310: Rearrange linked list values to low->high->low->high. One pass swapping
# adjacent values to enforce the alternating relation. Time O(n), Space O(1).

class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def zigzag(head):
    low = True  # current pair should satisfy a <= b
    c = head
    while c and c.next:
        if (low and c.val > c.next.val) or (not low and c.val < c.next.val):
            c.val, c.next.val = c.next.val, c.val
        c = c.next
        low = not low


def build(vals):
    head = tail = None
    for v in vals:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    return head


def to_str(head):
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    return " -> ".join(parts)


if __name__ == "__main__":
    h = build([1, 2, 3, 4, 5])
    zigzag(h)
    print(to_str(h))  # 1 -> 3 -> 2 -> 5 -> 4
