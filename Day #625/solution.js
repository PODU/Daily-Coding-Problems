// Longest consecutive run of 1s in binary repr of n.
// Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(bits) time, O(1) space.

function longestRun(n) {
  let count = 0;
  while (n) { n &= (n << 1); count++; }
  return count;
}

console.log(longestRun(156)); // 10011100 -> 3
