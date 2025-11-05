# Day 559: Merge k sorted linked lists using a min-heap of (value, index, node).
# Time: O(N log k), Space: O(k) for the heap.
import heapq


class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def build_list(values):
    dummy = ListNode(0)
    cur = dummy
    for x in values:
        cur.next = ListNode(x)
        cur = cur.next
    return dummy.next


def merge_k_lists(lists):
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


def main():
    lists = [build_list([1, 4, 5]), build_list([1, 3, 4]), build_list([2, 6])]
    merged = merge_k_lists(lists)
    out = []
    n = merged
    while n:
        out.append(str(n.val))
        n = n.next
    print(" ".join(out))


if __name__ == "__main__":
    main()
