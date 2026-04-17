// Max of two without if/comparison: subtract the masked difference using the sign bit.
// max(a,b) = a - ((a-b) & ((a-b)>>31)) on 32-bit ints. Time O(1), Space O(1).
function maxNoBranch(a, b) {
  const diff = a - b;
  return a - (diff & (diff >> 31));
}

console.log(maxNoBranch(3, 7));   // 7
console.log(maxNoBranch(10, -5)); // 10
