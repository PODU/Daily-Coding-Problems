# Day 956: merge k sorted singly linked lists using a min-heap.
# Time O(N log k), Space O(k) where N = total nodes.
import heapq


class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def merge_k(lists):
    heap = []
    for i, node in enumerate(lists):
        if node:
            heapq.heappush(heap, (node.val, i, node))
    dummy = ListNode(0)
    tail = dummy
    while heap:
        val, i, node = heapq.heappop(heap)
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
    m = merge_k(lists)
    out = []
    while m:
        out.append(m.val)
        m = m.next
    print(" ".join(map(str, out)))  # 1 1 2 3 4 4 5 6
