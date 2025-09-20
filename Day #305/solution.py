# Day 305: Remove consecutive nodes summing to zero. Prefix-sum + hashmap. O(N).
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def remove_zero_sum(head):
    dummy = Node(0)
    dummy.next = head
    seen = {}
    prefix = 0
    cur = dummy
    while cur:
        prefix += cur.val
        seen[prefix] = cur  # last occurrence
        cur = cur.next
    prefix = 0
    cur = dummy
    while cur:
        prefix += cur.val
        cur.next = seen[prefix].next
        cur = cur.next
    return dummy.next


if __name__ == "__main__":
    head = None
    tail = None
    for v in [3, 4, -7, 5, -6, 6]:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    head = remove_zero_sum(head)
    out = []
    while head:
        out.append(head.val)
        head = head.next
    print(" ".join(map(str, out)))  # 5
