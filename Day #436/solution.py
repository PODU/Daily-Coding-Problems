# Day 436: Three stacks backed by one list using node-based singly-linked
# indices + a free list. push/pop are O(1) time, O(n) space overall.

class Stack:
    def __init__(self):
        self.list = []            # single backing list of [val, prev] nodes
        self.tops = [-1, -1, -1]  # head index per stack
        self.free = []            # recycled slots

    def push(self, item, stack_number):
        if self.free:
            idx = self.free.pop()
            self.list[idx] = [item, self.tops[stack_number]]
        else:
            idx = len(self.list)
            self.list.append([item, self.tops[stack_number]])
        self.tops[stack_number] = idx

    def pop(self, stack_number):
        idx = self.tops[stack_number]
        if idx == -1:
            raise IndexError(f"stack {stack_number} is empty")
        val, prev = self.list[idx]
        self.tops[stack_number] = prev
        self.free.append(idx)
        return val


if __name__ == "__main__":
    st = Stack()
    st.push(1, 0); st.push(2, 0)
    st.push(10, 1)
    st.push(100, 2); st.push(200, 2)
    print(st.pop(0))  # 2
    print(st.pop(0))  # 1
    print(st.pop(1))  # 10
    print(st.pop(2))  # 200
    print(st.pop(2))  # 100
