# Day 1813: Last-N order log via fixed-size circular buffer.
# record: O(1), get_last(i): O(1). Space: O(N).


class OrderLog:
    def __init__(self, n):
        self.n = n
        self.buf = [None] * n
        self.head = 0  # index where the next write goes
        self.count = 0

    def record(self, order_id):
        self.buf[self.head] = order_id
        self.head = (self.head + 1) % self.n
        if self.count < self.n:
            self.count += 1

    def get_last(self, i):  # i=1 -> most recent
        return self.buf[(self.head - i) % self.n]


if __name__ == "__main__":
    log = OrderLog(5)
    for oid in [101, 102, 103, 104, 105, 106]:
        log.record(oid)
    print(log.get_last(1))  # 106
    print(log.get_last(3))  # 104
