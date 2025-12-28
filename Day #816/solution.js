// Branchless select via bitwise mask: y ^ ((-b) & (x ^ y)). O(1) time, O(1) space.

function select(x, y, b) {
  return y ^ ((-b) & (x ^ y));
}

console.log("b=1 -> " + select(5, 9, 1));
console.log("b=0 -> " + select(5, 9, 0));
