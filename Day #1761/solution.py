# Day 1761: Interleave first half of a stack with the reversed second half,
# in-place using ONE auxiliary queue (only push/pop/enqueue/dequeue).
# Dump stack into queue (reverses), rebuild taking alternately back/front.
# Time O(n^2), Space O(n). Stack modeled as list (end = top).
from collections import deque


def interleave(input_list):
    stack = list(input_list)          # end = top
    q = deque()
    while stack:
        q.append(stack.pop())         # pop top -> enqueue
    take_back = True
    while q:
        if take_back:
            for _ in range(len(q) - 1):
                q.append(q.popleft())
            v = q.popleft()
        else:
            v = q.popleft()
        stack.append(v)               # push
        take_back = not take_back
    return stack


if __name__ == "__main__":
    print(interleave([1, 2, 3, 4, 5]))
    print(interleave([1, 2, 3, 4]))
