// Day 769: Expected rolls for "5 then 6" vs "5 then 5" stopping games.
// Exact via 2-state Markov chains (E1=36, E2=42) plus Monte-Carlo check.
function simulate(second, trials) {
  let total = 0;
  for (let t = 0; t < trials; t++) {
    let prev = 0, rolls = 0;
    while (true) {
      const r = 1 + Math.floor(Math.random() * 6);
      rolls++;
      if (prev === 5 && r === second) break;
      prev = r;
    }
    total += rolls;
  }
  return total / trials;
}

const trials = 200000;
console.log(`Game 1 (5 then 6): exact=36, simulated=${simulate(6, trials).toFixed(2)}`);
console.log(`Game 2 (5 then 5): exact=42, simulated=${simulate(5, trials).toFixed(2)}`);
console.log("Alice should play Game 1 (5 then 6); it has the lower expected cost.");
