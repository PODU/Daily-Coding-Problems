// rand7 from rand5 via rejection sampling: idx=(rand5-1)*5+rand5 in 1..25, reject>21,
// return (idx-1)%7+1. O(1) expected calls. Space O(1).

// Deterministic LCG so the histogram is reproducible.
let seed = 12345;
function next() {
  seed = (seed * 1103515245 + 12345) & 0x7fffffff;
  return seed;
}
function rand5() {
  return (next() % 5) + 1;
}

function rand7() {
  while (true) {
    const idx = (rand5() - 1) * 5 + rand5(); // uniform 1..25
    if (idx <= 21) return ((idx - 1) % 7) + 1;
  }
}

function main() {
  const N = 70000;
  const counts = new Array(8).fill(0);
  for (let i = 0; i < N; i++) counts[rand7()]++;
  for (let v = 1; v <= 7; v++) {
    console.log(`${v}: ${counts[v]} ` + "#".repeat(Math.floor(counts[v] / 250)));
  }
}

main();
