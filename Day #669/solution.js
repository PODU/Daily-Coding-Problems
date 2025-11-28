// Day 669: Misere Nim. First player wins iff either (some heap>1 and xor!=0) or
// (all heaps<=1 and xor==0). Time O(n), Space O(1).
function firstPlayerWins(heaps) {
  let x = 0, anyBig = false;
  for (const h of heaps) { x ^= h; if (h > 1) anyBig = true; }
  return anyBig ? x !== 0 : x === 0;
}

console.log(firstPlayerWins([3, 4, 5]) ? "True" : "False"); // True
