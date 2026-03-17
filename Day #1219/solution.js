// Closure late-binding: all closures share one variable and read its final value; fix =
// per-iteration binding with `let`. O(n) build/call, O(n) space.

// Buggy: every closure closes over the SAME `var i`. The last value assigned to i while
// in range is 9, so all closures return 9.
const buggy = [];
for (var i = 0; i < 10; i++) {
  buggy.push(function () { return i; });
  if (i === 9) break; // leave shared i at 9 to mirror Python's loop-var semantics
}
for (const f of buggy) {
  console.log(f());
}

console.log("---");

// Fixed: `let` creates a fresh binding each iteration, so each closure keeps its own value.
const fixed = [];
for (let k = 0; k < 10; k++) {
  fixed.push(() => k);
}
for (const f of fixed) {
  console.log(f());
}
