# Day 1707: Elo rating: expected = 1/(1+10^((Rb-Ra)/400)); delta = K*(score-expected), zero-sum. O(1) per game.
class EloSystem:
    def __init__(self, k=32.0):
        self.ratings = {}
        self.K = k

    def add(self, p, r=1200.0):
        self.ratings[p] = r

    @staticmethod
    def expected(ra, rb):
        return 1.0 / (1.0 + 10.0 ** ((rb - ra) / 400.0))

    def record_game(self, w, l):
        rw, rl = self.ratings[w], self.ratings[l]
        ew = self.expected(rw, rl)
        delta = self.K * (1.0 - ew)
        self.ratings[w] = rw + delta
        self.ratings[l] = rl - delta
        print(f"{w} beats {l}: {w} {round(rw)}->{round(rw + delta)}, "
              f"{l} {round(rl)}->{round(rl - delta)}")


if __name__ == "__main__":
    e = EloSystem()
    for p in ("A", "B", "C", "D"):
        e.add(p)
    e.record_game("A", "B")
    e.record_game("A", "C")
    e.record_game("D", "A")
    print("Final ratings:")
    for p in sorted(e.ratings):
        print(f"{p} {round(e.ratings[p])}")
