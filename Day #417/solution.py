# Day 417: Remove all consecutive nodes summing to zero via prefix-sum + hashmap.
# Time O(n), Space O(n).


class ListNode:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def remove_zero_sum(head):
    dummy = ListNode(0, head)
    seen = {}
    prefix = 0
    node = dummy
    while node:
        prefix += node.val
        seen[prefix] = node  # keep latest node for this prefix sum
        node = node.next
    prefix = 0
    node = dummy
    while node:
        prefix += node.val
        node.next = seen[prefix].next  # skip zero-sum run
        node = node.next
    return dummy.next


def build(vals):
    dummy = ListNode(0)
    tail = dummy
    for v in vals:
        tail.next = ListNode(v)
        tail = tail.next
    return dummy.next


if __name__ == "__main__":
    head = remove_zero_sum(build([3, 4, -7, 5, -6, 6]))
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    print(" -> ".join(parts))
