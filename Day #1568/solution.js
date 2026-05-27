// Longest consecutive run of 1s using n &= (n<<1) bit trick. Time O(run length), space O(1).
function longestRun(n) {
  let v = BigInt(n);
  let count = 0;
  while (v !== 0n) {
    v &= (v << 1n);
    count++;
  }
  return count;
}

console.log(longestRun(156)); // 10011100 -> 3
