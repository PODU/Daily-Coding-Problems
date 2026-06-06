// Sevenish: write n in binary; each set bit k contributes 7^k. Time O(log n), Space O(1).
function sevenish(n) {
  let total = 0, pow7 = 1;
  while (n > 0) {
    if (n & 1) total += pow7;
    pow7 *= 7;
    n >>= 1;
  }
  return total;
}

const out = [];
for (let n = 1; n <= 5; n++) out.push(sevenish(n));
console.log(out.join(" "));
