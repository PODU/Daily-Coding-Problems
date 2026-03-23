// Interval point stabbing: greedy, sort by right endpoint, pick endpoint when uncovered.
// Time O(n log n), Space O(n).
function stab(intervals){
  const iv=[...intervals].sort((a,b)=>a[1]-b[1]);
  const pts=[]; let last=-Infinity;
  for(const [s,e] of iv){ if(s>last){ last=e; pts.push(e); } }
  return pts;
}
console.log("[" + stab([[1,4],[4,5],[7,9],[9,12]]).join(", ") + "]");
