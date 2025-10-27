# Day 504: Last-N order log via fixed-size circular buffer.
# record O(1), get_last(i) O(1); space O(N).


class OrderLog:
    def __init__(self, n):
        self.cap = n
        self.buf = [None] * n
        self.pos = 0    # next write index
        self.count = 0  # records seen (capped at cap)

    def record(self, order_id):
        self.buf[self.pos] = order_id
        self.pos = (self.pos + 1) % self.cap
        if self.count < self.cap:
            self.count += 1

    def get_last(self, i):
        # i = 1 is the most recent.
        idx = (self.pos - i) % self.cap
        return self.buf[idx]


def main():
    log = OrderLog(5)
    for order_id in [1, 2, 3, 4, 5, 6, 7]:
        log.record(order_id)
    print("get_last(1) =", log.get_last(1))
    print("get_last(2) =", log.get_last(2))
    print("get_last(3) =", log.get_last(3))


if __name__ == "__main__":
    main()
