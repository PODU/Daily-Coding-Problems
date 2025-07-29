// rand7 from rand5: rejection sampling over 5*(rand5-1)+rand5 in 1..25,
// reject >21, map ((v-1)%7)+1. Expected O(1) amortized. rand5 from a seeded LCG.
let state = 1; // deterministic seed

function rand5() {
  state = (state * 75 + 74) % 65537;
  return (state % 5) + 1; // uniform-ish 1..5 for the demo
}

function rand7() {
  while (true) {
    const v = 5 * (rand5() - 1) + rand5(); // 1..25
    if (v <= 21) return ((v - 1) % 7) + 1;
  }
}

const samples = [];
for (let i = 0; i < 20; i++) samples.push(rand7());
console.log("rand7 samples: " + samples.join(" "));

const counts = new Array(8).fill(0);
for (let i = 0; i < 7000; i++) counts[rand7()]++;
const parts = [];
for (let v = 1; v <= 7; v++) parts.push(`${v}:${counts[v]}`);
console.log("counts over 7000 trials: " + parts.join(" "));
