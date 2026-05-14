# Day 1510: Three stacks sharing ONE backing list of nodes (value, prevIndex) + free list.
# Three head pointers index into the single shared list. O(1) push/pop, O(n) space.

class ThreeStacks:
    def __init__(self):
        self.val = []
        self.prev = []
        self.head = [-1, -1, -1]
        self.free_head = -1

    def _alloc(self, v, p):
        if self.free_head != -1:
            idx = self.free_head
            self.free_head = self.prev[idx]
            self.val[idx] = v
            self.prev[idx] = p
        else:
            idx = len(self.val)
            self.val.append(v)
            self.prev.append(p)
        return idx

    def push(self, item, s):
        self.head[s] = self._alloc(item, self.head[s])

    def pop(self, s):
        idx = self.head[s]
        v = self.val[idx]
        self.head[s] = self.prev[idx]
        self.prev[idx] = self.free_head
        self.free_head = idx
        return v


if __name__ == "__main__":
    st = ThreeStacks()
    st.push(1, 0); st.push(2, 0)
    st.push(3, 1)
    st.push(4, 2); st.push(5, 2)
    print(st.pop(0), st.pop(2), st.pop(1), st.pop(0))
