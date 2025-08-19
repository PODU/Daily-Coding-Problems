# Day 141: Three stacks in one list via interleaved indexing: logical pos p of stack s -> physical p*3+s.
# push/pop O(1) amortized. Space O(total elements). Single backing list.


class Stack:
    def __init__(self):
        self.list = []          # single backing list
        self.sizes = [0, 0, 0]  # logical height of each stack

    def push(self, item, stack_number):
        phys = self.sizes[stack_number] * 3 + stack_number
        while len(self.list) <= phys:
            self.list.append(0)
        self.list[phys] = item
        self.sizes[stack_number] += 1

    def pop(self, stack_number):
        if self.sizes[stack_number] == 0:
            raise IndexError("empty stack")
        self.sizes[stack_number] -= 1
        phys = self.sizes[stack_number] * 3 + stack_number
        return self.list[phys]


if __name__ == "__main__":
    s = Stack()
    s.push(1, 0); s.push(2, 0)
    s.push(10, 1)
    s.push(100, 2); s.push(200, 2)
    print(s.pop(0), s.pop(2), s.pop(1), s.pop(2), s.pop(0))
    # 2 200 10 100 1
