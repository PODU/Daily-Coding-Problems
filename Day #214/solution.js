// Day 214: Longest consecutive run of 1s in binary representation.
// Approach: n &= (n<<1) collapses runs; count iterations (BigInt-safe). Time O(longest run), Space O(1).
function longestRun(n) {
  let x = BigInt(n);
  let count = 0;
  while (x !== 0n) {
    x &= (x << 1n);
    count++;
  }
  return count;
}

console.log(longestRun(156)); // 156 = 10011100 -> 3
