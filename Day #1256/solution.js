// Max contiguous subarray sum, empty allowed: Kadane, clamp running sum at 0.
// Time O(n), Space O(1).
function maxSub(a){
  let cur=0,best=0;
  for(const x of a){ cur+=x; if(cur<0)cur=0; best=Math.max(best,cur); }
  return best;
}
console.log(maxSub([34,-50,42,14,-5,86]));
console.log(maxSub([-5,-1,-8,-9]));
