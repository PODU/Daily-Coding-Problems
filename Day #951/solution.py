# Day 951: interleave first half of a stack with the reversed second half,
# in place using only one auxiliary queue (push/pop, enqueue/dequeue).
# Time O(n^2) due to rotations, Space O(1) extra besides the queue.
from collections import deque


def interleave(stack):
    stack = list(stack)  # top = end
    q = deque()
    while stack:
        q.append(stack.pop())
    while q:
        stack.append(q.popleft())
    while stack:
        q.append(stack.pop())  # q = [a0..a_{n-1}]
    while q:
        stack.append(q.popleft())  # front (low index)
        m = len(q)
        if m == 0:
            break
        for _ in range(m - 1):
            q.append(q.popleft())
        stack.append(q.popleft())  # back (high index)
    return stack  # bottom..top


if __name__ == "__main__":
    print(interleave([1, 2, 3, 4, 5]))  # [1, 5, 2, 4, 3]
    print(interleave([1, 2, 3, 4]))     # [1, 4, 2, 3]
