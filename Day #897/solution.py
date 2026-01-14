# Day 897: Queue via two stacks: enqueue->in_stack; dequeue moves all to out_stack when empty.
# Amortized O(1) per op, Space O(n).
class Queue:
    def __init__(self):
        self.in_stack = []
        self.out_stack = []

    def enqueue(self, x):
        self.in_stack.append(x)

    def dequeue(self):
        if not self.out_stack:
            while self.in_stack:
                self.out_stack.append(self.in_stack.pop())
        return self.out_stack.pop()


if __name__ == "__main__":
    q = Queue()
    q.enqueue(1)
    q.enqueue(2)
    print(q.dequeue())
    q.enqueue(3)
    print(q.dequeue())
    print(q.dequeue())
