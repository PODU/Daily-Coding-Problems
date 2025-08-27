# Day 180: Interleave first half with reversed second half using ONE auxiliary queue.
# Result (bottom->top) = arr[0],arr[n-1],arr[1],arr[n-2],...  O(n^2) time, O(n) space.
from collections import deque

def interleave(stack):
    # stack: list with top at end (append/pop). queue: deque (append=enqueue, popleft=dequeue).
    q = deque()
    while stack:        # A: stack -> queue
        q.append(stack.pop())
    while q:            # B: queue -> stack (reverse)
        stack.append(q.popleft())
    while stack:        # C: stack -> queue (front..back = bottom..top)
        q.append(stack.pop())

    take_front = True
    while q:
        if take_front:
            stack.append(q.popleft())
        else:
            for _ in range(len(q) - 1):  # rotate back element to front
                q.append(q.popleft())
            stack.append(q.popleft())
        take_front = not take_front
    return stack

def main():
    for inp in ([1, 2, 3, 4, 5], [1, 2, 3, 4]):
        print(interleave(inp[:]))

if __name__ == "__main__":
    main()
