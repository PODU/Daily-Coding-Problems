// Elo rating: expected = 1/(1+10^((Rb-Ra)/400)); delta = K*(score-expected), zero-sum. O(1) per game.
class EloSystem {
  constructor(k = 32.0) {
    this.ratings = new Map();
    this.K = k;
  }
  add(p, r = 1200.0) { this.ratings.set(p, r); }
  static expected(ra, rb) {
    return 1.0 / (1.0 + Math.pow(10.0, (rb - ra) / 400.0));
  }
  recordGame(w, l) {
    const rw = this.ratings.get(w), rl = this.ratings.get(l);
    const ew = EloSystem.expected(rw, rl);
    const delta = this.K * (1.0 - ew);
    this.ratings.set(w, rw + delta);
    this.ratings.set(l, rl - delta);
    console.log(`${w} beats ${l}: ${w} ${Math.round(rw)}->${Math.round(rw + delta)}, `
      + `${l} ${Math.round(rl)}->${Math.round(rl - delta)}`);
  }
}

const e = new EloSystem();
for (const p of ["A", "B", "C", "D"]) e.add(p);
e.recordGame("A", "B");
e.recordGame("A", "C");
e.recordGame("D", "A");
console.log("Final ratings:");
for (const p of [...e.ratings.keys()].sort())
  console.log(`${p} ${Math.round(e.ratings.get(p))}`);
