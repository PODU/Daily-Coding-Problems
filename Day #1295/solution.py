# Day 1295: Fixed-size log of last N order ids via circular buffer.
# record and get_last are both O(1) time, O(N) space.


class OrderLog:
    def __init__(self, n: int):
        self.n = n
        self.buf = [None] * n
        self.head = 0  # index of next write
        self.count = 0

    def record(self, order_id):
        self.buf[self.head] = order_id
        self.head = (self.head + 1) % self.n
        if self.count < self.n:
            self.count += 1

    def get_last(self, i):  # 1 = most recent
        return self.buf[(self.head - i) % self.n]


if __name__ == "__main__":
    log = OrderLog(3)
    log.record(10)
    log.record(20)
    log.record(30)
    print(log.get_last(1))  # 30
    print(log.get_last(2))  # 20
    log.record(40)
    print(log.get_last(1))  # 40
    print(log.get_last(3))  # 20
