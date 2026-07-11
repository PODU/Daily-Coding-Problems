// Misere Nim: P-position (loss for mover) iff (all heaps==1 with even count) or (some heap>1 and xor==0). First wins otherwise.
// Time O(n), Space O(1).
function firstPlayerWins(heaps) {
  const x = heaps.reduce((a, b) => a ^ b, 0);
  const mx = Math.max(...heaps);
  let pPosition;
  if (mx <= 1) pPosition = x === 0;  // all heaps == 1: P iff even count
  else pPosition = x === 0;           // some heap > 1: P iff xor == 0
  return !pPosition;
}

const heaps = [3, 4, 5];
console.log(firstPlayerWins(heaps)); // expected true
