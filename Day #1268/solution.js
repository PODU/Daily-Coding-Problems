// Day 1268: Select x if b==1 else y using only arithmetic/bit ops (no branches).
// y ^ ((x ^ y) & -b): -b is all-ones when b==1, all-zeros when b==0. O(1).
function select(x, y, b) {
  return (y ^ ((x ^ y) & -b)) | 0; // | 0 keeps it a 32-bit int
}

const x = 5, y = 10;
console.log("b=1 ->", select(x, y, 1)); // 5
console.log("b=0 ->", select(x, y, 0)); // 10
