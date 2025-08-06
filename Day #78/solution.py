# Day 78: Merge k sorted linked lists using a min-heap.
# Time O(N log k) where N total nodes, Space O(k).
import heapq


class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def merge_k_lists(lists):
    heap = []
    for i, node in enumerate(lists):
        if node:
            heapq.heappush(heap, (node.val, i, node))
    dummy = ListNode(0)
    tail = dummy
    while heap:
        _, i, node = heapq.heappop(heap)
        tail.next = node
        tail = node
        if node.next:
            heapq.heappush(heap, (node.next.val, i, node.next))
    return dummy.next


def build(vals):
    dummy = ListNode(0)
    t = dummy
    for x in vals:
        t.next = ListNode(x)
        t = t.next
    return dummy.next


if __name__ == "__main__":
    lists = [build([1, 4, 5]), build([1, 3, 4]), build([2, 6])]
    m = merge_k_lists(lists)
    out = []
    while m:
        out.append(str(m.val))
        m = m.next
    print(" -> ".join(out))  # 1 -> 1 -> 2 -> 3 -> 4 -> 4 -> 5 -> 6
