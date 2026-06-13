// Closure capture demo: buggy closures share one function-scoped `var` (final value);
// fix uses per-iteration block scoping (`const` in for-of). O(n) time/space.
console.log("Late binding (buggy):");
const buggy = [];
for (var k = 0; k < 3; k++) {
  var i = [1, 2, 3][k];
  buggy.push(() => i);
}
buggy.forEach(f => console.log(f()));
console.log("Fixed (capture value):");
const fixed = [];
for (const j of [1, 2, 3]) {
  fixed.push(() => j);
}
fixed.forEach(f => console.log(f()));
