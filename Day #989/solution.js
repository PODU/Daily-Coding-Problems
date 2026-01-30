// Day 989: Deduce coin denominations from a ways-to-make-change array.
// Walk amounts; whenever target[i] exceeds reconstructed ways, i is a coin and we fold it into the DP.
// O(N^2) time, O(N) space.

function findDenominations(target) {
  const n = target.length;
  const have = new Array(n).fill(0);
  have[0] = 1;                       // one way to make 0 with no coins
  const coins = [];
  for (let i = 1; i < n; i++) {
    if (target[i] > have[i]) {       // unaccounted combinations => i is a denomination
      coins.push(i);
      for (let j = i; j < n; j++) have[j] += have[j - i];
    }
  }
  return coins;
}

const target = [1, 0, 1, 1, 2];
console.log(findDenominations(target).join(", ")); // expected: 2, 3, 4
