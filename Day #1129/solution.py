# Day 1129: Simplified Elo rating system. Expected score Ea=1/(1+10^((Rb-Ra)/400)),
# update Ra'=Ra+K*(Sa-Ea), K=32. recordGame transfers points winner<-loser. O(1) per game.

class EloSystem:
    K = 32.0
    def __init__(self):
        self.rating = {}
    def add_player(self, name):
        self.rating.setdefault(name, 1200.0)
    @staticmethod
    def expected(ra, rb):
        return 1.0 / (1.0 + 10.0 ** ((rb - ra) / 400.0))
    def record_game(self, winner, loser):
        self.add_player(winner); self.add_player(loser)
        ra, rb = self.rating[winner], self.rating[loser]
        ea, eb = self.expected(ra, rb), self.expected(rb, ra)
        self.rating[winner] = ra + self.K * (1.0 - ea)
        self.rating[loser]  = rb + self.K * (0.0 - eb)
    def get(self, name):
        return int(round(self.rating[name]))

def main():
    elo = EloSystem()
    elo.add_player("A"); elo.rating["A"] = 1200.0
    elo.add_player("B"); elo.rating["B"] = 2000.0
    print(f"Before: A={elo.get('A')}, B={elo.get('B')}")
    elo.record_game("A", "B")  # lower-rated A beats higher-rated B
    print(f"After A(1200) beats B(2000): A={elo.get('A')}, B={elo.get('B')}")

if __name__ == "__main__":
    main()
