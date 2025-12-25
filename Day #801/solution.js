// Day 801: nth sevenish number = sum of unique powers of 7.
// Bits of n select which powers of 7 to add (base-7 analog of binary).
// Time O(log n), Space O(1).

function sevenish(n) {
  let result = 0, power = 1;
  while (n) {
    if (n & 1) result += power;
    power *= 7;
    n >>= 1;
  }
  return result;
}

const out = [];
for (let i = 1; i <= 5; i++) out.push(sevenish(i));
console.log(out.join(" ")); // 1 7 8 49 50
