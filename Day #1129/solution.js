// Simplified Elo rating system. Expected score Ea=1/(1+10^((Rb-Ra)/400)),
// update Ra'=Ra+K*(Sa-Ea), K=32. recordGame transfers points winner<-loser. O(1) per game.
'use strict';

class EloSystem {
    constructor() { this.K = 32.0; this.rating = new Map(); }
    addPlayer(name) { if (!this.rating.has(name)) this.rating.set(name, 1200.0); }
    expected(ra, rb) { return 1.0 / (1.0 + Math.pow(10.0, (rb - ra) / 400.0)); }
    recordGame(winner, loser) {
        this.addPlayer(winner); this.addPlayer(loser);
        const ra = this.rating.get(winner), rb = this.rating.get(loser);
        const ea = this.expected(ra, rb), eb = this.expected(rb, ra);
        this.rating.set(winner, ra + this.K * (1.0 - ea));
        this.rating.set(loser,  rb + this.K * (0.0 - eb));
    }
    get(name) { return Math.round(this.rating.get(name)); }
}

function main() {
    const elo = new EloSystem();
    elo.addPlayer("A"); elo.rating.set("A", 1200.0);
    elo.addPlayer("B"); elo.rating.set("B", 2000.0);
    console.log(`Before: A=${elo.get("A")}, B=${elo.get("B")}`);
    elo.recordGame("A", "B"); // lower-rated A beats higher-rated B
    console.log(`After A(1200) beats B(2000): A=${elo.get("A")}, B=${elo.get("B")}`);
}

main();
