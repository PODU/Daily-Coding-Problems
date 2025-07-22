# Day 16: Approach: Circular (ring) buffer of size N. record/get_last are O(1); O(N) space.

class OrderLog:
    def __init__(self, n):
        self.buf = [None] * n
        self.n = n
        self.count = 0
        self.head = 0  # next write position

    def record(self, order_id):
        self.buf[self.head] = order_id
        self.head = (self.head + 1) % self.n
        if self.count < self.n:
            self.count += 1

    # i is 1-based: get_last(1) is the most recent
    def get_last(self, i):
        return self.buf[(self.head - i) % self.n]


if __name__ == "__main__":
    log = OrderLog(3)
    for x in [1, 2, 3, 4, 5]:
        log.record(x)
    print(f"get_last(1) = {log.get_last(1)}")
    print(f"get_last(2) = {log.get_last(2)}")
    print(f"get_last(3) = {log.get_last(3)}")
