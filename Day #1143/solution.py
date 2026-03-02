# Day 1143: Merge k sorted linked lists.
# Min-heap of list heads. Time O(N log k), Space O(k).
import heapq


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def merge_k(lists):
    heap = [(node.val, i, node) for i, node in enumerate(lists) if node]
    heapq.heapify(heap)
    dummy = tail = Node(0)
    while heap:
        _, i, node = heapq.heappop(heap)
        tail.next = node
        tail = node
        if node.next:
            heapq.heappush(heap, (node.next.val, i, node.next))
    return dummy.next


def build(vals):
    dummy = t = Node(0)
    for x in vals:
        t.next = Node(x)
        t = t.next
    return dummy.next


if __name__ == "__main__":
    lists = [build([1, 4, 7]), build([2, 5, 8]), build([3, 6, 9])]
    out, n = [], merge_k(lists)
    while n:
        out.append(str(n.val))
        n = n.next
    print(" -> ".join(out))  # 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
