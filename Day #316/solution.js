// Reconstruct coin denominations from a ways-to-make-change array.
// DP coin detection: A[i] > ways[i] means i is a coin. Time O(N^2), Space O(N).

function joinCoins(c) {
  if (c.length === 0) return "";
  if (c.length === 1) return String(c[0]);
  if (c.length === 2) return `${c[0]} and ${c[1]}`;
  return c.slice(0, -1).join(", ") + `, and ${c[c.length - 1]}`;
}

function findCoins(A) {
  const n = A.length;
  const ways = new Array(n).fill(0);
  ways[0] = 1;
  const coins = [];
  for (let i = 1; i < n; i++) {
    if (A[i] > ways[i]) {
      coins.push(i);
      for (let j = i; j < n; j++) ways[j] += ways[j - i];
    }
  }
  return coins;
}

const A = [1, 0, 1, 1, 2];
console.log(joinCoins(findCoins(A)));
