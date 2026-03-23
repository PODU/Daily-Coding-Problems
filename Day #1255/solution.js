// Two-sum existence: one-pass hash set, check (k-num) seen before insert.
// Time O(n), Space O(n).
function hasPair(a, k){
  const seen=new Set();
  for(const x of a){ if(seen.has(k-x))return true; seen.add(x); }
  return false;
}
const v=[10,15,3,7];
console.log(hasPair(v,17));
console.log(hasPair(v,50));
