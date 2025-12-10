// Day 730: Closure-in-a-loop late binding (JS analogue).
// Using `var` (function scope) -> all closures share one binding (3,3,3 here using a
// shared variable). Fix: use block-scoped `let` so each iteration has its own binding.

// Buggy: shared variable captured
const buggy = [];
let shared = 0;
[1, 2, 3].forEach((v) => { shared = v; buggy.push(() => console.log(shared)); });
buggy.forEach((f) => f());   // 3, 3, 3

// Fixed: per-iteration binding with let
const fixed = [];
for (let i = 1; i <= 3; i++) fixed.push(() => console.log(i));
fixed.forEach((f) => f());   // 1, 2, 3
