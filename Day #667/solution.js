// Day 667: Simplified Elo rating. Expected score E = 1/(1+10^((Rb-Ra)/400)),
// update R += K*(actual - expected). Underdog gains more. Each update O(1).
class Elo {
  constructor(K = 32, start = 1200) { this.K = K; this.start = start; this.r = new Map(); }
  rating(p) { if (!this.r.has(p)) this.r.set(p, this.start); return this.r.get(p); }
  game(winner, loser) {
    const ra = this.rating(winner), rb = this.rating(loser);
    const ea = 1 / (1 + Math.pow(10, (rb - ra) / 400));
    const eb = 1 - ea;
    this.r.set(winner, ra + this.K * (1 - ea));
    this.r.set(loser, rb + this.K * (0 - eb));
  }
}

const e = new Elo();
e.r.set("A", 1200); e.r.set("B", 2000);
e.game("A", "B");
console.log("A:", e.rating("A").toFixed(1)); // ~1231.5
console.log("B:", e.rating("B").toFixed(1)); // ~1968.5
