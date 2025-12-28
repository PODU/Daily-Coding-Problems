# Day 812: Simplified Elo rating system. Expected score logistic, K=32 update on win/loss.
# record_game adjusts both players. Time O(1) per game, Space O(players).


class Elo:
    K = 32.0

    def __init__(self):
        self.ratings = {}

    def add(self, name, r=1200.0):
        self.ratings[name] = r

    def expected(self, ra, rb):
        return 1.0 / (1.0 + 10.0 ** ((rb - ra) / 400.0))

    def record_game(self, winner, loser):
        ra, rb = self.ratings[winner], self.ratings[loser]
        ea, eb = self.expected(ra, rb), self.expected(rb, ra)
        self.ratings[winner] = ra + self.K * (1.0 - ea)
        self.ratings[loser] = rb + self.K * (0.0 - eb)


def main():
    e = Elo()
    e.add("A"); e.add("B")
    print(f"Initial: A={e.ratings['A']:.2f} B={e.ratings['B']:.2f}")
    e.record_game("B", "A")
    print(f"After B beats A (equal): A={e.ratings['A']:.2f} B={e.ratings['B']:.2f}")

    e2 = Elo()
    e2.add("C", 1000.0); e2.add("D", 1600.0)
    print(f"Initial: C={e2.ratings['C']:.2f} D={e2.ratings['D']:.2f}")
    e2.record_game("C", "D")
    print(f"After underdog C beats D: C={e2.ratings['C']:.2f} D={e2.ratings['D']:.2f}")
    print(f"Underdog gained {e2.ratings['C'] - 1000.0:.2f} "
          f"vs even-match gain {e.ratings['B'] - 1200.0:.2f}")


if __name__ == "__main__":
    main()
