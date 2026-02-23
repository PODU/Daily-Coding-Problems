# Day 1117: Day 1117 - Three stacks backed by a single list
# Approach: each entry stores (value, prev_index); per-stack head pointers and a
# free-index list let push/pop be O(1) while sharing one list. Space O(n).

class Stack:
    def __init__(self):
        self.list = []      # entries: [value, prev_index]
        self.heads = [-1, -1, -1]
        self.free = []

    def push(self, item, stack_number):
        if self.free:
            idx = self.free.pop()
            self.list[idx] = [item, self.heads[stack_number]]
        else:
            idx = len(self.list)
            self.list.append([item, self.heads[stack_number]])
        self.heads[stack_number] = idx

    def pop(self, stack_number):
        idx = self.heads[stack_number]
        if idx == -1:
            raise IndexError("pop from empty stack " + str(stack_number))
        value, prev = self.list[idx]
        self.heads[stack_number] = prev
        self.free.append(idx)
        return value


if __name__ == "__main__":
    s = Stack()
    s.push(1, 0); s.push(2, 0)
    s.push(3, 1)
    s.push(4, 2); s.push(5, 2)
    print("pop(0):", s.pop(0))  # 2
    print("pop(0):", s.pop(0))  # 1
    print("pop(2):", s.pop(2))  # 5
    print("pop(1):", s.pop(1))  # 3
    print("pop(2):", s.pop(2))  # 4
