// Day 779: Egg drop - minimum worst-case trials for N eggs, k floors.
// Find smallest m with sum_{i=1..N} C(m,i) >= k. O(N * answer) time, O(1) space.
function eggDrop(eggs, floors) {
  if (floors === 0) return 0;
  let m = 0;
  while (true) {
    m++;
    let reach = 0, c = 1;
    for (let i = 1; i <= eggs; i++) {
      c = (c * (m - i + 1)) / i;
      reach += c;
      if (reach >= floors) break;
    }
    if (reach >= floors) return m;
  }
}

console.log(eggDrop(1, 5));   // 5
console.log(eggDrop(2, 100)); // 14
