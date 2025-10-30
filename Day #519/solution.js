// Branchless select: mask = -b (all 1s if b=1, all 0s if b=0); pick x or y. O(1).
function select(x, y, b) {
  const mask = -b; // 32-bit bitwise ops in JS
  return (x & mask) | (y & ~mask);
}

console.log(select(42, 17, 1)); // 42
console.log(select(42, 17, 0)); // 17
