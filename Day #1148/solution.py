# Day 1148: Rotate linked list right by k.
# Find length, close into ring, cut at (len - k%len). O(n) time, O(1) space.
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def rotate(head, k):
    if not head or not head.next:
        return head
    length, tail = 1, head
    while tail.next:
        tail = tail.next
        length += 1
    k %= length
    if k == 0:
        return head
    tail.next = head
    steps = length - k
    new_tail = head
    for _ in range(steps - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head


def build(vals):
    dummy = t = Node(0)
    for x in vals:
        t.next = Node(x)
        t = t.next
    return dummy.next


def to_str(h):
    out = []
    while h:
        out.append(str(h.val))
        h = h.next
    return " -> ".join(out)


if __name__ == "__main__":
    print(to_str(rotate(build([7, 7, 3, 5]), 2)))     # 3 -> 5 -> 7 -> 7
    print(to_str(rotate(build([1, 2, 3, 4, 5]), 3)))  # 3 -> 4 -> 5 -> 1 -> 2
