// Day 85: Select x if b==1 else y using only bit ops. mask = -b (all 1s or all 0s).
// Time O(1), Space O(1).
function select(x, y, b) {
  const mask = -b; // b=1 -> 0xFFFFFFFF, b=0 -> 0 (32-bit two's complement)
  return (x & mask) | (y & ~mask);
}

console.log(select(5, 10, 1)); // 5
console.log(select(5, 10, 0)); // 10
