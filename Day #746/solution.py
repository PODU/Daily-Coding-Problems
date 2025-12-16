# Day 746: Max stack: each entry stores the running max so far, giving O(1) push/pop/max.
# Time: O(1) per operation, Space: O(n).

class MaxStack:
    def __init__(self):
        self.st = []  # list of (value, max_so_far)

    def push(self, v):
        mx = v if not self.st else max(v, self.st[-1][1])
        self.st.append((v, mx))

    def pop(self):
        if not self.st:
            return None
        return self.st.pop()[0]

    def max(self):
        if not self.st:
            return None
        return self.st[-1][1]


if __name__ == "__main__":
    s = MaxStack()
    s.push(1); s.push(3); s.push(2)
    print("max:", s.max())  # 3
    print("pop:", s.pop())  # 2
    print("max:", s.max())  # 3
    print("pop:", s.pop())  # 3
    print("max:", s.max())  # 1
