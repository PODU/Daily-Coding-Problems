// Day 1262: Closure-in-a-loop (late binding) demonstrated in JS.
// With `var`, all closures share one binding and print the final value (3).
// Fix: use `let`, which creates a fresh binding per iteration.
const buggy = [];
for (var i of [1, 2, 3]) {
  buggy.push(() => console.log(i)); // shares one function-scoped `var i`
}
const fixed = [];
for (let j of [1, 2, 3]) {
  fixed.push(() => console.log(j)); // fresh block-scoped `let j` each iteration
}
console.log("Buggy output:");
buggy.forEach((f) => f());
console.log("Fixed output:");
fixed.forEach((f) => f());
