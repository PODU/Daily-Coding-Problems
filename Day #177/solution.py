# Day 177: Rotate singly linked list right by k: form ring, break at (len - k%len). Time O(n), Space O(1).


class Node:
    def __init__(self, val, next=None):
        self.val = val
        self.next = next


def rotate_right(head, k):
    if not head or not head.next or k == 0:
        return head
    length = 1
    tail = head
    while tail.next:
        tail = tail.next
        length += 1
    k %= length
    if k == 0:
        return head
    tail.next = head  # make ring
    steps = length - k
    new_tail = head
    for _ in range(steps - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head


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


if __name__ == "__main__":
    print(to_str(rotate_right(build([7, 7, 3, 5]), 2)))
    print(to_str(rotate_right(build([1, 2, 3, 4, 5]), 3)))
