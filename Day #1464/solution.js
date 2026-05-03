// Two single elements (rest in pairs): XOR all -> a^b, split by a set bit, XOR groups.
// Time: O(n), Space: O(1).
function twoUnique(arr) {
  let x = 0;
  for (const v of arr) x ^= v;
  const bit = x & -x; // lowest set bit
  let a = 0, b = 0;
  for (const v of arr) {
    if (v & bit) a ^= v;
    else b ^= v;
  }
  return a < b ? [a, b] : [b, a];
}

const arr = [2, 4, 6, 8, 10, 2, 6, 10];
const [a, b] = twoUnique(arr);
console.log(`${a} and ${b}`);
