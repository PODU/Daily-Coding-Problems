// Min broadcast range: sort towers, binary-search nearest per listener, answer = max of mins.
// Time O((N+M) log M), Space O(1).
function lowerBound(arr, x){
  let lo=0, hi=arr.length;
  while(lo<hi){ const mid=(lo+hi)>>1; if(arr[mid]<x)lo=mid+1; else hi=mid; }
  return lo;
}
function minRange(listeners, towers){
  towers=[...towers].sort((a,b)=>a-b);
  let ans=0;
  for(const l of listeners){
    const i=lowerBound(towers,l);
    let best=Infinity;
    if(i<towers.length)best=Math.min(best,towers[i]-l);
    if(i>0)best=Math.min(best,l-towers[i-1]);
    ans=Math.max(ans,best);
  }
  return ans;
}
console.log(minRange([1,5,11,20],[4,8,15]));
