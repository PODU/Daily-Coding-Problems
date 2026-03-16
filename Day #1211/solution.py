# Day 1211: Interleave first half of a stack with the reversed second half, using one queue.
# Load stack bottom->top into the queue, then alternately push front/back (back via rotation). Time O(n^2), Space O(n).
from collections import deque


class Stack:
    def __init__(self):
        self._d = []

    def push(self, x):
        self._d.append(x)

    def pop(self):
        return self._d.pop()

    def empty(self):
        return not self._d

    def items(self):  # bottom -> top, for display only
        return list(self._d)


class Queue:
    def __init__(self):
        self._d = deque()

    def enqueue(self, x):
        self._d.append(x)

    def dequeue(self):
        return self._d.popleft()

    def size(self):
        return len(self._d)


def interleave(stack):
    q = Queue()
    n = 0
    while not stack.empty():        # pop all -> queue (front = old top)
        q.enqueue(stack.pop())
        n += 1
    for _ in range(n):              # queue -> stack (top = old bottom)
        stack.push(q.dequeue())
    for _ in range(n):              # pop all -> queue (front = bottom)
        q.enqueue(stack.pop())
    # queue front->back is now bottom->top; build interleaving on the stack
    remaining = n
    take_front = True
    while remaining > 0:
        if take_front:
            stack.push(q.dequeue())
        else:
            for _ in range(remaining - 1):  # rotate back element to the front
                q.enqueue(q.dequeue())
            stack.push(q.dequeue())
        remaining -= 1
        take_front = not take_front


if __name__ == "__main__":
    s = Stack()
    for x in [1, 2, 3, 4, 5]:  # bottom -> top
        s.push(x)
    interleave(s)
    print(s.items())  # [1, 5, 2, 4, 3]
