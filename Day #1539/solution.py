# Day 1539: Remove all consecutive nodes summing to zero using prefix sums + hash map.
# A prefix sum seen before means the span between is zero-sum and is excised.
# Time O(n), Space O(n).
class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def remove_zero_sum(head):
    dummy = Node(0, head)
    seen = {}
    s = 0
    p = dummy
    while p:
        s += p.val
        seen[s] = p
        p = p.next
    s = 0
    p = dummy
    while p:
        s += p.val
        p.next = seen[s].next
        p = p.next
    return dummy.next


if __name__ == "__main__":
    head = None
    for v in reversed([3, 4, -7, 5, -6, 6]):
        head = Node(v, head)
    head = remove_zero_sum(head)
    out = []
    p = head
    while p:
        out.append(str(p.val))
        p = p.next
    print(" ".join(out))
