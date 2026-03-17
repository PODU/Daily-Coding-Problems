# Day 1218: Three stacks in one list via fixed equal regions, each with its own top. O(1) push/pop.

class ThreeStacks:
    def __init__(self, per_stack: int = 3):
        self.cap = per_stack
        self.arr = [0] * (3 * per_stack)
        self.top = [0, 0, 0]

    def push(self, item: int, sn: int) -> None:
        if self.top[sn] >= self.cap:
            raise OverflowError("stack full")
        self.arr[sn * self.cap + self.top[sn]] = item
        self.top[sn] += 1

    def pop(self, sn: int) -> int:
        if self.top[sn] <= 0:
            raise IndexError("stack empty")
        self.top[sn] -= 1
        return self.arr[sn * self.cap + self.top[sn]]


if __name__ == "__main__":
    s = ThreeStacks(3)
    s.push(1, 0); s.push(2, 0)
    s.push(10, 1); s.push(20, 1)
    s.push(100, 2)
    print("stack0 pop:", s.pop(0))
    print("stack1 pop:", s.pop(1))
    print("stack2 pop:", s.pop(2))
    print("stack0 pop:", s.pop(0))
