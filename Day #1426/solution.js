// Day 1426: Maximum XOR of any two elements in an array.
// Approach: Binary trie of bits; for each number greedily pick opposite bit.
// Time: O(n * B), Space: O(n * B) where B = number of bits.

function findMaxXOR(nums) {
  const root = {};
  let maxXor = 0;
  const BITS = 31;
  for (const num of nums) {
    let node = root;
    for (let b = BITS; b >= 0; b--) {
      const bit = (num >> b) & 1;
      if (!node[bit]) node[bit] = {};
      node = node[bit];
    }
    node = root;
    let cur = 0;
    for (let b = BITS; b >= 0; b--) {
      const bit = (num >> b) & 1;
      const opp = 1 - bit;
      if (node[opp]) {
        cur |= (1 << b);
        node = node[opp];
      } else {
        node = node[bit];
      }
    }
    maxXor = Math.max(maxXor, cur);
  }
  return maxXor;
}

console.log(findMaxXOR([3, 10, 5, 25, 2, 8])); // 28
