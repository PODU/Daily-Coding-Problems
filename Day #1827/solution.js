// Egg drop: min trials t s.t. floors(t,eggs)=sum_{i=1..eggs} C(t,i) >= k.
// O(eggs * answer). For N=1,k=5 -> 5.
function floorsCovered(t, eggs, cap) {
  let total = 0, c = 1;
  for (let i = 1; i <= eggs; i++) {
    c = (c * (t - i + 1)) / i;   // C(t,i) incrementally
    total += c;
    if (total >= cap) return cap;
  }
  return total;
}

function minDrops(eggs, k) {
  let t = 0;
  while (floorsCovered(t, eggs, k) < k) t++;
  return t;
}

console.log(minDrops(1, 5)); // 5
