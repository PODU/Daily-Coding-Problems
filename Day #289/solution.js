// Misere Nim (3 heaps): first player wins iff
// (some heap>1 && xor!=0) || (all heaps<=1 && xor==0). Time: O(1), Space: O(1).
function firstPlayerWins(heaps) {
  const [a, b, c] = heaps;
  const x = a ^ b ^ c;
  const anyBig = a > 1 || b > 1 || c > 1;
  return anyBig ? x !== 0 : x === 0;
}

console.log(firstPlayerWins([3, 4, 5]));
