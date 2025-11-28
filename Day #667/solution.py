# Day 667: Simplified Elo rating. Expected score E = 1/(1+10^((Rb-Ra)/400)),
# update R += K*(actual - expected). Underdog gains more. Each update O(1).


class Elo:
    def __init__(self, K=32, start=1200):
        self.K, self.start, self.r = K, start, {}

    def rating(self, p):
        return self.r.setdefault(p, self.start)

    def game(self, winner, loser):
        ra, rb = self.rating(winner), self.rating(loser)
        ea = 1.0 / (1.0 + 10 ** ((rb - ra) / 400.0))
        eb = 1.0 - ea
        self.r[winner] = ra + self.K * (1 - ea)
        self.r[loser] = rb + self.K * (0 - eb)


if __name__ == "__main__":
    e = Elo()
    e.r["A"], e.r["B"] = 1200, 2000
    e.game("A", "B")  # underdog A beats B
    print("A: %.1f" % e.rating("A"))  # ~1231.5
    print("B: %.1f" % e.rating("B"))  # ~1968.5
