// Day 1118 - Late-binding closure pitfall
// Closing over shared mutable state makes every function see its final value (9).
// Fix: give each closure its own binding (let / per-iteration copy).

function buggy() {
  const fns = [];
  const holder = { i: 0 };
  for (let k = 0; k < 10; k++) {
    holder.i = k;
    fns.push(() => holder.i); // shared state
  }
  return fns;
}

function fixed() {
  const fns = [];
  for (let k = 0; k < 10; k++) {
    fns.push(() => k); // `let` gives each iteration its own k
  }
  return fns;
}

console.log("Buggy output (all 9):");
for (const f of buggy()) console.log(f());
console.log("Fixed output (0-9):");
for (const f of fixed()) console.log(f());
