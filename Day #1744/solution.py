# Day 1744: Merge k sorted linked lists via min-heap of current heads. O(N log k) time, O(k) space.
import heapq


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def build(vals):
    dummy = Node(0)
    t = dummy
    for x in vals:
        t.next = Node(x)
        t = t.next
    return dummy.next


def merge_k(lists):
    heap = []
    for i, node in enumerate(lists):
        if node:
            heapq.heappush(heap, (node.val, i, node))
    dummy = Node(0)
    tail = dummy
    while heap:
        _, i, node = heapq.heappop(heap)
        tail.next = node
        tail = node
        if node.next:
            heapq.heappush(heap, (node.next.val, i, node.next))
    return dummy.next


def main():
    lists = [build([1, 4, 5]), build([1, 3, 4]), build([2, 6])]
    m = merge_k(lists)
    out = []
    p = m
    while p:
        out.append(str(p.val))
        p = p.next
    print(" ".join(out))


if __name__ == "__main__":
    main()
