// Day 991: Closure late-binding demonstration.
// Using `var` (function scope) all closures share one binding -> 3,3,3.
// Using `let` (block scope) each iteration gets a fresh binding -> 1,2,3.
// O(n) time/space.

function makeBuggy() {
  const flist = [];
  for (var i of [1, 2, 3]) {        // var: single shared binding
    flist.push(() => i);
  }
  return flist;
}

function makeFixed() {
  const flist = [];
  for (let i of [1, 2, 3]) {        // let: fresh binding per iteration
    flist.push(() => i);
  }
  return flist;
}

console.log("Buggy:", makeBuggy().map((f) => f()).join(" ")); // 3 3 3
console.log("Fixed:", makeFixed().map((f) => f()).join(" ")); // 1 2 3
