# Day 964: Order log keeping last N ids with O(1) record/get_last.
# Approach: fixed-size circular buffer. Time O(1) per op, Space O(N).


class OrderLog:
    def __init__(self, n):
        self.buf = [None] * n
        self.n = n
        self.count = 0

    def record(self, order_id):
        self.buf[self.count % self.n] = order_id
        self.count += 1

    def get_last(self, i):  # i=1 is most recent
        return self.buf[(self.count - i) % self.n]


if __name__ == '__main__':
    log = OrderLog(3)
    for x in (10, 20, 30, 40):  # 40 evicts 10
        log.record(x)
    print(log.get_last(1))  # 40
    print(log.get_last(2))  # 30
    print(log.get_last(3))  # 20
