// Simplified Elo rating system. Expected score logistic, K=32 update on win/loss.
// recordGame adjusts both players. Time O(1) per game, Space O(players).

class Elo {
  constructor() {
    this.ratings = {};
    this.K = 32.0;
  }
  add(name, r = 1200.0) {
    this.ratings[name] = r;
  }
  expected(ra, rb) {
    return 1.0 / (1.0 + Math.pow(10.0, (rb - ra) / 400.0));
  }
  recordGame(winner, loser) {
    const ra = this.ratings[winner], rb = this.ratings[loser];
    const ea = this.expected(ra, rb), eb = this.expected(rb, ra);
    this.ratings[winner] = ra + this.K * (1.0 - ea);
    this.ratings[loser] = rb + this.K * (0.0 - eb);
  }
}

function main() {
  const e = new Elo();
  e.add("A"); e.add("B");
  console.log(`Initial: A=${e.ratings.A.toFixed(2)} B=${e.ratings.B.toFixed(2)}`);
  e.recordGame("B", "A");
  console.log(`After B beats A (equal): A=${e.ratings.A.toFixed(2)} B=${e.ratings.B.toFixed(2)}`);

  const e2 = new Elo();
  e2.add("C", 1000.0); e2.add("D", 1600.0);
  console.log(`Initial: C=${e2.ratings.C.toFixed(2)} D=${e2.ratings.D.toFixed(2)}`);
  e2.recordGame("C", "D");
  console.log(`After underdog C beats D: C=${e2.ratings.C.toFixed(2)} D=${e2.ratings.D.toFixed(2)}`);
  console.log(`Underdog gained ${(e2.ratings.C - 1000.0).toFixed(2)} ` +
    `vs even-match gain ${(e.ratings.B - 1200.0).toFixed(2)}`);
}

main();
