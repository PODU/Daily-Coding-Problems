# Day 699: Rotate a singly linked list right by k places.
# Approach: close into a ring, then break it (len - k%len) nodes ahead.
# Time O(n), Space O(1).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def rotate_right(head, k):
    if not head or not head.next:
        return head
    length, tail = 1, head
    while tail.next:
        tail = tail.next
        length += 1
    k %= length
    if k == 0:
        return head
    tail.next = head  # ring
    steps = length - k
    new_tail = head
    for _ in range(steps - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head


def build(vals):
    dummy = Node(0)
    cur = dummy
    for x in vals:
        cur.next = Node(x)
        cur = cur.next
    return dummy.next


def show(h):
    out = []
    while h:
        out.append(str(h.val))
        h = h.next
    print(" -> ".join(out))


if __name__ == "__main__":
    show(rotate_right(build([7, 7, 3, 5]), 2))     # 3 -> 5 -> 7 -> 7
    show(rotate_right(build([1, 2, 3, 4, 5]), 3))  # 3 -> 4 -> 5 -> 1 -> 2
