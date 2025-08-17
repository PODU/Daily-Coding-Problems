// XOR all -> a^b; isolate a differing bit; partition into two groups and XOR each.
// Time O(n); Space O(1).
function twoSingles(nums) {
  let x = 0;
  for (const v of nums) x ^= v;
  const bit = x & -x; // lowest set bit where the two singles differ
  let a = 0,
    b = 0;
  for (const v of nums) {
    if (v & bit) a ^= v;
    else b ^= v;
  }
  return a < b ? [a, b] : [b, a];
}

const [a, b] = twoSingles([2, 4, 6, 8, 10, 2, 6, 10]);
console.log(`${a} and ${b}`); // 4 and 8
