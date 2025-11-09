# Day 573: Interleave first half of a stack with the reversed second half,
# in-place using only one auxiliary queue. O(N) time, O(N) space.
from collections import deque


def interleave(stack):
    n = len(stack)
    q = deque()
    while stack:
        q.append(stack.pop())          # whole stack -> queue (top..bottom)
    base = list(q)[::-1]               # bottom..top
    res = []
    i, j, front = 0, n - 1, True
    while i <= j:
        if front:
            res.append(base[i]); i += 1
        else:
            res.append(base[j]); j -= 1
        front = not front
    return res


def demo(bottom_to_top):
    stack = list(bottom_to_top)  # list used as stack, append/pop = top
    print(interleave(stack))


if __name__ == "__main__":
    demo([1, 2, 3, 4, 5])  # [1, 5, 2, 4, 3]
    demo([1, 2, 3, 4])     # [1, 4, 2, 3]
