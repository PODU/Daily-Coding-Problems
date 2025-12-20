// Day 770: Misere Nim forced-win check.
// If every heap == 1: first player wins iff count of heaps is even.
// Else: first player wins iff XOR of heaps != 0. O(N).
function firstPlayerWins(heaps) {
  let xorSum = 0, allOne = true;
  for (const h of heaps) { xorSum ^= h; if (h > 1) allOne = false; }
  if (allOne) return heaps.length % 2 === 0;
  return xorSum !== 0;
}

console.log(firstPlayerWins([3, 4, 5])); // true
